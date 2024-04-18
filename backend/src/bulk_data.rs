use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::HashSet,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
    time::Instant,
};
use tokio::task;
use unicode_normalization::UnicodeNormalization;

const BULK_DATA_URL: &str = "https://api.scryfall.com/bulk-data";
const CARDS_FILE_PATH: &str = "data/cards.bin";

#[derive(Debug, Deserialize)]
struct ScryfallDataList {
    data: Vec<ScryfallBulkData>,
}

#[derive(Debug, Deserialize)]
struct ScryfallBulkData {
    download_uri: String,
    r#type: String,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScryfallImageUris {
    pub large: String,
    pub png: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScryfallCardFace {
    pub image_uris: Option<ScryfallImageUris>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScryfallCard {
    pub id: String,
    pub name: String,
    pub image_uris: Option<ScryfallImageUris>,
    pub card_faces: Option<Vec<ScryfallCardFace>>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub promo: bool,
    pub variation: bool,
    pub layout: String,
    pub frame_effects: Option<Vec<String>>,
    pub released_at: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub images: Vec<ScryfallImageUris>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub released_at: NaiveDate,
    pub preferred: bool,
}

#[derive(Debug, Clone)]
pub struct NormalName {
    pub original: String,
    pub normal: String,
    pub normal_front: Option<String>,
    pub normal_back: Option<String>,
}

impl NormalName {
    pub fn new(original: &str) -> NormalName {
        fn normalize(text: &str) -> String {
            let mut output = String::new();
            let mut last_char_space = false;
            for char in text.nfd() {
                if char.is_ascii_alphanumeric() {
                    output.push(char.to_ascii_lowercase());
                    last_char_space = false;
                } else if char.is_whitespace() {
                    if !last_char_space {
                        output.push(' ');
                        last_char_space = true;
                    }
                }
            }
            output
        }
        let normal = normalize(&original);
        let mut normal_front = None;
        let mut normal_back = None;
        if original.contains(" // ") {
            let mut splits = original.split(" // ");
            normal_front = splits.next().map(normalize);
            normal_back = splits.next().map(normalize);
        }
        let original = original.to_string();
        NormalName {
            original,
            normal,
            normal_front,
            normal_back,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDataFile {
    last_fetched: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    cards: Vec<Card>,
}

pub struct BulkData {
    pub last_fetched: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cards: Vec<Card>,
    pub name_index: Vec<NormalName>,
}

impl BulkData {
    pub fn new() -> Self {
        BulkData {
            last_fetched: DateTime::from_timestamp_nanos(0),
            updated_at: DateTime::from_timestamp_nanos(0),
            cards: Vec::new(),
            name_index: Vec::new(),
        }
    }

    pub async fn fetch(&mut self) -> Result<()> {
        info!("Fetching bulk data index from scryfall");
        let start = Instant::now();
        let data_list: ScryfallDataList = reqwest::get(BULK_DATA_URL).await?.json().await?;
        let Some(default_cards) = data_list.data.iter().find(|x| x.r#type == "default_cards")
        else {
            bail!("unable to find bulk_data object with type default_cards");
        };
        let updated_at = default_cards.updated_at;
        let Some(oracle_cards) = data_list.data.iter().find(|x| x.r#type == "oracle_cards") else {
            bail!("unable to find bulk_data object with type oracle_cards");
        };
        info!("Fetching default card data");
        let default_cards: Vec<ScryfallCard> = reqwest::get(&default_cards.download_uri)
            .await?
            .json()
            .await?;
        info!("Fetching oracle card data");
        let oracle_cards: Vec<ScryfallCard> = reqwest::get(&oracle_cards.download_uri)
            .await?
            .json()
            .await?;
        let oracle_ids = oracle_cards
            .into_iter()
            .map(|x| x.id)
            .collect::<HashSet<String>>();

        let mut cards = Vec::new();
        for scryfall_card in default_cards {
            // Exclude certain cards
            if scryfall_card.promo || scryfall_card.layout == "art_series" {
                continue;
            }
            let mut images = Vec::new();
            // Add card face images
            if let Some(card_faces) = scryfall_card.card_faces {
                for face in card_faces {
                    if let Some(image) = face.image_uris {
                        images.push(image);
                    }
                }
            }
            // Add default card face
            if images.len() == 0 {
                if let Some(image) = scryfall_card.image_uris {
                    images.push(image);
                }
            }
            let preferred = oracle_ids.contains(&scryfall_card.id);
            let card = Card {
                id: scryfall_card.id,
                name: scryfall_card.name,
                images,
                set: scryfall_card.set,
                set_name: scryfall_card.set_name,
                collector_number: scryfall_card.collector_number,
                preferred,
                released_at: scryfall_card.released_at,
            };
            cards.push(card);
        }
        cards.sort_by_cached_key(|card| (card.name.clone(), Reverse(card.released_at)));

        self.cards = cards;
        self.last_fetched = Utc::now();
        self.updated_at = updated_at;
        self.name_index = Self::calculate_name_index(&self.cards);

        let elapsed = start.elapsed();
        info!("Fetched {} cards in {elapsed:?}", self.cards.len());
        Ok(())
    }

    pub async fn load_from_file(&mut self) -> Result<()> {
        info!("Loading card database from {CARDS_FILE_PATH}");
        let data = task::spawn_blocking(move || -> Result<BulkData> {
            let file = File::open(CARDS_FILE_PATH)?;
            let reader = BufReader::new(file);
            let data: BulkDataFile = decode::from_read(reader)?;
            let name_index = Self::calculate_name_index(&data.cards);
            Ok(BulkData {
                cards: data.cards,
                last_fetched: data.last_fetched,
                name_index,
                updated_at: data.updated_at,
            })
        })
        .await??;
        *self = data;
        Ok(())
    }

    pub async fn save_to_file(&self) -> Result<()> {
        info!("Saving card database to {CARDS_FILE_PATH}");
        let data = BulkDataFile {
            last_fetched: self.last_fetched,
            updated_at: self.updated_at,
            cards: self.cards.clone(),
        };
        task::spawn_blocking(move || -> Result<()> {
            fs::create_dir_all(Path::new(CARDS_FILE_PATH).parent().unwrap())?;
            let file = File::create(CARDS_FILE_PATH)?;
            let mut writer = BufWriter::new(file);
            encode::write(&mut writer, &data)?;
            Ok(())
        })
        .await
        .context("error saving to file")?
    }

    fn calculate_name_index(card: &[Card]) -> Vec<NormalName> {
        let unique_names = card
            .iter()
            .map(|x| x.name.to_string())
            .collect::<HashSet<_>>();
        let mut output = unique_names
            .into_iter()
            .map(|x| NormalName::new(&x))
            .collect::<Vec<_>>();
        output.sort_by(|a, b| a.original.cmp(&b.original));
        output
    }
}

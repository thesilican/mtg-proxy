use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::task;

const BULK_DATA_URL: &str = "https://api.scryfall.com/bulk-data";
const CARDS_FILE_PATH: &str = "data/cards.json";

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
    pub set_name: String,
    pub collector_number: String,
    pub promo: bool,
    pub variation: bool,
    pub layout: String,
    pub frame_effects: Option<Vec<String>>,
    pub released_at: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct NormalString {
    pub normal: String,
    pub original: String,
}

impl NormalString {
    pub fn new(original: impl Into<String>) -> NormalString {
        let original = original.into();
        let normal = original
            .chars()
            .filter_map(|x| {
                if x.is_alphanumeric() {
                    Some(x.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect();
        NormalString { normal, original }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDataFile {
    last_fetched: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    cards: Vec<ScryfallCard>,
}

pub struct BulkData {
    pub last_fetched: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cards: Vec<ScryfallCard>,
    pub name_index: Vec<NormalString>,
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

    async fn get_default_cards_object(&self) -> Result<ScryfallBulkData> {
        let response: ScryfallDataList = reqwest::get(BULK_DATA_URL).await?.json().await?;
        for object in response.data {
            if object.r#type == "default_cards" {
                return Ok(object);
            }
        }
        bail!("unable to find bulk_data object with type default_cards");
    }

    pub async fn fetch(&mut self) -> Result<()> {
        let start = Instant::now();
        info!("Fetching bulk data from scryfall");
        let default_cards = self.get_default_cards_object().await?;
        let uri = default_cards.download_uri;
        let cards: Vec<ScryfallCard> = reqwest::get(uri).await?.json().await?;

        self.cards = cards;
        self.last_fetched = Utc::now();
        self.updated_at = default_cards.updated_at;
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
            let data: BulkDataFile = serde_json::from_reader(reader)?;
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
            let file = File::create(CARDS_FILE_PATH)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer(writer, &data)?;
            Ok(())
        })
        .await
        .context("error saving to file")?
    }

    fn calculate_name_index(card: &[ScryfallCard]) -> Vec<NormalString> {
        card.iter()
            .map(|x| x.name.to_string())
            .collect::<HashSet<_>>()
            .iter()
            .map(|name| NormalString::new(name))
            .collect()
    }
}

use crate::database::{Card, Database, Metadata};
use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc, time::Instant};

static BULK_DATA_URL: &str = "https://api.scryfall.com/bulk-data";
static APP_USER_AGENT: &str = "reqwest/0.12.3";

#[derive(Deserialize)]
struct ScryfallBulkDataList {
    data: Vec<ScryfallBulkDataItem>,
}

#[derive(Deserialize, Clone)]
struct ScryfallBulkDataItem {
    download_uri: String,
    r#type: String,
    updated_at: DateTime<Utc>,
}

struct BulkDataIndex {
    updated_at: DateTime<Utc>,
    default_cards_uri: String,
    oracle_cards_uri: String,
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

#[derive(Debug)]
pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("failed to build client")?;
        Ok(Downloader { client })
    }

    async fn fetch_bulk_data_index(&self) -> Result<BulkDataIndex> {
        let data_list: ScryfallBulkDataList =
            self.client.get(BULK_DATA_URL).send().await?.json().await?;
        let Some(default_cards) = data_list.data.iter().find(|x| x.r#type == "default_cards")
        else {
            bail!("unable to find bulk_data object with type default_cards");
        };
        let Some(oracle_cards) = data_list.data.iter().find(|x| x.r#type == "oracle_cards") else {
            bail!("unable to find bulk_data object with type oracle_cards");
        };
        let updated_at = default_cards.updated_at.max(oracle_cards.updated_at);
        Ok(BulkDataIndex {
            default_cards_uri: default_cards.download_uri.to_string(),
            oracle_cards_uri: oracle_cards.download_uri.to_string(),
            updated_at,
        })
    }

    async fn fetch(&self, index: &BulkDataIndex) -> Result<(Vec<ScryfallCard>, HashSet<String>)> {
        info!("Fetching cards from scryfall");
        let start = Instant::now();

        let default_cards: Vec<ScryfallCard> = self
            .client
            .get(&index.default_cards_uri)
            .send()
            .await?
            .json()
            .await?;

        let oracle_cards: Vec<ScryfallCard> = self
            .client
            .get(&index.oracle_cards_uri)
            .send()
            .await?
            .json()
            .await?;

        let oracle_ids = oracle_cards
            .into_iter()
            .map(|card| card.id)
            .collect::<HashSet<_>>();
        info!(
            "Fetched {} cards in {:?}",
            default_cards.len(),
            start.elapsed()
        );

        Ok((default_cards, oracle_ids))
    }

    async fn populate_database(
        &self,
        scryfall_cards: Vec<ScryfallCard>,
        preferred_ids: HashSet<String>,
        database: Arc<Database>,
    ) -> Result<()> {
        let start = Instant::now();
        info!("Populating database");
        for scryfall_card in scryfall_cards {
            let mut front_large = None;
            let mut front_png = None;
            let mut back_large = None;
            let mut back_png = None;
            if let Some(imgs) = &scryfall_card.image_uris {
                front_large = Some(imgs.large.to_string());
                front_png = Some(imgs.png.to_string());
            }
            if let Some(faces) = &scryfall_card.card_faces {
                if let Some(front) = faces.first() {
                    if let Some(imgs) = &front.image_uris {
                        front_large = Some(imgs.large.to_string());
                        front_png = Some(imgs.png.to_string());
                    }
                }
                if let Some(back) = faces.get(1) {
                    if let Some(imgs) = &back.image_uris {
                        back_large = Some(imgs.large.to_string());
                        back_png = Some(imgs.png.to_string());
                    }
                }
            }
            let (front_large, front_png) = match (front_large, front_png) {
                (Some(large), Some(png)) => (large, png),
                _ => {
                    continue;
                }
            };

            let normal_name = Database::normalize_name(&scryfall_card.name);
            let card = Card {
                preferred: preferred_ids.contains(&scryfall_card.id),
                id: scryfall_card.id,
                name: scryfall_card.name,
                image_front_large: front_large,
                image_front_png: front_png,
                image_back_large: back_large,
                image_back_png: back_png,
                set: scryfall_card.set,
                set_name: scryfall_card.set_name,
                collector_number: scryfall_card.collector_number,
                released_at: scryfall_card.released_at,
            };
            database.insert_card(&card).await?;
            database.insert_normal_name(&normal_name).await?;
        }
        info!("Populated database in {:?}", start.elapsed());
        Ok(())
    }

    pub async fn refresh_database(&self, database: Arc<Database>) -> Result<()> {
        let now = Utc::now();
        let metadata = database.get_metadata().await?;
        let index = self.fetch_bulk_data_index().await?;
        let last_updated = metadata.last_updated.unwrap_or(DateTime::UNIX_EPOCH);
        if last_updated < index.updated_at {
            let (cards, preferred_ids) = self.fetch(&index).await?;
            self.populate_database(cards, preferred_ids, database.clone())
                .await?;
        }
        database
            .set_metadata(Metadata {
                last_updated: Some(now),
            })
            .await?;
        Ok(())
    }
}

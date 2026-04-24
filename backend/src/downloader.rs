use crate::{canonicalize_name, database::Card};
use crate::{normalize_name, split_normalize_name};
use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use log::warn;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::collections::HashSet;

static SF_BULK_DATA_URL: &str = "https://api.scryfall.com/bulk-data";
static USER_AGENT: &str = "reqwest/0.12.3";

#[derive(Deserialize)]
struct SfBulkDataList {
    data: Vec<SfBulkDataItem>,
}

#[derive(Deserialize)]
struct SfBulkDataItem {
    download_uri: String,
    r#type: String,
}

#[derive(Deserialize)]
pub struct SfImageUris {
    pub large: String,
    pub png: String,
}

impl SfImageUris {
    fn clean_url(url: &str) -> Result<String> {
        // Strip query param from url
        let mut url = Url::try_from(url).context(format!("failed to parse url: {url}"))?;
        url.set_query(None);
        Ok(url.to_string())
    }

    fn jpg_clean(&self) -> Result<String> {
        Self::clean_url(&self.large)
    }

    fn png_clean(&self) -> Result<String> {
        Self::clean_url(&self.png)
    }
}

#[derive(Deserialize)]
pub struct SfCardFace {
    pub name: String,
    pub flavor_name: Option<String>,
    pub image_uris: Option<SfImageUris>,
}

#[derive(Deserialize)]
pub struct SfCard {
    pub id: String,
    pub name: String,
    pub flavor_name: Option<String>,
    pub image_uris: Option<SfImageUris>,
    pub card_faces: Option<Vec<SfCardFace>>,
    pub card_back_id: Option<String>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub layout: String,
    pub released_at: NaiveDate,
}

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .context("failed to build client")?;
        Ok(Downloader { client })
    }

    pub async fn fetch(&self) -> Result<Vec<Card>> {
        let (sf_cards, preferred) = self.fetch_cards().await?;
        let cards = self.process_cards(sf_cards, preferred)?;
        Ok(cards)
    }

    async fn fetch_cards(&self) -> Result<(Vec<SfCard>, HashSet<String>)> {
        let items = self
            .client
            .get(SF_BULK_DATA_URL)
            .send()
            .await?
            .json::<SfBulkDataList>()
            .await?
            .data;
        let Some(default) = items.iter().find(|x| x.r#type == "default_cards") else {
            bail!("unable to find bulk_data object with type default_cards");
        };
        let Some(oracle) = items.iter().find(|x| x.r#type == "oracle_cards") else {
            bail!("unable to find bulk_data object with type oracle_cards");
        };

        let default = self
            .client
            .get(&default.download_uri)
            .send()
            .await?
            .json::<Vec<SfCard>>()
            .await?;
        let oracle = self
            .client
            .get(&oracle.download_uri)
            .send()
            .await?
            .json::<Vec<SfCard>>()
            .await?;
        // The oracle cards are the preferred printing of each card
        let preferred = oracle
            .into_iter()
            .map(|card| card.id)
            .collect::<HashSet<_>>();

        Ok((default, preferred))
    }

    fn process_cards(
        &self,
        sf_cards: Vec<SfCard>,
        preferred: HashSet<String>,
    ) -> Result<Vec<Card>> {
        let mut cards = Vec::new();
        for sf_card in sf_cards {
            let id = sf_card.id.clone();
            let name = canonicalize_name(&sf_card.name);
            let (normal_name_front, normal_name_back) = split_normalize_name(&name);
            let mut flavor_name = None;
            let mut normal_flavor_name_front = None;
            let mut normal_flavor_name_back = None;
            if let Some(name) = sf_card.flavor_name {
                flavor_name = Some(canonicalize_name(&name));
                let (f, b) = split_normalize_name(&name);
                normal_flavor_name_front = Some(f);
                normal_flavor_name_back = b;
            }
            let mut image_front_jpg = None;
            let mut image_front_png = None;
            let mut image_back_jpg = None;
            let mut image_back_png = None;
            // Get card front image
            if let Some(imgs) = &sf_card.image_uris {
                image_front_jpg = Some(imgs.jpg_clean()?);
                image_front_png = Some(imgs.png_clean()?);
            }
            // Handle card faces
            if let Some(faces) = &sf_card.card_faces {
                let mut flavor_name_front = None;
                let mut flavor_name_back = None;
                if let Some(front) = faces.get(0) {
                    if let Some(imgs) = &front.image_uris {
                        image_front_jpg = Some(imgs.jpg_clean()?);
                        image_front_png = Some(imgs.png_clean()?);
                    }
                    if let Some(flavor_name) = &front.flavor_name {
                        flavor_name_front = Some(canonicalize_name(&flavor_name));
                        normal_flavor_name_front = Some(normalize_name(&flavor_name));
                    }
                }
                if let Some(back) = faces.get(1) {
                    if let Some(imgs) = &back.image_uris {
                        image_back_jpg = Some(imgs.jpg_clean()?);
                        image_back_png = Some(imgs.png_clean()?);
                    }
                    if let Some(flavor_name) = &back.flavor_name {
                        flavor_name_back = Some(canonicalize_name(&flavor_name));
                        normal_flavor_name_back = Some(normalize_name(&flavor_name));
                    }
                }
                if let Some(front) = flavor_name_front {
                    let mut name = front;
                    if let Some(back) = flavor_name_back {
                        name.push_str(" // ");
                        name.push_str(&back);
                    }
                    flavor_name = Some(name);
                }
            }
            // Special case to get meld back face
            if let ("meld", Some(id)) = (sf_card.layout.as_ref(), &sf_card.card_back_id) {
                let a = id.get(0..1).context("unexpected card_back_id len")?;
                let b = id.get(1..2).context("unexpected card_back_id len")?;
                image_back_jpg = Some(format!("https://backs.scryfall.io/large/{a}/{b}/{id}.jpg"));
                image_back_png = Some(format!("https://backs.scryfall.io/png/{a}/{b}/{id}.png"));
            }

            let (Some(image_front_jpg), Some(image_front_png)) = (image_front_jpg, image_front_png)
            else {
                warn!("Card {id} ({name}) is missing front image, skipping");
                continue;
            };

            cards.push(Card {
                id,
                name,
                flavor_name,
                normal_name_front,
                normal_name_back,
                normal_flavor_name_front,
                normal_flavor_name_back,
                image_front_jpg,
                image_front_png,
                image_back_jpg,
                image_back_png,
                set: sf_card.set,
                set_name: sf_card.set_name,
                collector_number: sf_card.collector_number,
                released_at: sf_card.released_at,
                preferred: preferred.contains(&sf_card.id),
            })
        }
        Ok(cards)
    }
}

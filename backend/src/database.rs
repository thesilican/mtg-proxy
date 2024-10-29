use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use log::{info, warn};
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, prelude::FromRow, Sqlite, SqlitePool};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

pub struct Metadata {
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, FromRow)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub image_front_large: String,
    pub image_front_png: String,
    pub image_back_large: Option<String>,
    pub image_back_png: Option<String>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub released_at: NaiveDate,
    pub preferred: bool,
}

#[derive(FromRow)]
pub struct NormalName {
    pub name: String,
    pub normal_front: String,
    pub normal_back: Option<String>,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn open(file: &str) -> Result<Self> {
        if !Sqlite::database_exists(file).await.unwrap_or(false) {
            info!("Database {file} doesn't exist, creating");
            let file_path = PathBuf::from(file);
            if let Some(path) = file_path.parent() {
                tokio::fs::create_dir_all(path)
                    .await
                    .context("could not create parent dir for database")?;
            }
            Sqlite::create_database(file)
                .await
                .context("couldn't open database")?;
        }
        let pool = SqlitePool::connect(file)
            .await
            .context("error connecting to sqlite database")?;
        Ok(Database { pool })
    }

    pub async fn init(&self) -> Result<()> {
        static INITIALIZATION_QUERIES: &[&str] = &[
            "CREATE TABLE IF NOT EXISTS cards (
                id TEXT PRIMARY KEY,
                name TEXT,
                image_front_large TEXT,
                image_front_png TEXT,
                image_back_large TEXT,
                image_back_png TEXT,
                \"set\" TEXT,
                set_name TEXT,
                collector_number TEXT,
                released_at TEXT,
                preferred INTEGER
            )",
            "CREATE INDEX IF NOT EXISTS cards_name_index
                ON cards (name)",
            "CREATE TABLE IF NOT EXISTS normal_names (
                name TEXT PRIMARY KEY,
                normal_front TEXT,
                normal_back TEXT
            )",
            "CREATE INDEX IF NOT EXISTS normal_names_front_index
                ON normal_names (normal_front)",
            "CREATE INDEX IF NOT EXISTS normal_names_back_index
                ON normal_names (normal_back)",
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
        ];
        for query in INITIALIZATION_QUERIES {
            sqlx::query(query).execute(&self.pool).await?;
        }
        Ok(())
    }

    pub async fn get_metadata(&self) -> Result<Metadata> {
        let data: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM metadata")
            .fetch_all(&self.pool)
            .await?;
        let mut metadata = Metadata { last_updated: None };
        for (key, val) in data {
            match key.as_ref() {
                "last_updated" => {
                    let last_updated = DateTime::parse_from_rfc3339(&val)
                        .context("failed to parse last_updated")?
                        .to_utc();
                    metadata.last_updated = Some(last_updated);
                }
                _ => {
                    warn!("unknown database metadata key: {key}");
                }
            }
        }
        Ok(metadata)
    }

    pub async fn set_metadata(&self, metadata: Metadata) -> Result<()> {
        async fn insert(pool: &SqlitePool, key: &str, val: Option<String>) -> Result<()> {
            sqlx::query(
                "INSERT INTO metadata (key, value)
                    VALUES ($1, $2)
                    ON CONFLICT (key) DO UPDATE SET value = $2",
            )
            .bind(key)
            .bind(val)
            .execute(pool)
            .await?;
            Ok(())
        }

        insert(
            &self.pool,
            "last_updated",
            metadata.last_updated.map(|date| date.to_rfc3339()),
        )
        .await?;
        Ok(())
    }

    pub async fn get_cards_by_name(&self, name: &str) -> Result<Vec<Card>> {
        let cards: Vec<Card> = sqlx::query_as(
            "SELECT * FROM cards
                WHERE name = $1",
        )
        .bind(name)
        .fetch_all(&self.pool)
        .await?;
        Ok(cards)
    }

    pub async fn insert_card(&self, card: &Card) -> Result<()> {
        sqlx::query(
            "INSERT INTO cards
                    (id, name, image_front_large, image_front_png,
                        image_back_large, image_back_png, \"set\", set_name,
                        collector_number, released_at, preferred)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT DO NOTHING",
        )
        .bind(&card.id)
        .bind(&card.name)
        .bind(&card.image_front_large)
        .bind(&card.image_front_png)
        .bind(&card.image_back_large)
        .bind(&card.image_back_png)
        .bind(&card.set)
        .bind(&card.set_name)
        .bind(&card.collector_number)
        .bind(card.released_at.to_string())
        .bind(card.preferred)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_normal_name(&self, search: &NormalName) -> Result<Vec<NormalName>> {
        let mut results = Vec::<NormalName>::new();
        let front: Vec<NormalName> = sqlx::query_as(
            "SELECT * FROM normal_names
                WHERE normal_front LIKE $1 || '%'
                    OR normal_back LIKE $1 || '%'",
        )
        .bind(&search.normal_front)
        .fetch_all(&self.pool)
        .await?;
        results.extend(front);
        if let Some(search_back) = &search.normal_back {
            let back = sqlx::query_as(
                "SELECT * FROM normal_names
                    WHERE normal_back LIKE $1 || '%'",
            )
            .bind(search_back)
            .fetch_all(&self.pool)
            .await?;
            results.extend(back);
        }
        Ok(results)
    }

    pub async fn insert_normal_name(&self, normal_name: &NormalName) -> Result<()> {
        sqlx::query(
            "INSERT INTO normal_names
                    (name, normal_front, normal_back)
                VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING",
        )
        .bind(&normal_name.name)
        .bind(&normal_name.normal_front)
        .bind(&normal_name.normal_back)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub fn normalize_string(text: &str) -> String {
        let mut normalized = String::new();
        let mut last_char_space = false;
        for char in text.chars() {
            if char.is_ascii_alphanumeric() {
                normalized.push(char.to_ascii_lowercase());
                last_char_space = false;
            } else if char.is_whitespace() && !last_char_space {
                normalized.push('-');
                last_char_space = true;
            }
        }
        normalized
    }

    pub fn normalize_name(name: &str) -> NormalName {
        let name = name.trim().nfd().to_string();
        let mut splits = name.split(" // ");
        let normal_front = Database::normalize_string(splits.next().unwrap());
        let normal_back = splits.next().map(Database::normalize_string);
        NormalName {
            name,
            normal_front,
            normal_back,
        }
    }
}

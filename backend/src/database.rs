use crate::{canonicalize_name, split_normalize_name};
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use log::info;
use serde::Serialize;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, prelude::FromRow};
use std::path::PathBuf;

#[derive(Serialize, FromRow, Clone)]
pub struct Metadata {
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, FromRow, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub flavor_name: Option<String>,
    pub normal_name_front: String,
    pub normal_name_back: Option<String>,
    pub normal_flavor_name_front: Option<String>,
    pub normal_flavor_name_back: Option<String>,
    pub image_front_jpg: String,
    pub image_front_png: String,
    pub image_back_jpg: Option<String>,
    pub image_back_png: Option<String>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub released_at: NaiveDate,
    pub preferred: bool,
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
                name TEXT NOT NULL,
                flavor_name TEXT,
                normal_name_front TEXT NOT NULL,
                normal_name_back TEXT,
                normal_flavor_name_front TEXT,
                normal_flavor_name_back TEXT,
                image_front_jpg TEXT NOT NULL,
                image_front_png TEXT NOT NULL,
                image_back_jpg TEXT,
                image_back_png TEXT,
                \"set\" TEXT,
                set_name TEXT,
                collector_number TEXT,
                released_at TEXT,
                preferred INTEGER
            )",
            "CREATE INDEX IF NOT EXISTS cards_name_idx
                ON cards (name)",
            "CREATE TABLE IF NOT EXISTS metadata (
                id INTEGER PRIMARY KEY,
                last_updated TEXT
            )",
        ];
        let mut tx = self.pool.begin().await?;
        for query in INITIALIZATION_QUERIES {
            sqlx::query(query).execute(&mut *tx).await?;
        }
        tx.commit().await.context("failed to initialize db")
    }

    pub async fn get_card(&self, id: &str) -> Result<Option<Card>> {
        sqlx::query_as(
            "SELECT * FROM cards
                WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("failed to get card")
    }

    pub async fn get_cards_by_name(&self, name: &str) -> Result<Vec<Card>> {
        sqlx::query_as(
            "SELECT * FROM cards
                WHERE name = $1 OR flavor_name = $1",
        )
        .bind(canonicalize_name(name))
        .fetch_all(&self.pool)
        .await
        .context("failed to get cards by name")
    }

    pub async fn get_cards_by_name_front(&self, name: &str) -> Result<Vec<Card>> {
        sqlx::query_as(
            "SELECT * FROM cards
                WHERE name LIKE $1 OR flavor_name LIKE $1",
        )
        .bind(format!("{} // %", canonicalize_name(name)))
        .fetch_all(&self.pool)
        .await
        .context("failed to get cards by front name")
    }

    pub async fn get_cards_by_search(&self, name: &str) -> Result<Vec<Card>> {
        let (front, back) = split_normalize_name(name);
        sqlx::query_as(
            "SELECT * FROM cards
            WHERE 
                ($2 IS NULL AND
                    (instr(normal_name_front, $1)
                        OR instr(normal_name_back, $1)
                        OR instr(normal_flavor_name_front, $1)
                        OR instr(normal_flavor_name_back, $1)))
                OR
                ((normal_name_front == $1 AND normal_name_back == $2)
                    OR (normal_flavor_name_front == $1 AND normal_flavor_name_back == $2))
            ORDER BY released_at DESC",
        )
        .bind(front)
        .bind(back)
        .fetch_all(&self.pool)
        .await
        .context("failed to get cards by search")
    }

    pub async fn insert_cards(&self, cards: &[Card]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for card in cards {
            sqlx::query(
                "INSERT INTO cards
                    (id, name, flavor_name, normal_name_front, normal_name_back,
                        normal_flavor_name_front, normal_flavor_name_back, image_front_jpg,
                        image_front_png, image_back_jpg, image_back_png, \"set\", set_name,
                        collector_number, released_at, preferred)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                ON CONFLICT DO UPDATE SET
                    (id, name, flavor_name, normal_name_front, normal_name_back,
                            normal_flavor_name_front, normal_flavor_name_back, image_front_jpg,
                            image_front_png, image_back_jpg, image_back_png, \"set\", set_name,
                            collector_number, released_at, preferred)
                    = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
            )
            .bind(&card.id)
            .bind(&card.name)
            .bind(&card.flavor_name)
            .bind(&card.normal_name_front)
            .bind(&card.normal_name_back)
            .bind(&card.normal_flavor_name_front)
            .bind(&card.normal_flavor_name_back)
            .bind(&card.image_front_jpg)
            .bind(&card.image_front_png)
            .bind(&card.image_back_jpg)
            .bind(&card.image_back_png)
            .bind(&card.set)
            .bind(&card.set_name)
            .bind(&card.collector_number)
            .bind(card.released_at.to_string())
            .bind(card.preferred)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await.context("transaction failed")
    }

    pub async fn get_metadata(&self) -> Result<Option<Metadata>> {
        sqlx::query_as::<_, Metadata>(
            "SELECT * FROM metadata
                WHERE id = 0",
        )
        .fetch_optional(&self.pool)
        .await
        .context("failed to get metadata")
    }

    pub async fn set_metadata(&self, metadata: Metadata) -> Result<()> {
        sqlx::query(
            "INSERT INTO metadata (id, last_updated)
                VALUES (0, $1)
                ON CONFLICT (id) DO UPDATE
                SET last_updated = $1",
        )
        .bind(metadata.last_updated)
        .execute(&self.pool)
        .await
        .context("failed to set metadata")?;
        Ok(())
    }
}

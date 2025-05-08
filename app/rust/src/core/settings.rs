use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::frb;
use libsql::{de, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub openai_api_key: String,
}

#[derive(Clone)]
pub struct SettingsRepository {
    db: Arc<Connection>,
}

impl SettingsRepository {
    #[frb(sync)]
    pub fn new(db: &Arc<Connection>) -> Self {
        Self { db: Arc::clone(db) }
    }

    pub async fn init(&self) -> Result<()> {
        let mut stmt = self
            .db
            .prepare("CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY, openai_api_key TEXT NOT NULL)")
            .await?;

        stmt.execute(()).await?;

        Ok(())
    }

    #[frb(sync)]
    pub fn get_arc(&self) -> Arc<SettingsRepository> {
        Arc::new(self.clone())
    }
}

impl SettingsRepository {
    pub async fn load(&self) -> Result<Option<Settings>> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM settings WHERE id = 1")
            .await?;

        let row = stmt.query(()).await?.next().await?;

        if let Some(row) = row {
            let settings = de::from_row::<Settings>(&row)?;

            return Ok(Some(settings));
        }

        Ok(None)
    }

    pub async fn save(&self, settings: &Settings) -> Result<()> {
        let mut stmt = self
            .db
            .prepare(
                "INSERT OR REPLACE INTO settings (id, openai_api_key) VALUES (1, :openai_api_key)",
            )
            .await?;

        stmt.query(libsql::named_params! {":openai_api_key": settings.openai_api_key.clone()})
            .await?;

        Ok(())
    }
}

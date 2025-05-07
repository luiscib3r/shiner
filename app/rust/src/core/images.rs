use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::frb;
use libsql::Connection;
use serde::{Deserialize, Serialize};

use super::{job::JobStatus, pagination::Pagination};

#[derive(Serialize, Deserialize)]
pub struct ImageJob {
    pub id: i64,
    pub prompt: String,
    pub status: JobStatus,
    pub image_path: Option<String>,
}

pub struct ImageJobRepository {
    db: Arc<Connection>,
}

impl ImageJobRepository {
    #[frb(sync)]
    pub fn new(db: &Arc<Connection>) -> Self {
        Self { db: Arc::clone(db) }
    }

    pub async fn init(&self) -> Result<()> {
        let mut stmt = self
            .db
            .prepare(
                "CREATE TABLE IF NOT EXISTS image_jobs (
                       id INTEGER PRIMARY KEY,
                       prompt TEXT NOT NULL,
                       status TEXT NOT NULL DEFAULT 'waiting',
                       image_path TEXT
                   )",
            )
            .await?;

        stmt.execute(()).await?;

        Ok(())
    }
}

impl ImageJobRepository {
    pub async fn load(&self, pagination: Pagination) -> Result<Vec<ImageJob>> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM image_jobs ORDER BY id DESC LIMIT :limit OFFSET :offset")
            .await?;

        let mut rows = stmt
            .query(libsql::named_params! {":limit": pagination.limit, ":offset": pagination.offset})
            .await?;

        let mut jobs = Vec::new();

        while let Some(row) = rows.next().await? {
            let job = libsql::de::from_row::<ImageJob>(&row)?;
            jobs.push(job);
        }

        Ok(jobs)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<ImageJob>> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM image_jobs WHERE id = :id")
            .await?;

        let mut rows = stmt.query(libsql::named_params! {":id": id}).await?;

        if let Some(row) = rows.next().await? {
            let job = libsql::de::from_row::<ImageJob>(&row)?;
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_status(
        &self,
        status: JobStatus,
        pagination: Pagination,
    ) -> Result<Vec<ImageJob>> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM image_jobs WHERE status = :status ORDER BY id DESC LIMIT :limit OFFSET :offset")
            .await?;

        let mut rows = stmt
            .query(libsql::named_params! {
                ":status": status,
                ":limit": pagination.limit,
                ":offset": pagination.offset
            })
            .await?;

        let mut jobs = Vec::new();

        while let Some(row) = rows.next().await? {
            let job = libsql::de::from_row::<ImageJob>(&row)?;
            jobs.push(job);
        }

        Ok(jobs)
    }

    pub async fn create_job(&self, prompt: String) -> Result<i64> {
        let mut stmt = self
            .db
            .prepare("INSERT INTO image_jobs (prompt) VALUES (:prompt)")
            .await?;

        stmt.execute(libsql::named_params! {":prompt": prompt})
            .await?;

        Ok(self.db.last_insert_rowid())
    }

    pub async fn update_status(&self, id: i64, status: JobStatus) -> Result<()> {
        let mut stmt = self
            .db
            .prepare("UPDATE image_jobs SET status = :status WHERE id = :id")
            .await?;

        stmt.execute(libsql::named_params! {":status": status, ":id": id})
            .await?;

        Ok(())
    }

    pub async fn set_image_path(&self, id: i64, image_path: String) -> Result<()> {
        let mut stmt = self
            .db
            .prepare(
                "UPDATE image_jobs SET status = :status, image_path = :image_path WHERE id = :id",
            )
            .await?;

        stmt.execute(libsql::named_params! {":id": id, ":image_path": image_path, ":status": JobStatus::Done})
            .await?;

        Ok(())
    }

    pub async fn remove_job(&self, id: i64) -> Result<()> {
        let mut stmt = self
            .db
            .prepare("DELETE FROM image_jobs WHERE id = :id")
            .await?;

        stmt.execute(libsql::named_params! {":id": id}).await?;

        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        let mut stmt = self
            .db
            .prepare("DELETE FROM image_jobs WHERE status = :status")
            .await?;

        stmt.execute(libsql::named_params! {":status": JobStatus::Failed})
            .await?;

        Ok(())
    }

    pub async fn drop_all(&self) -> Result<()> {
        let mut stmt = self.db.prepare("DROP TABLE IF EXISTS image_jobs").await?;

        stmt.execute(()).await?;

        Ok(())
    }
}

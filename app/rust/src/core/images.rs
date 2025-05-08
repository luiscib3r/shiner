use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::{frb, spawn};
use libsql::{Connection, Rows};
use serde::{Deserialize, Serialize};

use crate::{ai::ImageGenerator, frb_generated::StreamSink};

use super::{job::JobStatus, pagination::Pagination, settings::SettingsRepository};

#[derive(Serialize, Deserialize)]
pub struct ImageJob {
    pub id: i64,
    pub prompt: String,
    pub status: JobStatus,
    pub image_path: Option<String>,
    pub err_msg: Option<String>,
}

#[derive(Clone)]
pub struct ImageJobRepository {
    app_directory: Arc<String>,
    db: Arc<Connection>,
    #[allow(dead_code)]
    settings_repository: Arc<SettingsRepository>,
    stream: Option<Arc<StreamSink<i64>>>,
}

//==========================
// Initializer
//==========================
impl ImageJobRepository {
    #[frb(sync)]
    pub fn new(
        app_directory: String,
        db: &Arc<Connection>,
        settings_repository: &Arc<SettingsRepository>,
    ) -> Self {
        Self {
            app_directory: Arc::new(app_directory),
            db: Arc::clone(db),
            settings_repository: Arc::clone(settings_repository),
            stream: None,
        }
    }

    pub async fn init(&self) -> Result<()> {
        // Initialize the database
        let mut stmt = self
            .db
            .prepare(
                "CREATE TABLE IF NOT EXISTS image_jobs (
                       id INTEGER PRIMARY KEY,
                       prompt TEXT NOT NULL,
                       status TEXT NOT NULL DEFAULT 'waiting',
                       image_path TEXT
                       err_msg TEXT
                   )",
            )
            .await?;

        stmt.execute(()).await?;

        // Create images directory if it doesn't exist
        let images_dir = format!("{}/images", self.app_directory);
        if !std::path::Path::new(&images_dir).exists() {
            std::fs::create_dir_all(&images_dir)?;
        }

        Ok(())
    }

    pub fn listen(&mut self, stream: StreamSink<i64>) -> Result<()> {
        self.stream = Some(Arc::new(stream));
        Ok(())
    }
}

//==========================
// Database operations
//==========================
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

        let id = self.db.last_insert_rowid();

        spawn({
            let self_clone = self.clone();
            async move {
                let result = self_clone.start_image_generation(id.clone()).await;
                if let Err(e) = result {
                    eprintln!("Error generating image for job {}: {}", id, e);
                }
            }
        });

        Ok(id)
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

    pub async fn set_error_msg(&self, id: i64, error_msg: String) -> Result<()> {
        let mut stmt = self
            .db
            .prepare("UPDATE image_jobs SET status = :status, err_msg = :err_msg WHERE id = :id")
            .await?;

        stmt.execute(
            libsql::named_params! {":id": id, ":err_msg": error_msg, ":status": JobStatus::Failed},
        )
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

        let rows = stmt.query(libsql::named_params! {":id": id}).await?;
        self.remove_images(rows).await?;

        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        // First get all rows to access their image paths
        let mut select_stmt = self
            .db
            .prepare("SELECT * FROM image_jobs WHERE status = :status")
            .await?;

        let rows = select_stmt
            .query(libsql::named_params! {":status": JobStatus::Failed})
            .await?;

        // Delete the physical files
        self.remove_images(rows).await?;

        // Then delete all records from the database
        let mut delete_stmt = self
            .db
            .prepare("DELETE FROM image_jobs WHERE status = :status")
            .await?;

        delete_stmt
            .execute(libsql::named_params! {":status": JobStatus::Failed})
            .await?;

        Ok(())
    }

    pub async fn drop_all(&self) -> Result<()> {
        // First get all rows to access their image paths
        let mut select_stmt = self.db.prepare("SELECT * FROM image_jobs").await?;
        let rows = select_stmt.query(()).await?;

        // Delete the physical files
        self.remove_images(rows).await?;

        // Then delete all records from the database
        let mut delete_stmt = self.db.prepare("DELETE FROM image_jobs").await?;
        delete_stmt.execute(()).await?;

        Ok(())
    }
}

//==========================
// File management
//==========================
impl ImageJobRepository {
    #[allow(dead_code)]
    async fn save_image(&self, image: Vec<u8>) -> Result<String> {
        let relative_path = format!("images/image_{}.png", uuid::Uuid::new_v4());
        let image_path = format!("{}/{}", self.app_directory, relative_path);

        std::fs::write(&image_path, image)?;

        Ok(relative_path)
    }

    async fn remove_images(&self, mut rows: Rows) -> Result<()> {
        while let Some(row) = rows.next().await? {
            let job = libsql::de::from_row::<ImageJob>(&row)?;
            if let Some(image_path) = job.image_path {
                std::fs::remove_file(image_path).ok();
            }
        }
        Ok(())
    }
}

//==========================
// Image generation
//==========================
impl ImageJobRepository {
    async fn start_image_generation(&self, id: i64) -> Result<()> {
        // Use a simple try/catch pattern with Result
        if let Err(error) = self.try_generate_image(id).await {
            let err_msg = format!("Image generation failed for job {}: {}", id, error);
            self.set_error_msg(id, err_msg).await?;
        }
        Ok(())
    }

    async fn try_generate_image(&self, id: i64) -> Result<()> {
        // Find the job and extract the prompt
        let image_job = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Image job not found"))?;

        // Get settings and API key
        let settings = self
            .settings_repository
            .load()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Settings not found"))?;
        let api_key = settings.openai_api_key.clone();

        // Generate, save, and update the image
        let image_generator = ImageGenerator::new(api_key);
        self.update_status(id, JobStatus::Running).await?;
        let image_bytes = image_generator.run(image_job.prompt).await?;
        let image_path = self.save_image(image_bytes).await?;
        self.set_image_path(id, image_path).await?;

        // Notify any listeners
        if let Some(stream) = &self.stream {
            let result = stream.add(id);
            if let Err(e) = result {
                anyhow::bail!("Failed to send to stream: {}", e);
            }
        }

        Ok(())
    }
}

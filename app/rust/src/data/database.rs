use std::sync::Arc;

use anyhow::Result;
pub use libsql::{Builder, Connection};

pub struct Database {}

impl Database {
    pub async fn connection(app_directory: &str) -> Result<Arc<Connection>> {
        let db_path = format!("{}/app.db", app_directory);
        let db = Builder::new_local(&db_path).build().await?;
        let connection = db.connect()?;
        Ok(Arc::new(connection))
    }
}

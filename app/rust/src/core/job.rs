use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Waiting,
    Running,
    Done,
    Failed,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Waiting => write!(f, "waiting"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Done => write!(f, "done"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "waiting" => Ok(JobStatus::Waiting),
            "running" => Ok(JobStatus::Running),
            "done" => Ok(JobStatus::Done),
            "failed" => Ok(JobStatus::Failed),
            _ => Err(format!("Unknown job status: {}", s)),
        }
    }
}

impl From<JobStatus> for libsql::Value {
    fn from(status: JobStatus) -> Self {
        libsql::Value::Text(status.to_string())
    }
}

impl TryFrom<libsql::Value> for JobStatus {
    type Error = String;

    fn try_from(value: libsql::Value) -> Result<Self, Self::Error> {
        match value {
            libsql::Value::Text(s) => s.parse(),
            _ => Err("Expected text value for JobStatus".to_string()),
        }
    }
}

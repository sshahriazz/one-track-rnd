use once_cell::sync::OnceCell;
use ot_server::project_dto::ProjectDto;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use thiserror::Error;

const BASE_URL: &str = "http://localhost:4000/api/project/";
const TIMEOUT_SECONDS: u64 = 30;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct ProjectService {
    client: OnceCell<Client>,
}

impl ProjectService {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .build()
            .expect("Failed to create HTTP client");
        let cell = OnceCell::new();
        cell.set(client).expect("Failed to set client");
        Self { client: cell }
    }

    pub async fn get_projects(&self) -> Result<Vec<ProjectDto>, ProjectError> {
        let client = self.client.get().expect("Client should be initialized");
        let response = client.get(BASE_URL.to_owned() + "all").send().await?;

        response
            .json()
            .await
            .map_err(|e| ProjectError::NetworkError(e))
    }
}

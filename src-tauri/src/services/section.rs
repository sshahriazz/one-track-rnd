use once_cell::sync::OnceCell;
use ot_server::section_dto::SectionDto;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:4000/api";
const SECTION_ENDPOINT: &str = "http://localhost:4000/api/section/by-project-id/";
const TIMEOUT_SECONDS: u64 = 30;

#[derive(Error, Debug)]
pub enum SectionError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct SectionService {
    client: OnceCell<Client>,
}

impl SectionService {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .build()
            .expect("Failed to create HTTP client");
        let cell = OnceCell::new();
        cell.set(client).expect("Failed to set client");
        Self { client: cell }
    }

    pub async fn get_sections_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<SectionDto>, SectionError> {
        let client = self.client.get().expect("Client should be initialized");
        let response = client
            .get(SECTION_ENDPOINT.to_owned() + project_id.to_string().as_str())
            .send()
            .await?;

        response
            .json()
            .await
            .map_err(|e| SectionError::NetworkError(e))
    }
}

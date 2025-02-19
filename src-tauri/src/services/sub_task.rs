use once_cell::sync::OnceCell;
use ot_server::sub_task_dto::SubTaskDto;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:4000/api/sub-task/";
const TIMEOUT_SECONDS: u64 = 30;

#[derive(Error, Debug)]
pub enum SubTaskError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct SubTaskService {
    client: OnceCell<Client>,
}

impl SubTaskService {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .build()
            .expect("Failed to create HTTP client");
        let cell = OnceCell::new();
        cell.set(client).expect("Failed to set client");
        Self { client: cell }
    }

    pub async fn get_sub_tasks_by_section_id(
        &self,
        section_id: Uuid,
    ) -> Result<Vec<SubTaskDto>, SubTaskError> {
        let client = self.client.get().expect("Client should be initialized");
        let response = client
            .get(BASE_URL.to_owned() + "by-section-id/" + section_id.to_string().as_str())
            .send()
            .await?;

        response
            .json()
            .await
            .map_err(|e| SubTaskError::NetworkError(e))
    }

    pub async fn get_sub_tasks_by_task_id(
        &self,
        task_id: Uuid,
    ) -> Result<Vec<SubTaskDto>, SubTaskError> {
        let client = self.client.get().expect("Client should be initialized");
        let response = client
            .get(BASE_URL.to_owned() + "by-task-id/" + task_id.to_string().as_str())
            .send()
            .await?;

        response
            .json()
            .await
            .map_err(|e| SubTaskError::NetworkError(e))
    }
}

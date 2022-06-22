use anyhow::Result;
use chrono::NaiveDate;

use openapi;
use openapi::apis::configuration::{ApiKey, Configuration};

pub struct Murdock {
    configuration: Configuration,
}

impl Murdock {
    pub fn new(url: &str, api_key: Option<&str>) -> Self {
        let mut configuration = Configuration::new();
        configuration.base_path = String::from(url);
        if let Some(api_key) = api_key {
            configuration.api_key = Some(ApiKey {
                prefix: None,
                key: api_key.to_owned(),
            });
        }
        Self { configuration }
    }

    pub async fn jobs_delete(
        &self,
        before: NaiveDate,
    ) -> Result<
        Vec<openapi::models::JobModel>,
        openapi::apis::Error<openapi::apis::jobs_api::FinishedJobDeleteHandlerJobsDeleteError>,
    > {
        use openapi::apis::jobs_api::finished_job_delete_handler_jobs_delete;
        finished_job_delete_handler_jobs_delete(
            &self.configuration,
            &before.format("%Y-%m-%d").to_string(),
        )
        .await
    }
}

use anyhow::Result;
use chrono::NaiveDate;

use murdock_api;
use murdock_api::apis::configuration::{ApiKey, Configuration};

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
        Vec<murdock_api::models::JobModel>,
        murdock_api::apis::Error<
            murdock_api::apis::jobs_api::FinishedJobDeleteHandlerJobsDeleteError,
        >,
    > {
        use murdock_api::apis::jobs_api::finished_job_delete_handler_jobs_delete;
        finished_job_delete_handler_jobs_delete(
            &self.configuration,
            &before.format("%Y-%m-%d").to_string(),
        )
        .await
    }

    pub async fn job_abort(
        &self,
        uuid: &str,
    ) -> std::result::Result<
        murdock_api::models::JobModel,
        murdock_api::apis::Error<murdock_api::apis::job_api::JobRemoveHandlerJobUidDeleteError>,
    > {
        use murdock_api::apis::job_api::job_remove_handler_job_uid_delete;
        job_remove_handler_job_uid_delete(&self.configuration, uuid).await
    }
}

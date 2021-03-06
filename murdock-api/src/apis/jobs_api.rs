/*
 * Murdock API
 *
 * This is the Murdock API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`finished_job_delete_handler_jobs_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinishedJobDeleteHandlerJobsDeleteError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_handler_jobs_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsHandlerJobsGetError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn finished_job_delete_handler_jobs_delete(configuration: &configuration::Configuration, before: &str) -> Result<Vec<crate::models::JobModel>, Error<FinishedJobDeleteHandlerJobsDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/jobs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("before", &before.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FinishedJobDeleteHandlerJobsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn jobs_handler_jobs_get(configuration: &configuration::Configuration, limit: Option<i32>, uid: Option<&str>, is_pr: Option<bool>, is_branch: Option<bool>, is_tag: Option<bool>, states: Option<&str>, prnum: Option<i32>, prstates: Option<&str>, branch: Option<&str>, tag: Option<&str>, _ref: Option<&str>, sha: Option<&str>, tree: Option<&str>, author: Option<&str>, after: Option<&str>, before: Option<&str>) -> Result<Vec<crate::models::JobModel>, Error<JobsHandlerJobsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/jobs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = uid {
        local_var_req_builder = local_var_req_builder.query(&[("uid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_pr {
        local_var_req_builder = local_var_req_builder.query(&[("is_pr", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_branch {
        local_var_req_builder = local_var_req_builder.query(&[("is_branch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_tag {
        local_var_req_builder = local_var_req_builder.query(&[("is_tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = states {
        local_var_req_builder = local_var_req_builder.query(&[("states", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prnum {
        local_var_req_builder = local_var_req_builder.query(&[("prnum", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prstates {
        local_var_req_builder = local_var_req_builder.query(&[("prstates", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = branch {
        local_var_req_builder = local_var_req_builder.query(&[("branch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _ref {
        local_var_req_builder = local_var_req_builder.query(&[("ref", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sha {
        local_var_req_builder = local_var_req_builder.query(&[("sha", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tree {
        local_var_req_builder = local_var_req_builder.query(&[("tree", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = author {
        local_var_req_builder = local_var_req_builder.query(&[("author", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder = local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = before {
        local_var_req_builder = local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<JobsHandlerJobsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


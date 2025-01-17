/*
 * Nomad
 *
 * Nomad OpenApi specification
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `garbage_collect_allocation`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GarbageCollectAllocationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `garbage_collect_allocation_0`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GarbageCollectAllocation0Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_client_allocation_stats`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientAllocationStatsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_client_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_client_file_at_offest`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientFileAtOffestError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_client_stats`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientStatsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_client_files`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClientFilesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stat_client_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatClientFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stream_client_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamClientFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stream_client_logs`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamClientLogsError {
    UnknownValue(serde_json::Value),
}


pub async fn garbage_collect_allocation(configuration: &configuration::Configuration, alloc_id: &str, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>) -> Result<(), Error<GarbageCollectAllocationError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/allocation/{alloc_id}/gc", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GarbageCollectAllocationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn garbage_collect_allocation_0(configuration: &configuration::Configuration, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, node_id: Option<&str>) -> Result<(), Error<GarbageCollectAllocation0Error>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/gc", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = node_id {
        local_var_req_builder = local_var_req_builder.query(&[("node_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GarbageCollectAllocation0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_client_allocation_stats(configuration: &configuration::Configuration, alloc_id: &str, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>) -> Result<crate::models::AllocResourceUsage, Error<GetClientAllocationStatsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/allocation/{alloc_id}/stats", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientAllocationStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_client_file(configuration: &configuration::Configuration, alloc_id: &str, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, path: Option<&str>) -> Result<String, Error<GetClientFileError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/cat/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_client_file_at_offest(configuration: &configuration::Configuration, alloc_id: &str, offset: i64, limit: i64, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, path: Option<&str>) -> Result<String, Error<GetClientFileAtOffestError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/readat/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("offset", &offset.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("limit", &limit.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientFileAtOffestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_client_stats(configuration: &configuration::Configuration, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, node_id: Option<&str>) -> Result<Vec<crate::models::HostStats>, Error<GetClientStatsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/stats", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = node_id {
        local_var_req_builder = local_var_req_builder.query(&[("node_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_client_files(configuration: &configuration::Configuration, alloc_id: &str, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, path: Option<&str>) -> Result<Vec<crate::models::AllocFileInfo>, Error<ListClientFilesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/ls/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListClientFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn stat_client_file(configuration: &configuration::Configuration, alloc_id: &str, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, path: Option<&str>) -> Result<crate::models::AllocFileInfo, Error<StatClientFileError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/stat/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StatClientFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn stream_client_file(configuration: &configuration::Configuration, alloc_id: &str, offset: i64, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, path: Option<&str>, follow: Option<bool>, origin: Option<&str>) -> Result<String, Error<StreamClientFileError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/stream/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = follow {
        local_var_req_builder = local_var_req_builder.query(&[("follow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("offset", &offset.to_string())]);
    if let Some(ref local_var_str) = origin {
        local_var_req_builder = local_var_req_builder.query(&[("origin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StreamClientFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn stream_client_logs(configuration: &configuration::Configuration, alloc_id: &str, task: &str, offset: i64, namespace: Option<&str>, region: Option<&str>, index: Option<i64>, wait: Option<&str>, follow: Option<bool>, _type: Option<&str>, origin: Option<&str>, plain: Option<bool>) -> Result<String, Error<StreamClientLogsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/client/fs/logs/{alloc_id}", configuration.base_path, alloc_id=crate::apis::urlencode(alloc_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("task", &task.to_string())]);
    if let Some(ref local_var_str) = follow {
        local_var_req_builder = local_var_req_builder.query(&[("follow", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _type {
        local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("offset", &offset.to_string())]);
    if let Some(ref local_var_str) = origin {
        local_var_req_builder = local_var_req_builder.query(&[("origin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = plain {
        local_var_req_builder = local_var_req_builder.query(&[("plain", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StreamClientLogsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


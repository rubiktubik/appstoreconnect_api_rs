/*
 * App Store Connect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`beta_license_agreements_app_get_to_one_related`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaLicenseAgreementsAppGetToOneRelatedError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`beta_license_agreements_get_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaLicenseAgreementsGetCollectionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`beta_license_agreements_get_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaLicenseAgreementsGetInstanceError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`beta_license_agreements_update_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaLicenseAgreementsUpdateInstanceError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status409(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn beta_license_agreements_app_get_to_one_related(configuration: &configuration::Configuration, id: &str, fields_left_square_bracket_apps_right_square_bracket: Option<Vec<String>>) -> Result<models::AppWithoutIncludesResponse, Error<BetaLicenseAgreementsAppGetToOneRelatedError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/betaLicenseAgreements/{id}/app", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields_left_square_bracket_apps_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields[apps]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields[apps]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BetaLicenseAgreementsAppGetToOneRelatedError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn beta_license_agreements_get_collection(configuration: &configuration::Configuration, filter_left_square_bracket_app_right_square_bracket: Option<Vec<String>>, fields_left_square_bracket_beta_license_agreements_right_square_bracket: Option<Vec<String>>, fields_left_square_bracket_apps_right_square_bracket: Option<Vec<String>>, limit: Option<i32>, include: Option<Vec<String>>) -> Result<models::BetaLicenseAgreementsResponse, Error<BetaLicenseAgreementsGetCollectionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/betaLicenseAgreements", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_left_square_bracket_app_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("filter[app]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("filter[app]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = fields_left_square_bracket_beta_license_agreements_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields[betaLicenseAgreements]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields[betaLicenseAgreements]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = fields_left_square_bracket_apps_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields[apps]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields[apps]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("include".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("include", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BetaLicenseAgreementsGetCollectionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn beta_license_agreements_get_instance(configuration: &configuration::Configuration, id: &str, fields_left_square_bracket_beta_license_agreements_right_square_bracket: Option<Vec<String>>, fields_left_square_bracket_apps_right_square_bracket: Option<Vec<String>>, include: Option<Vec<String>>) -> Result<models::BetaLicenseAgreementResponse, Error<BetaLicenseAgreementsGetInstanceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/betaLicenseAgreements/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields_left_square_bracket_beta_license_agreements_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields[betaLicenseAgreements]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields[betaLicenseAgreements]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = fields_left_square_bracket_apps_right_square_bracket {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields[apps]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields[apps]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("include".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("include", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BetaLicenseAgreementsGetInstanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn beta_license_agreements_update_instance(configuration: &configuration::Configuration, id: &str, beta_license_agreement_update_request: models::BetaLicenseAgreementUpdateRequest) -> Result<models::BetaLicenseAgreementResponse, Error<BetaLicenseAgreementsUpdateInstanceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/betaLicenseAgreements/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&beta_license_agreement_update_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BetaLicenseAgreementsUpdateInstanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

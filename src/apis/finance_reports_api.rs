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


/// struct for typed errors of method [`finance_reports_get_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinanceReportsGetCollectionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn finance_reports_get_collection(configuration: &configuration::Configuration, filter_left_square_bracket_vendor_number_right_square_bracket: Vec<String>, filter_left_square_bracket_report_type_right_square_bracket: Vec<String>, filter_left_square_bracket_region_code_right_square_bracket: Vec<String>, filter_left_square_bracket_report_date_right_square_bracket: Vec<String>) -> Result<std::path::PathBuf, Error<FinanceReportsGetCollectionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/financeReports", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(&filter_left_square_bracket_vendor_number_right_square_bracket.into_iter().map(|p| ("filter[vendorNumber]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("filter[vendorNumber]", &filter_left_square_bracket_vendor_number_right_square_bracket.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(&filter_left_square_bracket_report_type_right_square_bracket.into_iter().map(|p| ("filter[reportType]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("filter[reportType]", &filter_left_square_bracket_report_type_right_square_bracket.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(&filter_left_square_bracket_region_code_right_square_bracket.into_iter().map(|p| ("filter[regionCode]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("filter[regionCode]", &filter_left_square_bracket_region_code_right_square_bracket.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(&filter_left_square_bracket_report_date_right_square_bracket.into_iter().map(|p| ("filter[reportDate]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("filter[reportDate]", &filter_left_square_bracket_report_date_right_square_bracket.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
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
        let local_var_entity: Option<FinanceReportsGetCollectionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


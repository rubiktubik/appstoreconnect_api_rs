use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod actors_api;
pub mod age_rating_declarations_api;
pub mod alternative_distribution_domains_api;
pub mod alternative_distribution_keys_api;
pub mod alternative_distribution_package_deltas_api;
pub mod alternative_distribution_package_variants_api;
pub mod alternative_distribution_package_versions_api;
pub mod alternative_distribution_packages_api;
pub mod analytics_report_instances_api;
pub mod analytics_report_requests_api;
pub mod analytics_report_segments_api;
pub mod analytics_reports_api;
pub mod app_availabilities_api;
pub mod app_categories_api;
pub mod app_clip_advanced_experience_images_api;
pub mod app_clip_advanced_experiences_api;
pub mod app_clip_app_store_review_details_api;
pub mod app_clip_default_experience_localizations_api;
pub mod app_clip_default_experiences_api;
pub mod app_clip_header_images_api;
pub mod app_clips_api;
pub mod app_custom_product_page_localizations_api;
pub mod app_custom_product_page_versions_api;
pub mod app_custom_product_pages_api;
pub mod app_encryption_declaration_documents_api;
pub mod app_encryption_declarations_api;
pub mod app_event_localizations_api;
pub mod app_event_screenshots_api;
pub mod app_event_video_clips_api;
pub mod app_events_api;
pub mod app_info_localizations_api;
pub mod app_infos_api;
pub mod app_pre_orders_api;
pub mod app_preview_sets_api;
pub mod app_previews_api;
pub mod app_price_points_api;
pub mod app_price_schedules_api;
pub mod app_screenshot_sets_api;
pub mod app_screenshots_api;
pub mod app_store_review_attachments_api;
pub mod app_store_review_details_api;
pub mod app_store_version_experiment_treatment_localizations_api;
pub mod app_store_version_experiment_treatments_api;
pub mod app_store_version_experiments_api;
pub mod app_store_version_localizations_api;
pub mod app_store_version_phased_releases_api;
pub mod app_store_version_promotions_api;
pub mod app_store_version_release_requests_api;
pub mod app_store_version_submissions_api;
pub mod app_store_versions_api;
pub mod apps_api;
pub mod beta_app_clip_invocation_localizations_api;
pub mod beta_app_clip_invocations_api;
pub mod beta_app_localizations_api;
pub mod beta_app_review_details_api;
pub mod beta_app_review_submissions_api;
pub mod beta_build_localizations_api;
pub mod beta_groups_api;
pub mod beta_license_agreements_api;
pub mod beta_tester_invitations_api;
pub mod beta_testers_api;
pub mod build_beta_details_api;
pub mod build_beta_notifications_api;
pub mod build_bundles_api;
pub mod builds_api;
pub mod bundle_id_capabilities_api;
pub mod bundle_ids_api;
pub mod certificates_api;
pub mod ci_artifacts_api;
pub mod ci_build_actions_api;
pub mod ci_build_runs_api;
pub mod ci_issues_api;
pub mod ci_mac_os_versions_api;
pub mod ci_products_api;
pub mod ci_test_results_api;
pub mod ci_workflows_api;
pub mod ci_xcode_versions_api;
pub mod customer_review_responses_api;
pub mod customer_reviews_api;
pub mod devices_api;
pub mod diagnostic_signatures_api;
pub mod end_app_availability_pre_orders_api;
pub mod end_user_license_agreements_api;
pub mod finance_reports_api;
pub mod game_center_achievement_images_api;
pub mod game_center_achievement_localizations_api;
pub mod game_center_achievement_releases_api;
pub mod game_center_achievements_api;
pub mod game_center_app_versions_api;
pub mod game_center_details_api;
pub mod game_center_enabled_versions_api;
pub mod game_center_groups_api;
pub mod game_center_leaderboard_entry_submissions_api;
pub mod game_center_leaderboard_images_api;
pub mod game_center_leaderboard_localizations_api;
pub mod game_center_leaderboard_releases_api;
pub mod game_center_leaderboard_set_images_api;
pub mod game_center_leaderboard_set_localizations_api;
pub mod game_center_leaderboard_set_member_localizations_api;
pub mod game_center_leaderboard_set_releases_api;
pub mod game_center_leaderboard_sets_api;
pub mod game_center_leaderboards_api;
pub mod game_center_matchmaking_queues_api;
pub mod game_center_matchmaking_rule_set_tests_api;
pub mod game_center_matchmaking_rule_sets_api;
pub mod game_center_matchmaking_rules_api;
pub mod game_center_matchmaking_teams_api;
pub mod game_center_player_achievement_submissions_api;
pub mod in_app_purchase_app_store_review_screenshots_api;
pub mod in_app_purchase_availabilities_api;
pub mod in_app_purchase_contents_api;
pub mod in_app_purchase_images_api;
pub mod in_app_purchase_localizations_api;
pub mod in_app_purchase_price_schedules_api;
pub mod in_app_purchase_submissions_api;
pub mod in_app_purchases_api;
pub mod marketplace_domains_api;
pub mod marketplace_search_details_api;
pub mod marketplace_webhooks_api;
pub mod metrics_api;
pub mod pre_release_versions_api;
pub mod profiles_api;
pub mod promoted_purchase_images_api;
pub mod promoted_purchases_api;
pub mod review_submission_items_api;
pub mod review_submissions_api;
pub mod routing_app_coverages_api;
pub mod sales_reports_api;
pub mod sandbox_testers_api;
pub mod sandbox_testers_clear_purchase_history_request_api;
pub mod scm_git_references_api;
pub mod scm_providers_api;
pub mod scm_pull_requests_api;
pub mod scm_repositories_api;
pub mod subscription_app_store_review_screenshots_api;
pub mod subscription_availabilities_api;
pub mod subscription_grace_periods_api;
pub mod subscription_group_localizations_api;
pub mod subscription_group_submissions_api;
pub mod subscription_groups_api;
pub mod subscription_images_api;
pub mod subscription_introductory_offers_api;
pub mod subscription_localizations_api;
pub mod subscription_offer_code_custom_codes_api;
pub mod subscription_offer_code_one_time_use_codes_api;
pub mod subscription_offer_codes_api;
pub mod subscription_price_points_api;
pub mod subscription_prices_api;
pub mod subscription_promotional_offers_api;
pub mod subscription_submissions_api;
pub mod subscriptions_api;
pub mod territories_api;
pub mod territory_availabilities_api;
pub mod user_invitations_api;
pub mod users_api;
pub mod win_back_offers_api;

pub mod configuration;

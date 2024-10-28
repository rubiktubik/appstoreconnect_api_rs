# \CiBuildRunsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_build_runs_actions_get_to_many_related**](CiBuildRunsApi.md#ci_build_runs_actions_get_to_many_related) | **GET** /v1/ciBuildRuns/{id}/actions | 
[**ci_build_runs_builds_get_to_many_related**](CiBuildRunsApi.md#ci_build_runs_builds_get_to_many_related) | **GET** /v1/ciBuildRuns/{id}/builds | 
[**ci_build_runs_create_instance**](CiBuildRunsApi.md#ci_build_runs_create_instance) | **POST** /v1/ciBuildRuns | 
[**ci_build_runs_get_instance**](CiBuildRunsApi.md#ci_build_runs_get_instance) | **GET** /v1/ciBuildRuns/{id} | 



## ci_build_runs_actions_get_to_many_related

> models::CiBuildActionsResponse ci_build_runs_actions_get_to_many_related(id, fields_left_square_bracket_ci_build_actions_right_square_bracket, fields_left_square_bracket_ci_build_runs_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_build_actions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildActions |  |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CiBuildActionsResponse**](CiBuildActionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_runs_builds_get_to_many_related

> models::BuildsResponse ci_build_runs_builds_get_to_many_related(id, filter_left_square_bracket_version_right_square_bracket, filter_left_square_bracket_expired_right_square_bracket, filter_left_square_bracket_processing_state_right_square_bracket, filter_left_square_bracket_beta_app_review_submission_period_beta_review_state_right_square_bracket, filter_left_square_bracket_uses_non_exempt_encryption_right_square_bracket, filter_left_square_bracket_pre_release_version_period_version_right_square_bracket, filter_left_square_bracket_pre_release_version_period_platform_right_square_bracket, filter_left_square_bracket_build_audience_type_right_square_bracket, filter_left_square_bracket_pre_release_version_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_beta_groups_right_square_bracket, filter_left_square_bracket_app_store_version_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_beta_testers_right_square_bracket, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_beta_build_localizations_right_square_bracket, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_build_icons_right_square_bracket, fields_left_square_bracket_build_bundles_right_square_bracket, limit, include, limit_left_square_bracket_individual_testers_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_beta_build_localizations_right_square_bracket, limit_left_square_bracket_icons_right_square_bracket, limit_left_square_bracket_build_bundles_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'version' |  |
**filter_left_square_bracket_expired_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'expired' |  |
**filter_left_square_bracket_processing_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'processingState' |  |
**filter_left_square_bracket_beta_app_review_submission_period_beta_review_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'betaAppReviewSubmission.betaReviewState' |  |
**filter_left_square_bracket_uses_non_exempt_encryption_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'usesNonExemptEncryption' |  |
**filter_left_square_bracket_pre_release_version_period_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'preReleaseVersion.version' |  |
**filter_left_square_bracket_pre_release_version_period_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'preReleaseVersion.platform' |  |
**filter_left_square_bracket_build_audience_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'buildAudienceType' |  |
**filter_left_square_bracket_pre_release_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'preReleaseVersion' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**filter_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'betaGroups' |  |
**filter_left_square_bracket_app_store_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'appStoreVersion' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_build_icons_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildIcons |  |
**fields_left_square_bracket_build_bundles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBundles |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_individual_testers_right_square_bracket** | Option<**i32**> | maximum number of related individualTesters returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaBuildLocalizations returned (when they are included) |  |
**limit_left_square_bracket_icons_right_square_bracket** | Option<**i32**> | maximum number of related icons returned (when they are included) |  |
**limit_left_square_bracket_build_bundles_right_square_bracket** | Option<**i32**> | maximum number of related buildBundles returned (when they are included) |  |

### Return type

[**models::BuildsResponse**](BuildsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_runs_create_instance

> models::CiBuildRunResponse ci_build_runs_create_instance(ci_build_run_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ci_build_run_create_request** | [**CiBuildRunCreateRequest**](CiBuildRunCreateRequest.md) | CiBuildRun representation | [required] |

### Return type

[**models::CiBuildRunResponse**](CiBuildRunResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_runs_get_instance

> models::CiBuildRunResponse ci_build_runs_get_instance(id, fields_left_square_bracket_ci_build_runs_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::CiBuildRunResponse**](CiBuildRunResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


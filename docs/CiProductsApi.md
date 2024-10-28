# \CiProductsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_products_additional_repositories_get_to_many_related**](CiProductsApi.md#ci_products_additional_repositories_get_to_many_related) | **GET** /v1/ciProducts/{id}/additionalRepositories | 
[**ci_products_app_get_to_one_related**](CiProductsApi.md#ci_products_app_get_to_one_related) | **GET** /v1/ciProducts/{id}/app | 
[**ci_products_build_runs_get_to_many_related**](CiProductsApi.md#ci_products_build_runs_get_to_many_related) | **GET** /v1/ciProducts/{id}/buildRuns | 
[**ci_products_delete_instance**](CiProductsApi.md#ci_products_delete_instance) | **DELETE** /v1/ciProducts/{id} | 
[**ci_products_get_collection**](CiProductsApi.md#ci_products_get_collection) | **GET** /v1/ciProducts | 
[**ci_products_get_instance**](CiProductsApi.md#ci_products_get_instance) | **GET** /v1/ciProducts/{id} | 
[**ci_products_primary_repositories_get_to_many_related**](CiProductsApi.md#ci_products_primary_repositories_get_to_many_related) | **GET** /v1/ciProducts/{id}/primaryRepositories | 
[**ci_products_workflows_get_to_many_related**](CiProductsApi.md#ci_products_workflows_get_to_many_related) | **GET** /v1/ciProducts/{id}/workflows | 



## ci_products_additional_repositories_get_to_many_related

> models::ScmRepositoriesResponse ci_products_additional_repositories_get_to_many_related(id, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, fields_left_square_bracket_scm_providers_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmRepositoriesResponse**](ScmRepositoriesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_app_get_to_one_related

> models::AppResponse ci_products_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_beta_app_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_beta_license_agreements_right_square_bracket, fields_left_square_bracket_beta_app_review_details_right_square_bracket, fields_left_square_bracket_app_infos_right_square_bracket, fields_left_square_bracket_app_clips_right_square_bracket, fields_left_square_bracket_end_user_license_agreements_right_square_bracket, fields_left_square_bracket_app_pre_orders_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, fields_left_square_bracket_subscription_groups_right_square_bracket, fields_left_square_bracket_game_center_enabled_versions_right_square_bracket, fields_left_square_bracket_app_custom_product_pages_right_square_bracket, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_app_events_right_square_bracket, fields_left_square_bracket_review_submissions_right_square_bracket, fields_left_square_bracket_subscription_grace_periods_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, include, limit_left_square_bracket_app_encryption_declarations_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_app_store_versions_right_square_bracket, limit_left_square_bracket_pre_release_versions_right_square_bracket, limit_left_square_bracket_beta_app_localizations_right_square_bracket, limit_left_square_bracket_builds_right_square_bracket, limit_left_square_bracket_app_infos_right_square_bracket, limit_left_square_bracket_app_clips_right_square_bracket, limit_left_square_bracket_in_app_purchases_right_square_bracket, limit_left_square_bracket_subscription_groups_right_square_bracket, limit_left_square_bracket_game_center_enabled_versions_right_square_bracket, limit_left_square_bracket_app_custom_product_pages_right_square_bracket, limit_left_square_bracket_in_app_purchases_v2_right_square_bracket, limit_left_square_bracket_promoted_purchases_right_square_bracket, limit_left_square_bracket_app_events_right_square_bracket, limit_left_square_bracket_review_submissions_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |
**fields_left_square_bracket_beta_app_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppLocalizations |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_beta_license_agreements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaLicenseAgreements |  |
**fields_left_square_bracket_beta_app_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewDetails |  |
**fields_left_square_bracket_app_infos_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfos |  |
**fields_left_square_bracket_app_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClips |  |
**fields_left_square_bracket_end_user_license_agreements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type endUserLicenseAgreements |  |
**fields_left_square_bracket_app_pre_orders_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreOrders |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**fields_left_square_bracket_subscription_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroups |  |
**fields_left_square_bracket_game_center_enabled_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterEnabledVersions |  |
**fields_left_square_bracket_app_custom_product_pages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPages |  |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_app_events_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEvents |  |
**fields_left_square_bracket_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissions |  |
**fields_left_square_bracket_subscription_grace_periods_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGracePeriods |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<**i32**> | maximum number of related appEncryptionDeclarations returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_app_store_versions_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersions returned (when they are included) |  |
**limit_left_square_bracket_pre_release_versions_right_square_bracket** | Option<**i32**> | maximum number of related preReleaseVersions returned (when they are included) |  |
**limit_left_square_bracket_beta_app_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaAppLocalizations returned (when they are included) |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |
**limit_left_square_bracket_app_infos_right_square_bracket** | Option<**i32**> | maximum number of related appInfos returned (when they are included) |  |
**limit_left_square_bracket_app_clips_right_square_bracket** | Option<**i32**> | maximum number of related appClips returned (when they are included) |  |
**limit_left_square_bracket_in_app_purchases_right_square_bracket** | Option<**i32**> | maximum number of related inAppPurchases returned (when they are included) |  |
**limit_left_square_bracket_subscription_groups_right_square_bracket** | Option<**i32**> | maximum number of related subscriptionGroups returned (when they are included) |  |
**limit_left_square_bracket_game_center_enabled_versions_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterEnabledVersions returned (when they are included) |  |
**limit_left_square_bracket_app_custom_product_pages_right_square_bracket** | Option<**i32**> | maximum number of related appCustomProductPages returned (when they are included) |  |
**limit_left_square_bracket_in_app_purchases_v2_right_square_bracket** | Option<**i32**> | maximum number of related inAppPurchasesV2 returned (when they are included) |  |
**limit_left_square_bracket_promoted_purchases_right_square_bracket** | Option<**i32**> | maximum number of related promotedPurchases returned (when they are included) |  |
**limit_left_square_bracket_app_events_right_square_bracket** | Option<**i32**> | maximum number of related appEvents returned (when they are included) |  |
**limit_left_square_bracket_review_submissions_right_square_bracket** | Option<**i32**> | maximum number of related reviewSubmissions returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentsV2 returned (when they are included) |  |

### Return type

[**models::AppResponse**](AppResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_build_runs_get_to_many_related

> models::CiBuildRunsResponse ci_products_build_runs_get_to_many_related(id, filter_left_square_bracket_builds_right_square_bracket, sort, fields_left_square_bracket_ci_build_runs_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_ci_workflows_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, fields_left_square_bracket_scm_pull_requests_right_square_bracket, limit, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_ci_workflows_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciWorkflows |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**fields_left_square_bracket_scm_pull_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmPullRequests |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::CiBuildRunsResponse**](CiBuildRunsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_delete_instance

> ci_products_delete_instance(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_get_collection

> models::CiProductsResponse ci_products_get_collection(filter_left_square_bracket_product_type_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, limit, include, limit_left_square_bracket_primary_repositories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_product_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'productType' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_primary_repositories_right_square_bracket** | Option<**i32**> | maximum number of related primaryRepositories returned (when they are included) |  |

### Return type

[**models::CiProductsResponse**](CiProductsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_get_instance

> models::CiProductResponse ci_products_get_instance(id, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, include, limit_left_square_bracket_primary_repositories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_primary_repositories_right_square_bracket** | Option<**i32**> | maximum number of related primaryRepositories returned (when they are included) |  |

### Return type

[**models::CiProductResponse**](CiProductResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_primary_repositories_get_to_many_related

> models::ScmRepositoriesResponse ci_products_primary_repositories_get_to_many_related(id, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, fields_left_square_bracket_scm_providers_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmRepositoriesResponse**](ScmRepositoriesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_products_workflows_get_to_many_related

> models::CiWorkflowsResponse ci_products_workflows_get_to_many_related(id, fields_left_square_bracket_ci_workflows_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, fields_left_square_bracket_ci_xcode_versions_right_square_bracket, fields_left_square_bracket_ci_mac_os_versions_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_workflows_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciWorkflows |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**fields_left_square_bracket_ci_xcode_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciXcodeVersions |  |
**fields_left_square_bracket_ci_mac_os_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciMacOsVersions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CiWorkflowsResponse**](CiWorkflowsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


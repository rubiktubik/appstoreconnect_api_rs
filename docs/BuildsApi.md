# \BuildsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**builds_app_encryption_declaration_get_to_one_related**](BuildsApi.md#builds_app_encryption_declaration_get_to_one_related) | **GET** /v1/builds/{id}/appEncryptionDeclaration | 
[**builds_app_encryption_declaration_get_to_one_relationship**](BuildsApi.md#builds_app_encryption_declaration_get_to_one_relationship) | **GET** /v1/builds/{id}/relationships/appEncryptionDeclaration | 
[**builds_app_encryption_declaration_update_to_one_relationship**](BuildsApi.md#builds_app_encryption_declaration_update_to_one_relationship) | **PATCH** /v1/builds/{id}/relationships/appEncryptionDeclaration | 
[**builds_app_get_to_one_related**](BuildsApi.md#builds_app_get_to_one_related) | **GET** /v1/builds/{id}/app | 
[**builds_app_store_version_get_to_one_related**](BuildsApi.md#builds_app_store_version_get_to_one_related) | **GET** /v1/builds/{id}/appStoreVersion | 
[**builds_beta_app_review_submission_get_to_one_related**](BuildsApi.md#builds_beta_app_review_submission_get_to_one_related) | **GET** /v1/builds/{id}/betaAppReviewSubmission | 
[**builds_beta_build_localizations_get_to_many_related**](BuildsApi.md#builds_beta_build_localizations_get_to_many_related) | **GET** /v1/builds/{id}/betaBuildLocalizations | 
[**builds_beta_build_usages_get_metrics**](BuildsApi.md#builds_beta_build_usages_get_metrics) | **GET** /v1/builds/{id}/metrics/betaBuildUsages | 
[**builds_beta_groups_create_to_many_relationship**](BuildsApi.md#builds_beta_groups_create_to_many_relationship) | **POST** /v1/builds/{id}/relationships/betaGroups | 
[**builds_beta_groups_delete_to_many_relationship**](BuildsApi.md#builds_beta_groups_delete_to_many_relationship) | **DELETE** /v1/builds/{id}/relationships/betaGroups | 
[**builds_build_beta_detail_get_to_one_related**](BuildsApi.md#builds_build_beta_detail_get_to_one_related) | **GET** /v1/builds/{id}/buildBetaDetail | 
[**builds_diagnostic_signatures_get_to_many_related**](BuildsApi.md#builds_diagnostic_signatures_get_to_many_related) | **GET** /v1/builds/{id}/diagnosticSignatures | 
[**builds_get_collection**](BuildsApi.md#builds_get_collection) | **GET** /v1/builds | 
[**builds_get_instance**](BuildsApi.md#builds_get_instance) | **GET** /v1/builds/{id} | 
[**builds_icons_get_to_many_related**](BuildsApi.md#builds_icons_get_to_many_related) | **GET** /v1/builds/{id}/icons | 
[**builds_individual_testers_create_to_many_relationship**](BuildsApi.md#builds_individual_testers_create_to_many_relationship) | **POST** /v1/builds/{id}/relationships/individualTesters | 
[**builds_individual_testers_delete_to_many_relationship**](BuildsApi.md#builds_individual_testers_delete_to_many_relationship) | **DELETE** /v1/builds/{id}/relationships/individualTesters | 
[**builds_individual_testers_get_to_many_related**](BuildsApi.md#builds_individual_testers_get_to_many_related) | **GET** /v1/builds/{id}/individualTesters | 
[**builds_individual_testers_get_to_many_relationship**](BuildsApi.md#builds_individual_testers_get_to_many_relationship) | **GET** /v1/builds/{id}/relationships/individualTesters | 
[**builds_perf_power_metrics_get_to_many_related**](BuildsApi.md#builds_perf_power_metrics_get_to_many_related) | **GET** /v1/builds/{id}/perfPowerMetrics | 
[**builds_pre_release_version_get_to_one_related**](BuildsApi.md#builds_pre_release_version_get_to_one_related) | **GET** /v1/builds/{id}/preReleaseVersion | 
[**builds_update_instance**](BuildsApi.md#builds_update_instance) | **PATCH** /v1/builds/{id} | 



## builds_app_encryption_declaration_get_to_one_related

> models::AppEncryptionDeclarationWithoutIncludesResponse builds_app_encryption_declaration_get_to_one_related(id, fields_left_square_bracket_app_encryption_declarations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |

### Return type

[**models::AppEncryptionDeclarationWithoutIncludesResponse**](AppEncryptionDeclarationWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_app_encryption_declaration_get_to_one_relationship

> models::BuildAppEncryptionDeclarationLinkageResponse builds_app_encryption_declaration_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::BuildAppEncryptionDeclarationLinkageResponse**](BuildAppEncryptionDeclarationLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_app_encryption_declaration_update_to_one_relationship

> builds_app_encryption_declaration_update_to_one_relationship(id, build_app_encryption_declaration_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_app_encryption_declaration_linkage_request** | [**BuildAppEncryptionDeclarationLinkageRequest**](BuildAppEncryptionDeclarationLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_app_get_to_one_related

> models::AppWithoutIncludesResponse builds_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |

### Return type

[**models::AppWithoutIncludesResponse**](AppWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_app_store_version_get_to_one_related

> models::AppStoreVersionResponse builds_app_store_version_get_to_one_related(id, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_age_rating_declarations_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_routing_app_coverages_right_square_bracket, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_version_submissions_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, include, limit_left_square_bracket_app_store_version_localizations_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_age_rating_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ageRatingDeclarations |  |
**fields_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionLocalizations |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionPhasedReleases |  |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_routing_app_coverages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type routingAppCoverages |  |
**fields_left_square_bracket_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewDetails |  |
**fields_left_square_bracket_app_store_version_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionSubmissions |  |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_alternative_distribution_packages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionLocalizations returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperiments returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentsV2 returned (when they are included) |  |

### Return type

[**models::AppStoreVersionResponse**](AppStoreVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_beta_app_review_submission_get_to_one_related

> models::BetaAppReviewSubmissionWithoutIncludesResponse builds_beta_app_review_submission_get_to_one_related(id, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |

### Return type

[**models::BetaAppReviewSubmissionWithoutIncludesResponse**](BetaAppReviewSubmissionWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_beta_build_localizations_get_to_many_related

> models::BetaBuildLocalizationsWithoutIncludesResponse builds_beta_build_localizations_get_to_many_related(id, fields_left_square_bracket_beta_build_localizations_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaBuildLocalizationsWithoutIncludesResponse**](BetaBuildLocalizationsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_beta_build_usages_get_metrics

> models::BetaBuildUsagesV1MetricResponse builds_beta_build_usages_get_metrics(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::BetaBuildUsagesV1MetricResponse**](BetaBuildUsagesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_beta_groups_create_to_many_relationship

> builds_beta_groups_create_to_many_relationship(id, build_beta_groups_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_beta_groups_linkages_request** | [**BuildBetaGroupsLinkagesRequest**](BuildBetaGroupsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_beta_groups_delete_to_many_relationship

> builds_beta_groups_delete_to_many_relationship(id, build_beta_groups_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_beta_groups_linkages_request** | [**BuildBetaGroupsLinkagesRequest**](BuildBetaGroupsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_build_beta_detail_get_to_one_related

> models::BuildBetaDetailResponse builds_build_beta_detail_get_to_one_related(id, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BuildBetaDetailResponse**](BuildBetaDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_diagnostic_signatures_get_to_many_related

> models::DiagnosticSignaturesResponse builds_diagnostic_signatures_get_to_many_related(id, filter_left_square_bracket_diagnostic_type_right_square_bracket, fields_left_square_bracket_diagnostic_signatures_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_diagnostic_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'diagnosticType' |  |
**fields_left_square_bracket_diagnostic_signatures_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type diagnosticSignatures |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::DiagnosticSignaturesResponse**](DiagnosticSignaturesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_get_collection

> models::BuildsResponse builds_get_collection(filter_left_square_bracket_version_right_square_bracket, filter_left_square_bracket_expired_right_square_bracket, filter_left_square_bracket_processing_state_right_square_bracket, filter_left_square_bracket_beta_app_review_submission_period_beta_review_state_right_square_bracket, filter_left_square_bracket_uses_non_exempt_encryption_right_square_bracket, filter_left_square_bracket_pre_release_version_period_version_right_square_bracket, filter_left_square_bracket_pre_release_version_period_platform_right_square_bracket, filter_left_square_bracket_build_audience_type_right_square_bracket, filter_left_square_bracket_pre_release_version_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_beta_groups_right_square_bracket, filter_left_square_bracket_app_store_version_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_beta_testers_right_square_bracket, fields_left_square_bracket_beta_build_localizations_right_square_bracket, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_build_icons_right_square_bracket, limit, include, limit_left_square_bracket_beta_build_localizations_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_build_bundles_right_square_bracket, limit_left_square_bracket_icons_right_square_bracket, limit_left_square_bracket_individual_testers_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_build_icons_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildIcons |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaBuildLocalizations returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_build_bundles_right_square_bracket** | Option<**i32**> | maximum number of related buildBundles returned (when they are included) |  |
**limit_left_square_bracket_icons_right_square_bracket** | Option<**i32**> | maximum number of related icons returned (when they are included) |  |
**limit_left_square_bracket_individual_testers_right_square_bracket** | Option<**i32**> | maximum number of related individualTesters returned (when they are included) |  |

### Return type

[**models::BuildsResponse**](BuildsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_get_instance

> models::BuildResponse builds_get_instance(id, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_beta_testers_right_square_bracket, fields_left_square_bracket_beta_build_localizations_right_square_bracket, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_build_icons_right_square_bracket, include, limit_left_square_bracket_beta_build_localizations_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_build_bundles_right_square_bracket, limit_left_square_bracket_icons_right_square_bracket, limit_left_square_bracket_individual_testers_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_build_icons_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildIcons |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaBuildLocalizations returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_build_bundles_right_square_bracket** | Option<**i32**> | maximum number of related buildBundles returned (when they are included) |  |
**limit_left_square_bracket_icons_right_square_bracket** | Option<**i32**> | maximum number of related icons returned (when they are included) |  |
**limit_left_square_bracket_individual_testers_right_square_bracket** | Option<**i32**> | maximum number of related individualTesters returned (when they are included) |  |

### Return type

[**models::BuildResponse**](BuildResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_icons_get_to_many_related

> models::BuildIconsWithoutIncludesResponse builds_icons_get_to_many_related(id, fields_left_square_bracket_build_icons_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_build_icons_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildIcons |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BuildIconsWithoutIncludesResponse**](BuildIconsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_individual_testers_create_to_many_relationship

> builds_individual_testers_create_to_many_relationship(id, build_individual_testers_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_individual_testers_linkages_request** | [**BuildIndividualTestersLinkagesRequest**](BuildIndividualTestersLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_individual_testers_delete_to_many_relationship

> builds_individual_testers_delete_to_many_relationship(id, build_individual_testers_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_individual_testers_linkages_request** | [**BuildIndividualTestersLinkagesRequest**](BuildIndividualTestersLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_individual_testers_get_to_many_related

> models::BetaTestersWithoutIncludesResponse builds_individual_testers_get_to_many_related(id, fields_left_square_bracket_beta_testers_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaTestersWithoutIncludesResponse**](BetaTestersWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_individual_testers_get_to_many_relationship

> models::BuildIndividualTestersLinkagesResponse builds_individual_testers_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BuildIndividualTestersLinkagesResponse**](BuildIndividualTestersLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_perf_power_metrics_get_to_many_related

> models::XcodeMetrics builds_perf_power_metrics_get_to_many_related(id, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_metric_type_right_square_bracket, filter_left_square_bracket_device_type_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_metric_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'metricType' |  |
**filter_left_square_bracket_device_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'deviceType' |  |

### Return type

[**models::XcodeMetrics**](xcodeMetrics.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.apple.xcode-metrics+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_pre_release_version_get_to_one_related

> models::PrereleaseVersionWithoutIncludesResponse builds_pre_release_version_get_to_one_related(id, fields_left_square_bracket_pre_release_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |

### Return type

[**models::PrereleaseVersionWithoutIncludesResponse**](PrereleaseVersionWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_update_instance

> models::BuildResponse builds_update_instance(id, build_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_update_request** | [**BuildUpdateRequest**](BuildUpdateRequest.md) | Build representation | [required] |

### Return type

[**models::BuildResponse**](BuildResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


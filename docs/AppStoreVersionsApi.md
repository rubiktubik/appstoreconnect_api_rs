# \AppStoreVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_versions_age_rating_declaration_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_age_rating_declaration_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/ageRatingDeclaration | 
[**app_store_versions_alternative_distribution_package_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_alternative_distribution_package_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/alternativeDistributionPackage | 
[**app_store_versions_app_clip_default_experience_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_app_clip_default_experience_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/appClipDefaultExperience | 
[**app_store_versions_app_clip_default_experience_get_to_one_relationship**](AppStoreVersionsApi.md#app_store_versions_app_clip_default_experience_get_to_one_relationship) | **GET** /v1/appStoreVersions/{id}/relationships/appClipDefaultExperience | 
[**app_store_versions_app_clip_default_experience_update_to_one_relationship**](AppStoreVersionsApi.md#app_store_versions_app_clip_default_experience_update_to_one_relationship) | **PATCH** /v1/appStoreVersions/{id}/relationships/appClipDefaultExperience | 
[**app_store_versions_app_store_review_detail_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_app_store_review_detail_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/appStoreReviewDetail | 
[**app_store_versions_app_store_version_experiments_get_to_many_related**](AppStoreVersionsApi.md#app_store_versions_app_store_version_experiments_get_to_many_related) | **GET** /v1/appStoreVersions/{id}/appStoreVersionExperiments | 
[**app_store_versions_app_store_version_experiments_v2_get_to_many_related**](AppStoreVersionsApi.md#app_store_versions_app_store_version_experiments_v2_get_to_many_related) | **GET** /v1/appStoreVersions/{id}/appStoreVersionExperimentsV2 | 
[**app_store_versions_app_store_version_localizations_get_to_many_related**](AppStoreVersionsApi.md#app_store_versions_app_store_version_localizations_get_to_many_related) | **GET** /v1/appStoreVersions/{id}/appStoreVersionLocalizations | 
[**app_store_versions_app_store_version_phased_release_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_app_store_version_phased_release_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/appStoreVersionPhasedRelease | 
[**app_store_versions_app_store_version_submission_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_app_store_version_submission_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/appStoreVersionSubmission | 
[**app_store_versions_build_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_build_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/build | 
[**app_store_versions_build_get_to_one_relationship**](AppStoreVersionsApi.md#app_store_versions_build_get_to_one_relationship) | **GET** /v1/appStoreVersions/{id}/relationships/build | 
[**app_store_versions_build_update_to_one_relationship**](AppStoreVersionsApi.md#app_store_versions_build_update_to_one_relationship) | **PATCH** /v1/appStoreVersions/{id}/relationships/build | 
[**app_store_versions_create_instance**](AppStoreVersionsApi.md#app_store_versions_create_instance) | **POST** /v1/appStoreVersions | 
[**app_store_versions_customer_reviews_get_to_many_related**](AppStoreVersionsApi.md#app_store_versions_customer_reviews_get_to_many_related) | **GET** /v1/appStoreVersions/{id}/customerReviews | 
[**app_store_versions_delete_instance**](AppStoreVersionsApi.md#app_store_versions_delete_instance) | **DELETE** /v1/appStoreVersions/{id} | 
[**app_store_versions_game_center_app_version_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_game_center_app_version_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/gameCenterAppVersion | 
[**app_store_versions_get_instance**](AppStoreVersionsApi.md#app_store_versions_get_instance) | **GET** /v1/appStoreVersions/{id} | 
[**app_store_versions_routing_app_coverage_get_to_one_related**](AppStoreVersionsApi.md#app_store_versions_routing_app_coverage_get_to_one_related) | **GET** /v1/appStoreVersions/{id}/routingAppCoverage | 
[**app_store_versions_update_instance**](AppStoreVersionsApi.md#app_store_versions_update_instance) | **PATCH** /v1/appStoreVersions/{id} | 



## app_store_versions_age_rating_declaration_get_to_one_related

> models::AgeRatingDeclarationWithoutIncludesResponse app_store_versions_age_rating_declaration_get_to_one_related(id, fields_left_square_bracket_age_rating_declarations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_age_rating_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ageRatingDeclarations |  |

### Return type

[**models::AgeRatingDeclarationWithoutIncludesResponse**](AgeRatingDeclarationWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_alternative_distribution_package_get_to_one_related

> models::AlternativeDistributionPackageResponse app_store_versions_alternative_distribution_package_get_to_one_related(id, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket, include, limit_left_square_bracket_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_packages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackages |  |
**fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_versions_right_square_bracket** | Option<**i32**> | maximum number of related versions returned (when they are included) |  |

### Return type

[**models::AlternativeDistributionPackageResponse**](AlternativeDistributionPackageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_clip_default_experience_get_to_one_related

> models::AppClipDefaultExperienceResponse app_store_versions_app_clip_default_experience_get_to_one_related(id, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_clips_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket, include, limit_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**fields_left_square_bracket_app_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClips |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperienceLocalizations |  |
**fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAppStoreReviewDetails |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appClipDefaultExperienceLocalizations returned (when they are included) |  |

### Return type

[**models::AppClipDefaultExperienceResponse**](AppClipDefaultExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_clip_default_experience_get_to_one_relationship

> models::AppStoreVersionAppClipDefaultExperienceLinkageResponse app_store_versions_app_clip_default_experience_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::AppStoreVersionAppClipDefaultExperienceLinkageResponse**](AppStoreVersionAppClipDefaultExperienceLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_clip_default_experience_update_to_one_relationship

> app_store_versions_app_clip_default_experience_update_to_one_relationship(id, app_store_version_app_clip_default_experience_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_app_clip_default_experience_linkage_request** | [**AppStoreVersionAppClipDefaultExperienceLinkageRequest**](AppStoreVersionAppClipDefaultExperienceLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_review_detail_get_to_one_related

> models::AppStoreReviewDetailResponse app_store_versions_app_store_review_detail_get_to_one_related(id, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_store_review_attachments_right_square_bracket, include, limit_left_square_bracket_app_store_review_attachments_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewDetails |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewAttachments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreReviewAttachments returned (when they are included) |  |

### Return type

[**models::AppStoreReviewDetailResponse**](AppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_version_experiments_get_to_many_related

> models::AppStoreVersionExperimentsResponse app_store_versions_app_store_version_experiments_get_to_many_related(id, filter_left_square_bracket_state_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, limit, include, limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatments returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentsResponse**](AppStoreVersionExperimentsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_version_experiments_v2_get_to_many_related

> models::AppStoreVersionExperimentsV2Response app_store_versions_app_store_version_experiments_v2_get_to_many_related(id, filter_left_square_bracket_state_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, limit, include, limit_left_square_bracket_control_versions_right_square_bracket, limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_control_versions_right_square_bracket** | Option<**i32**> | maximum number of related controlVersions returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatments returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentsV2Response**](AppStoreVersionExperimentsV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_version_localizations_get_to_many_related

> models::AppStoreVersionLocalizationsResponse app_store_versions_app_store_version_localizations_get_to_many_related(id, filter_left_square_bracket_locale_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, limit, include, limit_left_square_bracket_app_screenshot_sets_right_square_bracket, limit_left_square_bracket_app_preview_sets_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**fields_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionLocalizations |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshotSets returned (when they are included) |  |
**limit_left_square_bracket_app_preview_sets_right_square_bracket** | Option<**i32**> | maximum number of related appPreviewSets returned (when they are included) |  |

### Return type

[**models::AppStoreVersionLocalizationsResponse**](AppStoreVersionLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_version_phased_release_get_to_one_related

> models::AppStoreVersionPhasedReleaseWithoutIncludesResponse app_store_versions_app_store_version_phased_release_get_to_one_related(id, fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionPhasedReleases |  |

### Return type

[**models::AppStoreVersionPhasedReleaseWithoutIncludesResponse**](AppStoreVersionPhasedReleaseWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_app_store_version_submission_get_to_one_related

> models::AppStoreVersionSubmissionResponse app_store_versions_app_store_version_submission_get_to_one_related(id, fields_left_square_bracket_app_store_version_submissions_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionSubmissions |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppStoreVersionSubmissionResponse**](AppStoreVersionSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_build_get_to_one_related

> models::BuildWithoutIncludesResponse app_store_versions_build_get_to_one_related(id, fields_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |

### Return type

[**models::BuildWithoutIncludesResponse**](BuildWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_build_get_to_one_relationship

> models::AppStoreVersionBuildLinkageResponse app_store_versions_build_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::AppStoreVersionBuildLinkageResponse**](AppStoreVersionBuildLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_build_update_to_one_relationship

> app_store_versions_build_update_to_one_relationship(id, app_store_version_build_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_build_linkage_request** | [**AppStoreVersionBuildLinkageRequest**](AppStoreVersionBuildLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_create_instance

> models::AppStoreVersionResponse app_store_versions_create_instance(app_store_version_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_create_request** | [**AppStoreVersionCreateRequest**](AppStoreVersionCreateRequest.md) | AppStoreVersion representation | [required] |

### Return type

[**models::AppStoreVersionResponse**](AppStoreVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_customer_reviews_get_to_many_related

> models::CustomerReviewsResponse app_store_versions_customer_reviews_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, filter_left_square_bracket_rating_right_square_bracket, exists_left_square_bracket_published_response_right_square_bracket, sort, fields_left_square_bracket_customer_reviews_right_square_bracket, fields_left_square_bracket_customer_review_responses_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'territory' |  |
**filter_left_square_bracket_rating_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'rating' |  |
**exists_left_square_bracket_published_response_right_square_bracket** | Option<**bool**> | filter by publishedResponse |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_customer_reviews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviews |  |
**fields_left_square_bracket_customer_review_responses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviewResponses |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CustomerReviewsResponse**](CustomerReviewsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_delete_instance

> app_store_versions_delete_instance(id)


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


## app_store_versions_game_center_app_version_get_to_one_related

> models::GameCenterAppVersionResponse app_store_versions_game_center_app_version_get_to_one_related(id, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, include, limit_left_square_bracket_compatibility_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_compatibility_versions_right_square_bracket** | Option<**i32**> | maximum number of related compatibilityVersions returned (when they are included) |  |

### Return type

[**models::GameCenterAppVersionResponse**](GameCenterAppVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_get_instance

> models::AppStoreVersionResponse app_store_versions_get_instance(id, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_age_rating_declarations_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_routing_app_coverages_right_square_bracket, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_version_submissions_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, include, limit_left_square_bracket_app_store_version_experiments_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket, limit_left_square_bracket_app_store_version_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
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
**limit_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperiments returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentsV2 returned (when they are included) |  |
**limit_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionLocalizations returned (when they are included) |  |

### Return type

[**models::AppStoreVersionResponse**](AppStoreVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_routing_app_coverage_get_to_one_related

> models::RoutingAppCoverageWithoutIncludesResponse app_store_versions_routing_app_coverage_get_to_one_related(id, fields_left_square_bracket_routing_app_coverages_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_routing_app_coverages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type routingAppCoverages |  |

### Return type

[**models::RoutingAppCoverageWithoutIncludesResponse**](RoutingAppCoverageWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_versions_update_instance

> models::AppStoreVersionResponse app_store_versions_update_instance(id, app_store_version_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_update_request** | [**AppStoreVersionUpdateRequest**](AppStoreVersionUpdateRequest.md) | AppStoreVersion representation | [required] |

### Return type

[**models::AppStoreVersionResponse**](AppStoreVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


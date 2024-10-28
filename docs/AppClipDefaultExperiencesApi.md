# \AppClipDefaultExperiencesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clip_default_experiences_app_clip_app_store_review_detail_get_to_one_related**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_app_clip_app_store_review_detail_get_to_one_related) | **GET** /v1/appClipDefaultExperiences/{id}/appClipAppStoreReviewDetail | 
[**app_clip_default_experiences_app_clip_default_experience_localizations_get_to_many_related**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_app_clip_default_experience_localizations_get_to_many_related) | **GET** /v1/appClipDefaultExperiences/{id}/appClipDefaultExperienceLocalizations | 
[**app_clip_default_experiences_create_instance**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_create_instance) | **POST** /v1/appClipDefaultExperiences | 
[**app_clip_default_experiences_delete_instance**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_delete_instance) | **DELETE** /v1/appClipDefaultExperiences/{id} | 
[**app_clip_default_experiences_get_instance**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_get_instance) | **GET** /v1/appClipDefaultExperiences/{id} | 
[**app_clip_default_experiences_release_with_app_store_version_get_to_one_related**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_release_with_app_store_version_get_to_one_related) | **GET** /v1/appClipDefaultExperiences/{id}/releaseWithAppStoreVersion | 
[**app_clip_default_experiences_release_with_app_store_version_get_to_one_relationship**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_release_with_app_store_version_get_to_one_relationship) | **GET** /v1/appClipDefaultExperiences/{id}/relationships/releaseWithAppStoreVersion | 
[**app_clip_default_experiences_release_with_app_store_version_update_to_one_relationship**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_release_with_app_store_version_update_to_one_relationship) | **PATCH** /v1/appClipDefaultExperiences/{id}/relationships/releaseWithAppStoreVersion | 
[**app_clip_default_experiences_update_instance**](AppClipDefaultExperiencesApi.md#app_clip_default_experiences_update_instance) | **PATCH** /v1/appClipDefaultExperiences/{id} | 



## app_clip_default_experiences_app_clip_app_store_review_detail_get_to_one_related

> models::AppClipAppStoreReviewDetailResponse app_clip_default_experiences_app_clip_app_store_review_detail_get_to_one_related(id, fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAppStoreReviewDetails |  |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipAppStoreReviewDetailResponse**](AppClipAppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experiences_app_clip_default_experience_localizations_get_to_many_related

> models::AppClipDefaultExperienceLocalizationsResponse app_clip_default_experiences_app_clip_default_experience_localizations_get_to_many_related(id, filter_left_square_bracket_locale_right_square_bracket, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_clip_header_images_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperienceLocalizations |  |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**fields_left_square_bracket_app_clip_header_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipHeaderImages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipDefaultExperienceLocalizationsResponse**](AppClipDefaultExperienceLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experiences_create_instance

> models::AppClipDefaultExperienceResponse app_clip_default_experiences_create_instance(app_clip_default_experience_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_clip_default_experience_create_request** | [**AppClipDefaultExperienceCreateRequest**](AppClipDefaultExperienceCreateRequest.md) | AppClipDefaultExperience representation | [required] |

### Return type

[**models::AppClipDefaultExperienceResponse**](AppClipDefaultExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experiences_delete_instance

> app_clip_default_experiences_delete_instance(id)


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


## app_clip_default_experiences_get_instance

> models::AppClipDefaultExperienceResponse app_clip_default_experiences_get_instance(id, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket, include, limit_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
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


## app_clip_default_experiences_release_with_app_store_version_get_to_one_related

> models::AppStoreVersionResponse app_clip_default_experiences_release_with_app_store_version_get_to_one_related(id, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_age_rating_declarations_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_routing_app_coverages_right_square_bracket, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_version_submissions_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, include, limit_left_square_bracket_app_store_version_localizations_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket)


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


## app_clip_default_experiences_release_with_app_store_version_get_to_one_relationship

> models::AppClipDefaultExperienceReleaseWithAppStoreVersionLinkageResponse app_clip_default_experiences_release_with_app_store_version_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::AppClipDefaultExperienceReleaseWithAppStoreVersionLinkageResponse**](AppClipDefaultExperienceReleaseWithAppStoreVersionLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experiences_release_with_app_store_version_update_to_one_relationship

> app_clip_default_experiences_release_with_app_store_version_update_to_one_relationship(id, app_clip_default_experience_release_with_app_store_version_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_default_experience_release_with_app_store_version_linkage_request** | [**AppClipDefaultExperienceReleaseWithAppStoreVersionLinkageRequest**](AppClipDefaultExperienceReleaseWithAppStoreVersionLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experiences_update_instance

> models::AppClipDefaultExperienceResponse app_clip_default_experiences_update_instance(id, app_clip_default_experience_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_default_experience_update_request** | [**AppClipDefaultExperienceUpdateRequest**](AppClipDefaultExperienceUpdateRequest.md) | AppClipDefaultExperience representation | [required] |

### Return type

[**models::AppClipDefaultExperienceResponse**](AppClipDefaultExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


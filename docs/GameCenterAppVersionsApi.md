# \GameCenterAppVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_app_versions_app_store_version_get_to_one_related**](GameCenterAppVersionsApi.md#game_center_app_versions_app_store_version_get_to_one_related) | **GET** /v1/gameCenterAppVersions/{id}/appStoreVersion | 
[**game_center_app_versions_compatibility_versions_create_to_many_relationship**](GameCenterAppVersionsApi.md#game_center_app_versions_compatibility_versions_create_to_many_relationship) | **POST** /v1/gameCenterAppVersions/{id}/relationships/compatibilityVersions | 
[**game_center_app_versions_compatibility_versions_delete_to_many_relationship**](GameCenterAppVersionsApi.md#game_center_app_versions_compatibility_versions_delete_to_many_relationship) | **DELETE** /v1/gameCenterAppVersions/{id}/relationships/compatibilityVersions | 
[**game_center_app_versions_compatibility_versions_get_to_many_related**](GameCenterAppVersionsApi.md#game_center_app_versions_compatibility_versions_get_to_many_related) | **GET** /v1/gameCenterAppVersions/{id}/compatibilityVersions | 
[**game_center_app_versions_compatibility_versions_get_to_many_relationship**](GameCenterAppVersionsApi.md#game_center_app_versions_compatibility_versions_get_to_many_relationship) | **GET** /v1/gameCenterAppVersions/{id}/relationships/compatibilityVersions | 
[**game_center_app_versions_create_instance**](GameCenterAppVersionsApi.md#game_center_app_versions_create_instance) | **POST** /v1/gameCenterAppVersions | 
[**game_center_app_versions_get_instance**](GameCenterAppVersionsApi.md#game_center_app_versions_get_instance) | **GET** /v1/gameCenterAppVersions/{id} | 
[**game_center_app_versions_update_instance**](GameCenterAppVersionsApi.md#game_center_app_versions_update_instance) | **PATCH** /v1/gameCenterAppVersions/{id} | 



## game_center_app_versions_app_store_version_get_to_one_related

> models::AppStoreVersionResponse game_center_app_versions_app_store_version_get_to_one_related(id, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_age_rating_declarations_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_app_store_version_phased_releases_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_routing_app_coverages_right_square_bracket, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_version_submissions_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, include, limit_left_square_bracket_app_store_version_localizations_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_right_square_bracket, limit_left_square_bracket_app_store_version_experiments_v2_right_square_bracket)


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


## game_center_app_versions_compatibility_versions_create_to_many_relationship

> game_center_app_versions_compatibility_versions_create_to_many_relationship(id, game_center_app_version_compatibility_versions_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_app_version_compatibility_versions_linkages_request** | [**GameCenterAppVersionCompatibilityVersionsLinkagesRequest**](GameCenterAppVersionCompatibilityVersionsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_app_versions_compatibility_versions_delete_to_many_relationship

> game_center_app_versions_compatibility_versions_delete_to_many_relationship(id, game_center_app_version_compatibility_versions_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_app_version_compatibility_versions_linkages_request** | [**GameCenterAppVersionCompatibilityVersionsLinkagesRequest**](GameCenterAppVersionCompatibilityVersionsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_app_versions_compatibility_versions_get_to_many_related

> models::GameCenterAppVersionsResponse game_center_app_versions_compatibility_versions_get_to_many_related(id, filter_left_square_bracket_enabled_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, limit, include, limit_left_square_bracket_compatibility_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_enabled_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'enabled' |  |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_compatibility_versions_right_square_bracket** | Option<**i32**> | maximum number of related compatibilityVersions returned (when they are included) |  |

### Return type

[**models::GameCenterAppVersionsResponse**](GameCenterAppVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_app_versions_compatibility_versions_get_to_many_relationship

> models::GameCenterAppVersionCompatibilityVersionsLinkagesResponse game_center_app_versions_compatibility_versions_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterAppVersionCompatibilityVersionsLinkagesResponse**](GameCenterAppVersionCompatibilityVersionsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_app_versions_create_instance

> models::GameCenterAppVersionResponse game_center_app_versions_create_instance(game_center_app_version_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_app_version_create_request** | [**GameCenterAppVersionCreateRequest**](GameCenterAppVersionCreateRequest.md) | GameCenterAppVersion representation | [required] |

### Return type

[**models::GameCenterAppVersionResponse**](GameCenterAppVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_app_versions_get_instance

> models::GameCenterAppVersionResponse game_center_app_versions_get_instance(id, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, include, limit_left_square_bracket_compatibility_versions_right_square_bracket)


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


## game_center_app_versions_update_instance

> models::GameCenterAppVersionResponse game_center_app_versions_update_instance(id, game_center_app_version_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_app_version_update_request** | [**GameCenterAppVersionUpdateRequest**](GameCenterAppVersionUpdateRequest.md) | GameCenterAppVersion representation | [required] |

### Return type

[**models::GameCenterAppVersionResponse**](GameCenterAppVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


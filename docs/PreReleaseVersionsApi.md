# \PreReleaseVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pre_release_versions_app_get_to_one_related**](PreReleaseVersionsApi.md#pre_release_versions_app_get_to_one_related) | **GET** /v1/preReleaseVersions/{id}/app | 
[**pre_release_versions_builds_get_to_many_related**](PreReleaseVersionsApi.md#pre_release_versions_builds_get_to_many_related) | **GET** /v1/preReleaseVersions/{id}/builds | 
[**pre_release_versions_get_collection**](PreReleaseVersionsApi.md#pre_release_versions_get_collection) | **GET** /v1/preReleaseVersions | 
[**pre_release_versions_get_instance**](PreReleaseVersionsApi.md#pre_release_versions_get_instance) | **GET** /v1/preReleaseVersions/{id} | 



## pre_release_versions_app_get_to_one_related

> models::AppWithoutIncludesResponse pre_release_versions_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


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


## pre_release_versions_builds_get_to_many_related

> models::BuildsWithoutIncludesResponse pre_release_versions_builds_get_to_many_related(id, fields_left_square_bracket_builds_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BuildsWithoutIncludesResponse**](BuildsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pre_release_versions_get_collection

> models::PreReleaseVersionsResponse pre_release_versions_get_collection(filter_left_square_bracket_builds_period_expired_right_square_bracket, filter_left_square_bracket_builds_period_processing_state_right_square_bracket, filter_left_square_bracket_builds_period_version_right_square_bracket, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_version_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_builds_right_square_bracket, sort, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_builds_period_expired_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'builds.expired' |  |
**filter_left_square_bracket_builds_period_processing_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'builds.processingState' |  |
**filter_left_square_bracket_builds_period_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'builds.version' |  |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'version' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::PreReleaseVersionsResponse**](PreReleaseVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pre_release_versions_get_instance

> models::PrereleaseVersionResponse pre_release_versions_get_instance(id, fields_left_square_bracket_pre_release_versions_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_pre_release_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type preReleaseVersions |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::PrereleaseVersionResponse**](PrereleaseVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


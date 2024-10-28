# \GameCenterEnabledVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_enabled_versions_compatible_versions_create_to_many_relationship**](GameCenterEnabledVersionsApi.md#game_center_enabled_versions_compatible_versions_create_to_many_relationship) | **POST** /v1/gameCenterEnabledVersions/{id}/relationships/compatibleVersions | 
[**game_center_enabled_versions_compatible_versions_delete_to_many_relationship**](GameCenterEnabledVersionsApi.md#game_center_enabled_versions_compatible_versions_delete_to_many_relationship) | **DELETE** /v1/gameCenterEnabledVersions/{id}/relationships/compatibleVersions | 
[**game_center_enabled_versions_compatible_versions_get_to_many_related**](GameCenterEnabledVersionsApi.md#game_center_enabled_versions_compatible_versions_get_to_many_related) | **GET** /v1/gameCenterEnabledVersions/{id}/compatibleVersions | 
[**game_center_enabled_versions_compatible_versions_get_to_many_relationship**](GameCenterEnabledVersionsApi.md#game_center_enabled_versions_compatible_versions_get_to_many_relationship) | **GET** /v1/gameCenterEnabledVersions/{id}/relationships/compatibleVersions | 
[**game_center_enabled_versions_compatible_versions_replace_to_many_relationship**](GameCenterEnabledVersionsApi.md#game_center_enabled_versions_compatible_versions_replace_to_many_relationship) | **PATCH** /v1/gameCenterEnabledVersions/{id}/relationships/compatibleVersions | 



## game_center_enabled_versions_compatible_versions_create_to_many_relationship

> game_center_enabled_versions_compatible_versions_create_to_many_relationship(id, game_center_enabled_version_compatible_versions_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_enabled_version_compatible_versions_linkages_request** | [**GameCenterEnabledVersionCompatibleVersionsLinkagesRequest**](GameCenterEnabledVersionCompatibleVersionsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_enabled_versions_compatible_versions_delete_to_many_relationship

> game_center_enabled_versions_compatible_versions_delete_to_many_relationship(id, game_center_enabled_version_compatible_versions_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_enabled_version_compatible_versions_linkages_request** | [**GameCenterEnabledVersionCompatibleVersionsLinkagesRequest**](GameCenterEnabledVersionCompatibleVersionsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_enabled_versions_compatible_versions_get_to_many_related

> models::GameCenterEnabledVersionsResponse game_center_enabled_versions_compatible_versions_get_to_many_related(id, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_version_string_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_game_center_enabled_versions_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include, limit_left_square_bracket_compatible_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_version_string_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'versionString' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_game_center_enabled_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterEnabledVersions |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_compatible_versions_right_square_bracket** | Option<**i32**> | maximum number of related compatibleVersions returned (when they are included) |  |

### Return type

[**models::GameCenterEnabledVersionsResponse**](GameCenterEnabledVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_enabled_versions_compatible_versions_get_to_many_relationship

> models::GameCenterEnabledVersionCompatibleVersionsLinkagesResponse game_center_enabled_versions_compatible_versions_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterEnabledVersionCompatibleVersionsLinkagesResponse**](GameCenterEnabledVersionCompatibleVersionsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_enabled_versions_compatible_versions_replace_to_many_relationship

> game_center_enabled_versions_compatible_versions_replace_to_many_relationship(id, game_center_enabled_version_compatible_versions_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_enabled_version_compatible_versions_linkages_request** | [**GameCenterEnabledVersionCompatibleVersionsLinkagesRequest**](GameCenterEnabledVersionCompatibleVersionsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


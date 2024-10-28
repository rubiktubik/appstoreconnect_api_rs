# \AppScreenshotSetsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_screenshot_sets_app_screenshots_get_to_many_related**](AppScreenshotSetsApi.md#app_screenshot_sets_app_screenshots_get_to_many_related) | **GET** /v1/appScreenshotSets/{id}/appScreenshots | 
[**app_screenshot_sets_app_screenshots_get_to_many_relationship**](AppScreenshotSetsApi.md#app_screenshot_sets_app_screenshots_get_to_many_relationship) | **GET** /v1/appScreenshotSets/{id}/relationships/appScreenshots | 
[**app_screenshot_sets_app_screenshots_replace_to_many_relationship**](AppScreenshotSetsApi.md#app_screenshot_sets_app_screenshots_replace_to_many_relationship) | **PATCH** /v1/appScreenshotSets/{id}/relationships/appScreenshots | 
[**app_screenshot_sets_create_instance**](AppScreenshotSetsApi.md#app_screenshot_sets_create_instance) | **POST** /v1/appScreenshotSets | 
[**app_screenshot_sets_delete_instance**](AppScreenshotSetsApi.md#app_screenshot_sets_delete_instance) | **DELETE** /v1/appScreenshotSets/{id} | 
[**app_screenshot_sets_get_instance**](AppScreenshotSetsApi.md#app_screenshot_sets_get_instance) | **GET** /v1/appScreenshotSets/{id} | 



## app_screenshot_sets_app_screenshots_get_to_many_related

> models::AppScreenshotsResponse app_screenshot_sets_app_screenshots_get_to_many_related(id, fields_left_square_bracket_app_screenshots_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshots |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppScreenshotsResponse**](AppScreenshotsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshot_sets_app_screenshots_get_to_many_relationship

> models::AppScreenshotSetAppScreenshotsLinkagesResponse app_screenshot_sets_app_screenshots_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AppScreenshotSetAppScreenshotsLinkagesResponse**](AppScreenshotSetAppScreenshotsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshot_sets_app_screenshots_replace_to_many_relationship

> app_screenshot_sets_app_screenshots_replace_to_many_relationship(id, app_screenshot_set_app_screenshots_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_screenshot_set_app_screenshots_linkages_request** | [**AppScreenshotSetAppScreenshotsLinkagesRequest**](AppScreenshotSetAppScreenshotsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshot_sets_create_instance

> models::AppScreenshotSetResponse app_screenshot_sets_create_instance(app_screenshot_set_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_screenshot_set_create_request** | [**AppScreenshotSetCreateRequest**](AppScreenshotSetCreateRequest.md) | AppScreenshotSet representation | [required] |

### Return type

[**models::AppScreenshotSetResponse**](AppScreenshotSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshot_sets_delete_instance

> app_screenshot_sets_delete_instance(id)


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


## app_screenshot_sets_get_instance

> models::AppScreenshotSetResponse app_screenshot_sets_get_instance(id, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_screenshots_right_square_bracket, include, limit_left_square_bracket_app_screenshots_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshots |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_screenshots_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshots returned (when they are included) |  |

### Return type

[**models::AppScreenshotSetResponse**](AppScreenshotSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


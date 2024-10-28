# \AppScreenshotsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_screenshots_create_instance**](AppScreenshotsApi.md#app_screenshots_create_instance) | **POST** /v1/appScreenshots | 
[**app_screenshots_delete_instance**](AppScreenshotsApi.md#app_screenshots_delete_instance) | **DELETE** /v1/appScreenshots/{id} | 
[**app_screenshots_get_instance**](AppScreenshotsApi.md#app_screenshots_get_instance) | **GET** /v1/appScreenshots/{id} | 
[**app_screenshots_update_instance**](AppScreenshotsApi.md#app_screenshots_update_instance) | **PATCH** /v1/appScreenshots/{id} | 



## app_screenshots_create_instance

> models::AppScreenshotResponse app_screenshots_create_instance(app_screenshot_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_screenshot_create_request** | [**AppScreenshotCreateRequest**](AppScreenshotCreateRequest.md) | AppScreenshot representation | [required] |

### Return type

[**models::AppScreenshotResponse**](AppScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshots_delete_instance

> app_screenshots_delete_instance(id)


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


## app_screenshots_get_instance

> models::AppScreenshotResponse app_screenshots_get_instance(id, fields_left_square_bracket_app_screenshots_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshots |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppScreenshotResponse**](AppScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_screenshots_update_instance

> models::AppScreenshotResponse app_screenshots_update_instance(id, app_screenshot_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_screenshot_update_request** | [**AppScreenshotUpdateRequest**](AppScreenshotUpdateRequest.md) | AppScreenshot representation | [required] |

### Return type

[**models::AppScreenshotResponse**](AppScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


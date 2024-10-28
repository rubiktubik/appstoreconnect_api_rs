# \AppPreviewsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_previews_create_instance**](AppPreviewsApi.md#app_previews_create_instance) | **POST** /v1/appPreviews | 
[**app_previews_delete_instance**](AppPreviewsApi.md#app_previews_delete_instance) | **DELETE** /v1/appPreviews/{id} | 
[**app_previews_get_instance**](AppPreviewsApi.md#app_previews_get_instance) | **GET** /v1/appPreviews/{id} | 
[**app_previews_update_instance**](AppPreviewsApi.md#app_previews_update_instance) | **PATCH** /v1/appPreviews/{id} | 



## app_previews_create_instance

> models::AppPreviewResponse app_previews_create_instance(app_preview_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_preview_create_request** | [**AppPreviewCreateRequest**](AppPreviewCreateRequest.md) | AppPreview representation | [required] |

### Return type

[**models::AppPreviewResponse**](AppPreviewResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_previews_delete_instance

> app_previews_delete_instance(id)


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


## app_previews_get_instance

> models::AppPreviewResponse app_previews_get_instance(id, fields_left_square_bracket_app_previews_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_previews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviews |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPreviewResponse**](AppPreviewResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_previews_update_instance

> models::AppPreviewResponse app_previews_update_instance(id, app_preview_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_preview_update_request** | [**AppPreviewUpdateRequest**](AppPreviewUpdateRequest.md) | AppPreview representation | [required] |

### Return type

[**models::AppPreviewResponse**](AppPreviewResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


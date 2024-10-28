# \AppClipHeaderImagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clip_header_images_create_instance**](AppClipHeaderImagesApi.md#app_clip_header_images_create_instance) | **POST** /v1/appClipHeaderImages | 
[**app_clip_header_images_delete_instance**](AppClipHeaderImagesApi.md#app_clip_header_images_delete_instance) | **DELETE** /v1/appClipHeaderImages/{id} | 
[**app_clip_header_images_get_instance**](AppClipHeaderImagesApi.md#app_clip_header_images_get_instance) | **GET** /v1/appClipHeaderImages/{id} | 
[**app_clip_header_images_update_instance**](AppClipHeaderImagesApi.md#app_clip_header_images_update_instance) | **PATCH** /v1/appClipHeaderImages/{id} | 



## app_clip_header_images_create_instance

> models::AppClipHeaderImageResponse app_clip_header_images_create_instance(app_clip_header_image_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_clip_header_image_create_request** | [**AppClipHeaderImageCreateRequest**](AppClipHeaderImageCreateRequest.md) | AppClipHeaderImage representation | [required] |

### Return type

[**models::AppClipHeaderImageResponse**](AppClipHeaderImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_header_images_delete_instance

> app_clip_header_images_delete_instance(id)


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


## app_clip_header_images_get_instance

> models::AppClipHeaderImageResponse app_clip_header_images_get_instance(id, fields_left_square_bracket_app_clip_header_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_header_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipHeaderImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipHeaderImageResponse**](AppClipHeaderImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_header_images_update_instance

> models::AppClipHeaderImageResponse app_clip_header_images_update_instance(id, app_clip_header_image_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_header_image_update_request** | [**AppClipHeaderImageUpdateRequest**](AppClipHeaderImageUpdateRequest.md) | AppClipHeaderImage representation | [required] |

### Return type

[**models::AppClipHeaderImageResponse**](AppClipHeaderImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


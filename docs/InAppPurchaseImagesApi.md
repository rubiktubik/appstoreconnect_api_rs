# \InAppPurchaseImagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchase_images_create_instance**](InAppPurchaseImagesApi.md#in_app_purchase_images_create_instance) | **POST** /v1/inAppPurchaseImages | 
[**in_app_purchase_images_delete_instance**](InAppPurchaseImagesApi.md#in_app_purchase_images_delete_instance) | **DELETE** /v1/inAppPurchaseImages/{id} | 
[**in_app_purchase_images_get_instance**](InAppPurchaseImagesApi.md#in_app_purchase_images_get_instance) | **GET** /v1/inAppPurchaseImages/{id} | 
[**in_app_purchase_images_update_instance**](InAppPurchaseImagesApi.md#in_app_purchase_images_update_instance) | **PATCH** /v1/inAppPurchaseImages/{id} | 



## in_app_purchase_images_create_instance

> models::InAppPurchaseImageResponse in_app_purchase_images_create_instance(in_app_purchase_image_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_image_create_request** | [**InAppPurchaseImageCreateRequest**](InAppPurchaseImageCreateRequest.md) | InAppPurchaseImage representation | [required] |

### Return type

[**models::InAppPurchaseImageResponse**](InAppPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_images_delete_instance

> in_app_purchase_images_delete_instance(id)


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


## in_app_purchase_images_get_instance

> models::InAppPurchaseImageResponse in_app_purchase_images_get_instance(id, fields_left_square_bracket_in_app_purchase_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseImageResponse**](InAppPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_images_update_instance

> models::InAppPurchaseImageResponse in_app_purchase_images_update_instance(id, in_app_purchase_image_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**in_app_purchase_image_update_request** | [**InAppPurchaseImageUpdateRequest**](InAppPurchaseImageUpdateRequest.md) | InAppPurchaseImage representation | [required] |

### Return type

[**models::InAppPurchaseImageResponse**](InAppPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


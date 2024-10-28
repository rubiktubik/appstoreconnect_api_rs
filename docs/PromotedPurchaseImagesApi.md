# \PromotedPurchaseImagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**promoted_purchase_images_create_instance**](PromotedPurchaseImagesApi.md#promoted_purchase_images_create_instance) | **POST** /v1/promotedPurchaseImages | 
[**promoted_purchase_images_delete_instance**](PromotedPurchaseImagesApi.md#promoted_purchase_images_delete_instance) | **DELETE** /v1/promotedPurchaseImages/{id} | 
[**promoted_purchase_images_get_instance**](PromotedPurchaseImagesApi.md#promoted_purchase_images_get_instance) | **GET** /v1/promotedPurchaseImages/{id} | 
[**promoted_purchase_images_update_instance**](PromotedPurchaseImagesApi.md#promoted_purchase_images_update_instance) | **PATCH** /v1/promotedPurchaseImages/{id} | 



## promoted_purchase_images_create_instance

> models::PromotedPurchaseImageResponse promoted_purchase_images_create_instance(promoted_purchase_image_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**promoted_purchase_image_create_request** | [**PromotedPurchaseImageCreateRequest**](PromotedPurchaseImageCreateRequest.md) | PromotedPurchaseImage representation | [required] |

### Return type

[**models::PromotedPurchaseImageResponse**](PromotedPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promoted_purchase_images_delete_instance

> promoted_purchase_images_delete_instance(id)


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


## promoted_purchase_images_get_instance

> models::PromotedPurchaseImageResponse promoted_purchase_images_get_instance(id, fields_left_square_bracket_promoted_purchase_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_promoted_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchaseImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::PromotedPurchaseImageResponse**](PromotedPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promoted_purchase_images_update_instance

> models::PromotedPurchaseImageResponse promoted_purchase_images_update_instance(id, promoted_purchase_image_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**promoted_purchase_image_update_request** | [**PromotedPurchaseImageUpdateRequest**](PromotedPurchaseImageUpdateRequest.md) | PromotedPurchaseImage representation | [required] |

### Return type

[**models::PromotedPurchaseImageResponse**](PromotedPurchaseImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


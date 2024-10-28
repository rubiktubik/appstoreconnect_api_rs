# \PromotedPurchasesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**promoted_purchases_create_instance**](PromotedPurchasesApi.md#promoted_purchases_create_instance) | **POST** /v1/promotedPurchases | 
[**promoted_purchases_delete_instance**](PromotedPurchasesApi.md#promoted_purchases_delete_instance) | **DELETE** /v1/promotedPurchases/{id} | 
[**promoted_purchases_get_instance**](PromotedPurchasesApi.md#promoted_purchases_get_instance) | **GET** /v1/promotedPurchases/{id} | 
[**promoted_purchases_promotion_images_get_to_many_related**](PromotedPurchasesApi.md#promoted_purchases_promotion_images_get_to_many_related) | **GET** /v1/promotedPurchases/{id}/promotionImages | 
[**promoted_purchases_update_instance**](PromotedPurchasesApi.md#promoted_purchases_update_instance) | **PATCH** /v1/promotedPurchases/{id} | 



## promoted_purchases_create_instance

> models::PromotedPurchaseResponse promoted_purchases_create_instance(promoted_purchase_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**promoted_purchase_create_request** | [**PromotedPurchaseCreateRequest**](PromotedPurchaseCreateRequest.md) | PromotedPurchase representation | [required] |

### Return type

[**models::PromotedPurchaseResponse**](PromotedPurchaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promoted_purchases_delete_instance

> promoted_purchases_delete_instance(id)


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


## promoted_purchases_get_instance

> models::PromotedPurchaseResponse promoted_purchases_get_instance(id, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_promoted_purchase_images_right_square_bracket, include, limit_left_square_bracket_promotion_images_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_promoted_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchaseImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_promotion_images_right_square_bracket** | Option<**i32**> | maximum number of related promotionImages returned (when they are included) |  |

### Return type

[**models::PromotedPurchaseResponse**](PromotedPurchaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promoted_purchases_promotion_images_get_to_many_related

> models::PromotedPurchaseImagesResponse promoted_purchases_promotion_images_get_to_many_related(id, fields_left_square_bracket_promoted_purchase_images_right_square_bracket, fields_left_square_bracket_promoted_purchases_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_promoted_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchaseImages |  |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::PromotedPurchaseImagesResponse**](PromotedPurchaseImagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promoted_purchases_update_instance

> models::PromotedPurchaseResponse promoted_purchases_update_instance(id, promoted_purchase_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**promoted_purchase_update_request** | [**PromotedPurchaseUpdateRequest**](PromotedPurchaseUpdateRequest.md) | PromotedPurchase representation | [required] |

### Return type

[**models::PromotedPurchaseResponse**](PromotedPurchaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


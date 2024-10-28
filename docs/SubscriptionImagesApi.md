# \SubscriptionImagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_images_create_instance**](SubscriptionImagesApi.md#subscription_images_create_instance) | **POST** /v1/subscriptionImages | 
[**subscription_images_delete_instance**](SubscriptionImagesApi.md#subscription_images_delete_instance) | **DELETE** /v1/subscriptionImages/{id} | 
[**subscription_images_get_instance**](SubscriptionImagesApi.md#subscription_images_get_instance) | **GET** /v1/subscriptionImages/{id} | 
[**subscription_images_update_instance**](SubscriptionImagesApi.md#subscription_images_update_instance) | **PATCH** /v1/subscriptionImages/{id} | 



## subscription_images_create_instance

> models::SubscriptionImageResponse subscription_images_create_instance(subscription_image_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_image_create_request** | [**SubscriptionImageCreateRequest**](SubscriptionImageCreateRequest.md) | SubscriptionImage representation | [required] |

### Return type

[**models::SubscriptionImageResponse**](SubscriptionImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_images_delete_instance

> subscription_images_delete_instance(id)


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


## subscription_images_get_instance

> models::SubscriptionImageResponse subscription_images_get_instance(id, fields_left_square_bracket_subscription_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionImageResponse**](SubscriptionImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_images_update_instance

> models::SubscriptionImageResponse subscription_images_update_instance(id, subscription_image_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_image_update_request** | [**SubscriptionImageUpdateRequest**](SubscriptionImageUpdateRequest.md) | SubscriptionImage representation | [required] |

### Return type

[**models::SubscriptionImageResponse**](SubscriptionImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


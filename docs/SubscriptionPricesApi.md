# \SubscriptionPricesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_prices_create_instance**](SubscriptionPricesApi.md#subscription_prices_create_instance) | **POST** /v1/subscriptionPrices | 
[**subscription_prices_delete_instance**](SubscriptionPricesApi.md#subscription_prices_delete_instance) | **DELETE** /v1/subscriptionPrices/{id} | 



## subscription_prices_create_instance

> models::SubscriptionPriceResponse subscription_prices_create_instance(subscription_price_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_price_create_request** | [**SubscriptionPriceCreateRequest**](SubscriptionPriceCreateRequest.md) | SubscriptionPrice representation | [required] |

### Return type

[**models::SubscriptionPriceResponse**](SubscriptionPriceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_prices_delete_instance

> subscription_prices_delete_instance(id)


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


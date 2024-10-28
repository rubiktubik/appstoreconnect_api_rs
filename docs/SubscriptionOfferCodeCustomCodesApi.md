# \SubscriptionOfferCodeCustomCodesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_offer_code_custom_codes_create_instance**](SubscriptionOfferCodeCustomCodesApi.md#subscription_offer_code_custom_codes_create_instance) | **POST** /v1/subscriptionOfferCodeCustomCodes | 
[**subscription_offer_code_custom_codes_get_instance**](SubscriptionOfferCodeCustomCodesApi.md#subscription_offer_code_custom_codes_get_instance) | **GET** /v1/subscriptionOfferCodeCustomCodes/{id} | 
[**subscription_offer_code_custom_codes_update_instance**](SubscriptionOfferCodeCustomCodesApi.md#subscription_offer_code_custom_codes_update_instance) | **PATCH** /v1/subscriptionOfferCodeCustomCodes/{id} | 



## subscription_offer_code_custom_codes_create_instance

> models::SubscriptionOfferCodeCustomCodeResponse subscription_offer_code_custom_codes_create_instance(subscription_offer_code_custom_code_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_offer_code_custom_code_create_request** | [**SubscriptionOfferCodeCustomCodeCreateRequest**](SubscriptionOfferCodeCustomCodeCreateRequest.md) | SubscriptionOfferCodeCustomCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeCustomCodeResponse**](SubscriptionOfferCodeCustomCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_code_custom_codes_get_instance

> models::SubscriptionOfferCodeCustomCodeResponse subscription_offer_code_custom_codes_get_instance(id, fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeCustomCodes |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionOfferCodeCustomCodeResponse**](SubscriptionOfferCodeCustomCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_code_custom_codes_update_instance

> models::SubscriptionOfferCodeCustomCodeResponse subscription_offer_code_custom_codes_update_instance(id, subscription_offer_code_custom_code_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_offer_code_custom_code_update_request** | [**SubscriptionOfferCodeCustomCodeUpdateRequest**](SubscriptionOfferCodeCustomCodeUpdateRequest.md) | SubscriptionOfferCodeCustomCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeCustomCodeResponse**](SubscriptionOfferCodeCustomCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


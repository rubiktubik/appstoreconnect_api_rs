# \SubscriptionOfferCodeOneTimeUseCodesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_offer_code_one_time_use_codes_create_instance**](SubscriptionOfferCodeOneTimeUseCodesApi.md#subscription_offer_code_one_time_use_codes_create_instance) | **POST** /v1/subscriptionOfferCodeOneTimeUseCodes | 
[**subscription_offer_code_one_time_use_codes_get_instance**](SubscriptionOfferCodeOneTimeUseCodesApi.md#subscription_offer_code_one_time_use_codes_get_instance) | **GET** /v1/subscriptionOfferCodeOneTimeUseCodes/{id} | 
[**subscription_offer_code_one_time_use_codes_update_instance**](SubscriptionOfferCodeOneTimeUseCodesApi.md#subscription_offer_code_one_time_use_codes_update_instance) | **PATCH** /v1/subscriptionOfferCodeOneTimeUseCodes/{id} | 
[**subscription_offer_code_one_time_use_codes_values_get_to_one_related**](SubscriptionOfferCodeOneTimeUseCodesApi.md#subscription_offer_code_one_time_use_codes_values_get_to_one_related) | **GET** /v1/subscriptionOfferCodeOneTimeUseCodes/{id}/values | 



## subscription_offer_code_one_time_use_codes_create_instance

> models::SubscriptionOfferCodeOneTimeUseCodeResponse subscription_offer_code_one_time_use_codes_create_instance(subscription_offer_code_one_time_use_code_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_offer_code_one_time_use_code_create_request** | [**SubscriptionOfferCodeOneTimeUseCodeCreateRequest**](SubscriptionOfferCodeOneTimeUseCodeCreateRequest.md) | SubscriptionOfferCodeOneTimeUseCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeOneTimeUseCodeResponse**](SubscriptionOfferCodeOneTimeUseCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_code_one_time_use_codes_get_instance

> models::SubscriptionOfferCodeOneTimeUseCodeResponse subscription_offer_code_one_time_use_codes_get_instance(id, fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeOneTimeUseCodes |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionOfferCodeOneTimeUseCodeResponse**](SubscriptionOfferCodeOneTimeUseCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_code_one_time_use_codes_update_instance

> models::SubscriptionOfferCodeOneTimeUseCodeResponse subscription_offer_code_one_time_use_codes_update_instance(id, subscription_offer_code_one_time_use_code_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_offer_code_one_time_use_code_update_request** | [**SubscriptionOfferCodeOneTimeUseCodeUpdateRequest**](SubscriptionOfferCodeOneTimeUseCodeUpdateRequest.md) | SubscriptionOfferCodeOneTimeUseCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeOneTimeUseCodeResponse**](SubscriptionOfferCodeOneTimeUseCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_code_one_time_use_codes_values_get_to_one_related

> String subscription_offer_code_one_time_use_codes_values_get_to_one_related(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

**String**

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


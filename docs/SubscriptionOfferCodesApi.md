# \SubscriptionOfferCodesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_offer_codes_create_instance**](SubscriptionOfferCodesApi.md#subscription_offer_codes_create_instance) | **POST** /v1/subscriptionOfferCodes | 
[**subscription_offer_codes_custom_codes_get_to_many_related**](SubscriptionOfferCodesApi.md#subscription_offer_codes_custom_codes_get_to_many_related) | **GET** /v1/subscriptionOfferCodes/{id}/customCodes | 
[**subscription_offer_codes_get_instance**](SubscriptionOfferCodesApi.md#subscription_offer_codes_get_instance) | **GET** /v1/subscriptionOfferCodes/{id} | 
[**subscription_offer_codes_one_time_use_codes_get_to_many_related**](SubscriptionOfferCodesApi.md#subscription_offer_codes_one_time_use_codes_get_to_many_related) | **GET** /v1/subscriptionOfferCodes/{id}/oneTimeUseCodes | 
[**subscription_offer_codes_prices_get_to_many_related**](SubscriptionOfferCodesApi.md#subscription_offer_codes_prices_get_to_many_related) | **GET** /v1/subscriptionOfferCodes/{id}/prices | 
[**subscription_offer_codes_update_instance**](SubscriptionOfferCodesApi.md#subscription_offer_codes_update_instance) | **PATCH** /v1/subscriptionOfferCodes/{id} | 



## subscription_offer_codes_create_instance

> models::SubscriptionOfferCodeResponse subscription_offer_codes_create_instance(subscription_offer_code_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_offer_code_create_request** | [**SubscriptionOfferCodeCreateRequest**](SubscriptionOfferCodeCreateRequest.md) | SubscriptionOfferCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeResponse**](SubscriptionOfferCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_codes_custom_codes_get_to_many_related

> models::SubscriptionOfferCodeCustomCodesResponse subscription_offer_codes_custom_codes_get_to_many_related(id, fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeCustomCodes |  |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionOfferCodeCustomCodesResponse**](SubscriptionOfferCodeCustomCodesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_codes_get_instance

> models::SubscriptionOfferCodeResponse subscription_offer_codes_get_instance(id, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket, include, limit_left_square_bracket_custom_codes_right_square_bracket, limit_left_square_bracket_one_time_use_codes_right_square_bracket, limit_left_square_bracket_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeOneTimeUseCodes |  |
**fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeCustomCodes |  |
**fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodePrices |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_custom_codes_right_square_bracket** | Option<**i32**> | maximum number of related customCodes returned (when they are included) |  |
**limit_left_square_bracket_one_time_use_codes_right_square_bracket** | Option<**i32**> | maximum number of related oneTimeUseCodes returned (when they are included) |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |

### Return type

[**models::SubscriptionOfferCodeResponse**](SubscriptionOfferCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_codes_one_time_use_codes_get_to_many_related

> models::SubscriptionOfferCodeOneTimeUseCodesResponse subscription_offer_codes_one_time_use_codes_get_to_many_related(id, fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeOneTimeUseCodes |  |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionOfferCodeOneTimeUseCodesResponse**](SubscriptionOfferCodeOneTimeUseCodesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_codes_prices_get_to_many_related

> models::SubscriptionOfferCodePricesResponse subscription_offer_codes_prices_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodePrices |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionOfferCodePricesResponse**](SubscriptionOfferCodePricesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_offer_codes_update_instance

> models::SubscriptionOfferCodeResponse subscription_offer_codes_update_instance(id, subscription_offer_code_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_offer_code_update_request** | [**SubscriptionOfferCodeUpdateRequest**](SubscriptionOfferCodeUpdateRequest.md) | SubscriptionOfferCode representation | [required] |

### Return type

[**models::SubscriptionOfferCodeResponse**](SubscriptionOfferCodeResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


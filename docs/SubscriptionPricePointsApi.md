# \SubscriptionPricePointsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_price_points_equalizations_get_to_many_related**](SubscriptionPricePointsApi.md#subscription_price_points_equalizations_get_to_many_related) | **GET** /v1/subscriptionPricePoints/{id}/equalizations | 
[**subscription_price_points_get_instance**](SubscriptionPricePointsApi.md#subscription_price_points_get_instance) | **GET** /v1/subscriptionPricePoints/{id} | 



## subscription_price_points_equalizations_get_to_many_related

> models::SubscriptionPricePointsResponse subscription_price_points_equalizations_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, filter_left_square_bracket_subscription_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**filter_left_square_bracket_subscription_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'subscription' |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionPricePointsResponse**](SubscriptionPricePointsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_price_points_get_instance

> models::SubscriptionPricePointResponse subscription_price_points_get_instance(id, fields_left_square_bracket_subscription_price_points_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionPricePointResponse**](SubscriptionPricePointResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \SubscriptionGracePeriodsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_grace_periods_get_instance**](SubscriptionGracePeriodsApi.md#subscription_grace_periods_get_instance) | **GET** /v1/subscriptionGracePeriods/{id} | 
[**subscription_grace_periods_update_instance**](SubscriptionGracePeriodsApi.md#subscription_grace_periods_update_instance) | **PATCH** /v1/subscriptionGracePeriods/{id} | 



## subscription_grace_periods_get_instance

> models::SubscriptionGracePeriodResponse subscription_grace_periods_get_instance(id, fields_left_square_bracket_subscription_grace_periods_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_grace_periods_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGracePeriods |  |

### Return type

[**models::SubscriptionGracePeriodResponse**](SubscriptionGracePeriodResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_grace_periods_update_instance

> models::SubscriptionGracePeriodResponse subscription_grace_periods_update_instance(id, subscription_grace_period_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_grace_period_update_request** | [**SubscriptionGracePeriodUpdateRequest**](SubscriptionGracePeriodUpdateRequest.md) | SubscriptionGracePeriod representation | [required] |

### Return type

[**models::SubscriptionGracePeriodResponse**](SubscriptionGracePeriodResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


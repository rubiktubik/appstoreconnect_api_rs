# \SubscriptionGroupLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_group_localizations_create_instance**](SubscriptionGroupLocalizationsApi.md#subscription_group_localizations_create_instance) | **POST** /v1/subscriptionGroupLocalizations | 
[**subscription_group_localizations_delete_instance**](SubscriptionGroupLocalizationsApi.md#subscription_group_localizations_delete_instance) | **DELETE** /v1/subscriptionGroupLocalizations/{id} | 
[**subscription_group_localizations_get_instance**](SubscriptionGroupLocalizationsApi.md#subscription_group_localizations_get_instance) | **GET** /v1/subscriptionGroupLocalizations/{id} | 
[**subscription_group_localizations_update_instance**](SubscriptionGroupLocalizationsApi.md#subscription_group_localizations_update_instance) | **PATCH** /v1/subscriptionGroupLocalizations/{id} | 



## subscription_group_localizations_create_instance

> models::SubscriptionGroupLocalizationResponse subscription_group_localizations_create_instance(subscription_group_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_group_localization_create_request** | [**SubscriptionGroupLocalizationCreateRequest**](SubscriptionGroupLocalizationCreateRequest.md) | SubscriptionGroupLocalization representation | [required] |

### Return type

[**models::SubscriptionGroupLocalizationResponse**](SubscriptionGroupLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_group_localizations_delete_instance

> subscription_group_localizations_delete_instance(id)


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


## subscription_group_localizations_get_instance

> models::SubscriptionGroupLocalizationResponse subscription_group_localizations_get_instance(id, fields_left_square_bracket_subscription_group_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_group_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroupLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionGroupLocalizationResponse**](SubscriptionGroupLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_group_localizations_update_instance

> models::SubscriptionGroupLocalizationResponse subscription_group_localizations_update_instance(id, subscription_group_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_group_localization_update_request** | [**SubscriptionGroupLocalizationUpdateRequest**](SubscriptionGroupLocalizationUpdateRequest.md) | SubscriptionGroupLocalization representation | [required] |

### Return type

[**models::SubscriptionGroupLocalizationResponse**](SubscriptionGroupLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


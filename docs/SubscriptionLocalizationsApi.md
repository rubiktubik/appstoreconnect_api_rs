# \SubscriptionLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_localizations_create_instance**](SubscriptionLocalizationsApi.md#subscription_localizations_create_instance) | **POST** /v1/subscriptionLocalizations | 
[**subscription_localizations_delete_instance**](SubscriptionLocalizationsApi.md#subscription_localizations_delete_instance) | **DELETE** /v1/subscriptionLocalizations/{id} | 
[**subscription_localizations_get_instance**](SubscriptionLocalizationsApi.md#subscription_localizations_get_instance) | **GET** /v1/subscriptionLocalizations/{id} | 
[**subscription_localizations_update_instance**](SubscriptionLocalizationsApi.md#subscription_localizations_update_instance) | **PATCH** /v1/subscriptionLocalizations/{id} | 



## subscription_localizations_create_instance

> models::SubscriptionLocalizationResponse subscription_localizations_create_instance(subscription_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_localization_create_request** | [**SubscriptionLocalizationCreateRequest**](SubscriptionLocalizationCreateRequest.md) | SubscriptionLocalization representation | [required] |

### Return type

[**models::SubscriptionLocalizationResponse**](SubscriptionLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_localizations_delete_instance

> subscription_localizations_delete_instance(id)


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


## subscription_localizations_get_instance

> models::SubscriptionLocalizationResponse subscription_localizations_get_instance(id, fields_left_square_bracket_subscription_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionLocalizationResponse**](SubscriptionLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_localizations_update_instance

> models::SubscriptionLocalizationResponse subscription_localizations_update_instance(id, subscription_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_localization_update_request** | [**SubscriptionLocalizationUpdateRequest**](SubscriptionLocalizationUpdateRequest.md) | SubscriptionLocalization representation | [required] |

### Return type

[**models::SubscriptionLocalizationResponse**](SubscriptionLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


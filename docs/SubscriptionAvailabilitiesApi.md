# \SubscriptionAvailabilitiesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_availabilities_available_territories_get_to_many_related**](SubscriptionAvailabilitiesApi.md#subscription_availabilities_available_territories_get_to_many_related) | **GET** /v1/subscriptionAvailabilities/{id}/availableTerritories | 
[**subscription_availabilities_create_instance**](SubscriptionAvailabilitiesApi.md#subscription_availabilities_create_instance) | **POST** /v1/subscriptionAvailabilities | 
[**subscription_availabilities_get_instance**](SubscriptionAvailabilitiesApi.md#subscription_availabilities_get_instance) | **GET** /v1/subscriptionAvailabilities/{id} | 



## subscription_availabilities_available_territories_get_to_many_related

> models::TerritoriesResponse subscription_availabilities_available_territories_get_to_many_related(id, fields_left_square_bracket_territories_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::TerritoriesResponse**](TerritoriesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_availabilities_create_instance

> models::SubscriptionAvailabilityResponse subscription_availabilities_create_instance(subscription_availability_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_availability_create_request** | [**SubscriptionAvailabilityCreateRequest**](SubscriptionAvailabilityCreateRequest.md) | SubscriptionAvailability representation | [required] |

### Return type

[**models::SubscriptionAvailabilityResponse**](SubscriptionAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_availabilities_get_instance

> models::SubscriptionAvailabilityResponse subscription_availabilities_get_instance(id, fields_left_square_bracket_subscription_availabilities_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_available_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAvailabilities |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_available_territories_right_square_bracket** | Option<**i32**> | maximum number of related availableTerritories returned (when they are included) |  |

### Return type

[**models::SubscriptionAvailabilityResponse**](SubscriptionAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


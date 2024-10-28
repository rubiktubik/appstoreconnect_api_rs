# \InAppPurchaseAvailabilitiesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchase_availabilities_available_territories_get_to_many_related**](InAppPurchaseAvailabilitiesApi.md#in_app_purchase_availabilities_available_territories_get_to_many_related) | **GET** /v1/inAppPurchaseAvailabilities/{id}/availableTerritories | 
[**in_app_purchase_availabilities_create_instance**](InAppPurchaseAvailabilitiesApi.md#in_app_purchase_availabilities_create_instance) | **POST** /v1/inAppPurchaseAvailabilities | 
[**in_app_purchase_availabilities_get_instance**](InAppPurchaseAvailabilitiesApi.md#in_app_purchase_availabilities_get_instance) | **GET** /v1/inAppPurchaseAvailabilities/{id} | 



## in_app_purchase_availabilities_available_territories_get_to_many_related

> models::TerritoriesResponse in_app_purchase_availabilities_available_territories_get_to_many_related(id, fields_left_square_bracket_territories_right_square_bracket, limit)


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


## in_app_purchase_availabilities_create_instance

> models::InAppPurchaseAvailabilityResponse in_app_purchase_availabilities_create_instance(in_app_purchase_availability_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_availability_create_request** | [**InAppPurchaseAvailabilityCreateRequest**](InAppPurchaseAvailabilityCreateRequest.md) | InAppPurchaseAvailability representation | [required] |

### Return type

[**models::InAppPurchaseAvailabilityResponse**](InAppPurchaseAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_availabilities_get_instance

> models::InAppPurchaseAvailabilityResponse in_app_purchase_availabilities_get_instance(id, fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_available_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAvailabilities |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_available_territories_right_square_bracket** | Option<**i32**> | maximum number of related availableTerritories returned (when they are included) |  |

### Return type

[**models::InAppPurchaseAvailabilityResponse**](InAppPurchaseAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


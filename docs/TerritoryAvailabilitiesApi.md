# \TerritoryAvailabilitiesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**territory_availabilities_update_instance**](TerritoryAvailabilitiesApi.md#territory_availabilities_update_instance) | **PATCH** /v1/territoryAvailabilities/{id} | 



## territory_availabilities_update_instance

> models::TerritoryAvailabilityResponse territory_availabilities_update_instance(id, territory_availability_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**territory_availability_update_request** | [**TerritoryAvailabilityUpdateRequest**](TerritoryAvailabilityUpdateRequest.md) | TerritoryAvailability representation | [required] |

### Return type

[**models::TerritoryAvailabilityResponse**](TerritoryAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


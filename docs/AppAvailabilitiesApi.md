# \AppAvailabilitiesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_availabilities_available_territories_get_to_many_related**](AppAvailabilitiesApi.md#app_availabilities_available_territories_get_to_many_related) | **GET** /v1/appAvailabilities/{id}/availableTerritories | 
[**app_availabilities_create_instance**](AppAvailabilitiesApi.md#app_availabilities_create_instance) | **POST** /v1/appAvailabilities | 
[**app_availabilities_get_instance**](AppAvailabilitiesApi.md#app_availabilities_get_instance) | **GET** /v1/appAvailabilities/{id} | 
[**app_availabilities_v2_create_instance**](AppAvailabilitiesApi.md#app_availabilities_v2_create_instance) | **POST** /v2/appAvailabilities | 
[**app_availabilities_v2_get_instance**](AppAvailabilitiesApi.md#app_availabilities_v2_get_instance) | **GET** /v2/appAvailabilities/{id} | 
[**app_availabilities_v2_territory_availabilities_get_to_many_related**](AppAvailabilitiesApi.md#app_availabilities_v2_territory_availabilities_get_to_many_related) | **GET** /v2/appAvailabilities/{id}/territoryAvailabilities | 



## app_availabilities_available_territories_get_to_many_related

> models::TerritoriesResponse app_availabilities_available_territories_get_to_many_related(id, fields_left_square_bracket_territories_right_square_bracket, limit)


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


## app_availabilities_create_instance

> models::AppAvailabilityResponse app_availabilities_create_instance(app_availability_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_availability_create_request** | [**AppAvailabilityCreateRequest**](AppAvailabilityCreateRequest.md) | AppAvailability representation | [required] |

### Return type

[**models::AppAvailabilityResponse**](AppAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_availabilities_get_instance

> models::AppAvailabilityResponse app_availabilities_get_instance(id, fields_left_square_bracket_app_availabilities_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_available_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appAvailabilities |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_available_territories_right_square_bracket** | Option<**i32**> | maximum number of related availableTerritories returned (when they are included) |  |

### Return type

[**models::AppAvailabilityResponse**](AppAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_availabilities_v2_create_instance

> models::AppAvailabilityV2Response app_availabilities_v2_create_instance(app_availability_v2_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_availability_v2_create_request** | [**AppAvailabilityV2CreateRequest**](AppAvailabilityV2CreateRequest.md) | AppAvailability representation | [required] |

### Return type

[**models::AppAvailabilityV2Response**](AppAvailabilityV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_availabilities_v2_get_instance

> models::AppAvailabilityV2Response app_availabilities_v2_get_instance(id, fields_left_square_bracket_app_availabilities_right_square_bracket, fields_left_square_bracket_territory_availabilities_right_square_bracket, include, limit_left_square_bracket_territory_availabilities_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appAvailabilities |  |
**fields_left_square_bracket_territory_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territoryAvailabilities |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_territory_availabilities_right_square_bracket** | Option<**i32**> | maximum number of related territoryAvailabilities returned (when they are included) |  |

### Return type

[**models::AppAvailabilityV2Response**](AppAvailabilityV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_availabilities_v2_territory_availabilities_get_to_many_related

> models::TerritoryAvailabilitiesResponse app_availabilities_v2_territory_availabilities_get_to_many_related(id, fields_left_square_bracket_territory_availabilities_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_territory_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territoryAvailabilities |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::TerritoryAvailabilitiesResponse**](TerritoryAvailabilitiesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


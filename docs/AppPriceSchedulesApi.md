# \AppPriceSchedulesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_price_schedules_automatic_prices_get_to_many_related**](AppPriceSchedulesApi.md#app_price_schedules_automatic_prices_get_to_many_related) | **GET** /v1/appPriceSchedules/{id}/automaticPrices | 
[**app_price_schedules_base_territory_get_to_one_related**](AppPriceSchedulesApi.md#app_price_schedules_base_territory_get_to_one_related) | **GET** /v1/appPriceSchedules/{id}/baseTerritory | 
[**app_price_schedules_create_instance**](AppPriceSchedulesApi.md#app_price_schedules_create_instance) | **POST** /v1/appPriceSchedules | 
[**app_price_schedules_get_instance**](AppPriceSchedulesApi.md#app_price_schedules_get_instance) | **GET** /v1/appPriceSchedules/{id} | 
[**app_price_schedules_manual_prices_get_to_many_related**](AppPriceSchedulesApi.md#app_price_schedules_manual_prices_get_to_many_related) | **GET** /v1/appPriceSchedules/{id}/manualPrices | 



## app_price_schedules_automatic_prices_get_to_many_related

> models::AppPricesV2Response app_price_schedules_automatic_prices_get_to_many_related(id, filter_left_square_bracket_start_date_right_square_bracket, filter_left_square_bracket_end_date_right_square_bracket, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_app_prices_right_square_bracket, fields_left_square_bracket_app_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_start_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'startDate' |  |
**filter_left_square_bracket_end_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'endDate' |  |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_app_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPrices |  |
**fields_left_square_bracket_app_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPricesV2Response**](AppPricesV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_price_schedules_base_territory_get_to_one_related

> models::TerritoryResponse app_price_schedules_base_territory_get_to_one_related(id, fields_left_square_bracket_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |

### Return type

[**models::TerritoryResponse**](TerritoryResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_price_schedules_create_instance

> models::AppPriceScheduleResponse app_price_schedules_create_instance(app_price_schedule_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_price_schedule_create_request** | [**AppPriceScheduleCreateRequest**](AppPriceScheduleCreateRequest.md) | AppPriceSchedule representation | [required] |

### Return type

[**models::AppPriceScheduleResponse**](AppPriceScheduleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_price_schedules_get_instance

> models::AppPriceScheduleResponse app_price_schedules_get_instance(id, fields_left_square_bracket_app_price_schedules_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_app_prices_right_square_bracket, include, limit_left_square_bracket_automatic_prices_right_square_bracket, limit_left_square_bracket_manual_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_price_schedules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPriceSchedules |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_app_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPrices |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_automatic_prices_right_square_bracket** | Option<**i32**> | maximum number of related automaticPrices returned (when they are included) |  |
**limit_left_square_bracket_manual_prices_right_square_bracket** | Option<**i32**> | maximum number of related manualPrices returned (when they are included) |  |

### Return type

[**models::AppPriceScheduleResponse**](AppPriceScheduleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_price_schedules_manual_prices_get_to_many_related

> models::AppPricesV2Response app_price_schedules_manual_prices_get_to_many_related(id, filter_left_square_bracket_start_date_right_square_bracket, filter_left_square_bracket_end_date_right_square_bracket, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_app_prices_right_square_bracket, fields_left_square_bracket_app_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_start_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'startDate' |  |
**filter_left_square_bracket_end_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'endDate' |  |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_app_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPrices |  |
**fields_left_square_bracket_app_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPricesV2Response**](AppPricesV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


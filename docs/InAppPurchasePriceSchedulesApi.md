# \InAppPurchasePriceSchedulesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchase_price_schedules_automatic_prices_get_to_many_related**](InAppPurchasePriceSchedulesApi.md#in_app_purchase_price_schedules_automatic_prices_get_to_many_related) | **GET** /v1/inAppPurchasePriceSchedules/{id}/automaticPrices | 
[**in_app_purchase_price_schedules_base_territory_get_to_one_related**](InAppPurchasePriceSchedulesApi.md#in_app_purchase_price_schedules_base_territory_get_to_one_related) | **GET** /v1/inAppPurchasePriceSchedules/{id}/baseTerritory | 
[**in_app_purchase_price_schedules_create_instance**](InAppPurchasePriceSchedulesApi.md#in_app_purchase_price_schedules_create_instance) | **POST** /v1/inAppPurchasePriceSchedules | 
[**in_app_purchase_price_schedules_get_instance**](InAppPurchasePriceSchedulesApi.md#in_app_purchase_price_schedules_get_instance) | **GET** /v1/inAppPurchasePriceSchedules/{id} | 
[**in_app_purchase_price_schedules_manual_prices_get_to_many_related**](InAppPurchasePriceSchedulesApi.md#in_app_purchase_price_schedules_manual_prices_get_to_many_related) | **GET** /v1/inAppPurchasePriceSchedules/{id}/manualPrices | 



## in_app_purchase_price_schedules_automatic_prices_get_to_many_related

> models::InAppPurchasePricesResponse in_app_purchase_price_schedules_automatic_prices_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_in_app_purchase_prices_right_square_bracket, fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_in_app_purchase_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePrices |  |
**fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchasePricesResponse**](InAppPurchasePricesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_price_schedules_base_territory_get_to_one_related

> models::TerritoryResponse in_app_purchase_price_schedules_base_territory_get_to_one_related(id, fields_left_square_bracket_territories_right_square_bracket)


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


## in_app_purchase_price_schedules_create_instance

> models::InAppPurchasePriceScheduleResponse in_app_purchase_price_schedules_create_instance(in_app_purchase_price_schedule_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_price_schedule_create_request** | [**InAppPurchasePriceScheduleCreateRequest**](InAppPurchasePriceScheduleCreateRequest.md) | InAppPurchasePriceSchedule representation | [required] |

### Return type

[**models::InAppPurchasePriceScheduleResponse**](InAppPurchasePriceScheduleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_price_schedules_get_instance

> models::InAppPurchasePriceScheduleResponse in_app_purchase_price_schedules_get_instance(id, fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_in_app_purchase_prices_right_square_bracket, include, limit_left_square_bracket_automatic_prices_right_square_bracket, limit_left_square_bracket_manual_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePriceSchedules |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_in_app_purchase_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePrices |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_automatic_prices_right_square_bracket** | Option<**i32**> | maximum number of related automaticPrices returned (when they are included) |  |
**limit_left_square_bracket_manual_prices_right_square_bracket** | Option<**i32**> | maximum number of related manualPrices returned (when they are included) |  |

### Return type

[**models::InAppPurchasePriceScheduleResponse**](InAppPurchasePriceScheduleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_price_schedules_manual_prices_get_to_many_related

> models::InAppPurchasePricesResponse in_app_purchase_price_schedules_manual_prices_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_in_app_purchase_prices_right_square_bracket, fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_in_app_purchase_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePrices |  |
**fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchasePricesResponse**](InAppPurchasePricesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


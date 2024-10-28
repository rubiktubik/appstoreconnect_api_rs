# \AppPricePointsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_price_points_v3_equalizations_get_to_many_related**](AppPricePointsApi.md#app_price_points_v3_equalizations_get_to_many_related) | **GET** /v3/appPricePoints/{id}/equalizations | 
[**app_price_points_v3_get_instance**](AppPricePointsApi.md#app_price_points_v3_get_instance) | **GET** /v3/appPricePoints/{id} | 



## app_price_points_v3_equalizations_get_to_many_related

> models::AppPricePointsV3Response app_price_points_v3_equalizations_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_app_price_points_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_app_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPricePoints |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPricePointsV3Response**](AppPricePointsV3Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_price_points_v3_get_instance

> models::AppPricePointV3Response app_price_points_v3_get_instance(id, fields_left_square_bracket_app_price_points_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPricePoints |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPricePointV3Response**](AppPricePointV3Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

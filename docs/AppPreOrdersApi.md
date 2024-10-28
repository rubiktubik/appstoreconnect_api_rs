# \AppPreOrdersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_pre_orders_create_instance**](AppPreOrdersApi.md#app_pre_orders_create_instance) | **POST** /v1/appPreOrders | 
[**app_pre_orders_delete_instance**](AppPreOrdersApi.md#app_pre_orders_delete_instance) | **DELETE** /v1/appPreOrders/{id} | 
[**app_pre_orders_get_instance**](AppPreOrdersApi.md#app_pre_orders_get_instance) | **GET** /v1/appPreOrders/{id} | 
[**app_pre_orders_update_instance**](AppPreOrdersApi.md#app_pre_orders_update_instance) | **PATCH** /v1/appPreOrders/{id} | 



## app_pre_orders_create_instance

> models::AppPreOrderResponse app_pre_orders_create_instance(app_pre_order_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_pre_order_create_request** | [**AppPreOrderCreateRequest**](AppPreOrderCreateRequest.md) | AppPreOrder representation | [required] |

### Return type

[**models::AppPreOrderResponse**](AppPreOrderResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_pre_orders_delete_instance

> app_pre_orders_delete_instance(id)


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


## app_pre_orders_get_instance

> models::AppPreOrderResponse app_pre_orders_get_instance(id, fields_left_square_bracket_app_pre_orders_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_pre_orders_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreOrders |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPreOrderResponse**](AppPreOrderResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_pre_orders_update_instance

> models::AppPreOrderResponse app_pre_orders_update_instance(id, app_pre_order_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_pre_order_update_request** | [**AppPreOrderUpdateRequest**](AppPreOrderUpdateRequest.md) | AppPreOrder representation | [required] |

### Return type

[**models::AppPreOrderResponse**](AppPreOrderResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


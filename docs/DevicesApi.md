# \DevicesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**devices_create_instance**](DevicesApi.md#devices_create_instance) | **POST** /v1/devices | 
[**devices_get_collection**](DevicesApi.md#devices_get_collection) | **GET** /v1/devices | 
[**devices_get_instance**](DevicesApi.md#devices_get_instance) | **GET** /v1/devices/{id} | 
[**devices_update_instance**](DevicesApi.md#devices_update_instance) | **PATCH** /v1/devices/{id} | 



## devices_create_instance

> models::DeviceResponse devices_create_instance(device_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_create_request** | [**DeviceCreateRequest**](DeviceCreateRequest.md) | Device representation | [required] |

### Return type

[**models::DeviceResponse**](DeviceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_get_collection

> models::DevicesResponse devices_get_collection(filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_udid_right_square_bracket, filter_left_square_bracket_status_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_devices_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_udid_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'udid' |  |
**filter_left_square_bracket_status_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'status' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_devices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type devices |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::DevicesResponse**](DevicesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_get_instance

> models::DeviceResponse devices_get_instance(id, fields_left_square_bracket_devices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_devices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type devices |  |

### Return type

[**models::DeviceResponse**](DeviceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_update_instance

> models::DeviceResponse devices_update_instance(id, device_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**device_update_request** | [**DeviceUpdateRequest**](DeviceUpdateRequest.md) | Device representation | [required] |

### Return type

[**models::DeviceResponse**](DeviceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


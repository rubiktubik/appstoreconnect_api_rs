# \ActorsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**actors_get_collection**](ActorsApi.md#actors_get_collection) | **GET** /v1/actors | 
[**actors_get_instance**](ActorsApi.md#actors_get_instance) | **GET** /v1/actors/{id} | 



## actors_get_collection

> models::ActorsResponse actors_get_collection(filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_actors_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) | [required] |
**fields_left_square_bracket_actors_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type actors |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::ActorsResponse**](ActorsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actors_get_instance

> models::ActorResponse actors_get_instance(id, fields_left_square_bracket_actors_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_actors_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type actors |  |

### Return type

[**models::ActorResponse**](ActorResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


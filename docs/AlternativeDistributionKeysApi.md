# \AlternativeDistributionKeysApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alternative_distribution_keys_create_instance**](AlternativeDistributionKeysApi.md#alternative_distribution_keys_create_instance) | **POST** /v1/alternativeDistributionKeys | 
[**alternative_distribution_keys_delete_instance**](AlternativeDistributionKeysApi.md#alternative_distribution_keys_delete_instance) | **DELETE** /v1/alternativeDistributionKeys/{id} | 
[**alternative_distribution_keys_get_collection**](AlternativeDistributionKeysApi.md#alternative_distribution_keys_get_collection) | **GET** /v1/alternativeDistributionKeys | 
[**alternative_distribution_keys_get_instance**](AlternativeDistributionKeysApi.md#alternative_distribution_keys_get_instance) | **GET** /v1/alternativeDistributionKeys/{id} | 



## alternative_distribution_keys_create_instance

> models::AlternativeDistributionKeyResponse alternative_distribution_keys_create_instance(alternative_distribution_key_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alternative_distribution_key_create_request** | [**AlternativeDistributionKeyCreateRequest**](AlternativeDistributionKeyCreateRequest.md) | AlternativeDistributionKey representation | [required] |

### Return type

[**models::AlternativeDistributionKeyResponse**](AlternativeDistributionKeyResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_keys_delete_instance

> alternative_distribution_keys_delete_instance(id)


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


## alternative_distribution_keys_get_collection

> models::AlternativeDistributionKeysResponse alternative_distribution_keys_get_collection(exists_left_square_bracket_app_right_square_bracket, fields_left_square_bracket_alternative_distribution_keys_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exists_left_square_bracket_app_right_square_bracket** | Option<**bool**> | filter by existence or non-existence of related 'app' |  |
**fields_left_square_bracket_alternative_distribution_keys_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionKeys |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AlternativeDistributionKeysResponse**](AlternativeDistributionKeysResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_keys_get_instance

> models::AlternativeDistributionKeyResponse alternative_distribution_keys_get_instance(id, fields_left_square_bracket_alternative_distribution_keys_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_keys_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionKeys |  |

### Return type

[**models::AlternativeDistributionKeyResponse**](AlternativeDistributionKeyResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


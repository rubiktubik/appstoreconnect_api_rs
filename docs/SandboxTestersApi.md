# \SandboxTestersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sandbox_testers_v2_get_collection**](SandboxTestersApi.md#sandbox_testers_v2_get_collection) | **GET** /v2/sandboxTesters | 
[**sandbox_testers_v2_update_instance**](SandboxTestersApi.md#sandbox_testers_v2_update_instance) | **PATCH** /v2/sandboxTesters/{id} | 



## sandbox_testers_v2_get_collection

> models::SandboxTestersV2Response sandbox_testers_v2_get_collection(fields_left_square_bracket_sandbox_testers_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_sandbox_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type sandboxTesters |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::SandboxTestersV2Response**](SandboxTestersV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_testers_v2_update_instance

> models::SandboxTesterV2Response sandbox_testers_v2_update_instance(id, sandbox_tester_v2_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**sandbox_tester_v2_update_request** | [**SandboxTesterV2UpdateRequest**](SandboxTesterV2UpdateRequest.md) | SandboxTester representation | [required] |

### Return type

[**models::SandboxTesterV2Response**](SandboxTesterV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


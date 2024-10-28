# \CiTestResultsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_test_results_get_instance**](CiTestResultsApi.md#ci_test_results_get_instance) | **GET** /v1/ciTestResults/{id} | 



## ci_test_results_get_instance

> models::CiTestResultResponse ci_test_results_get_instance(id, fields_left_square_bracket_ci_test_results_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_test_results_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciTestResults |  |

### Return type

[**models::CiTestResultResponse**](CiTestResultResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DiagnosticSignaturesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**diagnostic_signatures_logs_get_to_many_related**](DiagnosticSignaturesApi.md#diagnostic_signatures_logs_get_to_many_related) | **GET** /v1/diagnosticSignatures/{id}/logs | 



## diagnostic_signatures_logs_get_to_many_related

> models::DiagnosticLogs diagnostic_signatures_logs_get_to_many_related(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::DiagnosticLogs**](diagnosticLogs.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.apple.diagnostic-logs+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


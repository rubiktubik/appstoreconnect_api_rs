# \ScmPullRequestsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scm_pull_requests_get_instance**](ScmPullRequestsApi.md#scm_pull_requests_get_instance) | **GET** /v1/scmPullRequests/{id} | 



## scm_pull_requests_get_instance

> models::ScmPullRequestResponse scm_pull_requests_get_instance(id, fields_left_square_bracket_scm_pull_requests_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_pull_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmPullRequests |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmPullRequestResponse**](ScmPullRequestResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


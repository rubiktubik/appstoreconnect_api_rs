# \ScmGitReferencesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scm_git_references_get_instance**](ScmGitReferencesApi.md#scm_git_references_get_instance) | **GET** /v1/scmGitReferences/{id} | 



## scm_git_references_get_instance

> models::ScmGitReferenceResponse scm_git_references_get_instance(id, fields_left_square_bracket_scm_git_references_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmGitReferenceResponse**](ScmGitReferenceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


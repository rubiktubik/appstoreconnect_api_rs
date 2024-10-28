# \ScmRepositoriesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scm_repositories_get_collection**](ScmRepositoriesApi.md#scm_repositories_get_collection) | **GET** /v1/scmRepositories | 
[**scm_repositories_get_instance**](ScmRepositoriesApi.md#scm_repositories_get_instance) | **GET** /v1/scmRepositories/{id} | 
[**scm_repositories_git_references_get_to_many_related**](ScmRepositoriesApi.md#scm_repositories_git_references_get_to_many_related) | **GET** /v1/scmRepositories/{id}/gitReferences | 
[**scm_repositories_pull_requests_get_to_many_related**](ScmRepositoriesApi.md#scm_repositories_pull_requests_get_to_many_related) | **GET** /v1/scmRepositories/{id}/pullRequests | 



## scm_repositories_get_collection

> models::ScmRepositoriesResponse scm_repositories_get_collection(filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmRepositoriesResponse**](ScmRepositoriesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scm_repositories_get_instance

> models::ScmRepositoryResponse scm_repositories_get_instance(id, fields_left_square_bracket_scm_repositories_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmRepositoryResponse**](ScmRepositoryResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scm_repositories_git_references_get_to_many_related

> models::ScmGitReferencesResponse scm_repositories_git_references_get_to_many_related(id, fields_left_square_bracket_scm_git_references_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmGitReferencesResponse**](ScmGitReferencesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scm_repositories_pull_requests_get_to_many_related

> models::ScmPullRequestsResponse scm_repositories_pull_requests_get_to_many_related(id, fields_left_square_bracket_scm_pull_requests_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_pull_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmPullRequests |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmPullRequestsResponse**](ScmPullRequestsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


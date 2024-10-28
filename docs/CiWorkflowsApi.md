# \CiWorkflowsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_workflows_build_runs_get_to_many_related**](CiWorkflowsApi.md#ci_workflows_build_runs_get_to_many_related) | **GET** /v1/ciWorkflows/{id}/buildRuns | 
[**ci_workflows_create_instance**](CiWorkflowsApi.md#ci_workflows_create_instance) | **POST** /v1/ciWorkflows | 
[**ci_workflows_delete_instance**](CiWorkflowsApi.md#ci_workflows_delete_instance) | **DELETE** /v1/ciWorkflows/{id} | 
[**ci_workflows_get_instance**](CiWorkflowsApi.md#ci_workflows_get_instance) | **GET** /v1/ciWorkflows/{id} | 
[**ci_workflows_repository_get_to_one_related**](CiWorkflowsApi.md#ci_workflows_repository_get_to_one_related) | **GET** /v1/ciWorkflows/{id}/repository | 
[**ci_workflows_update_instance**](CiWorkflowsApi.md#ci_workflows_update_instance) | **PATCH** /v1/ciWorkflows/{id} | 



## ci_workflows_build_runs_get_to_many_related

> models::CiBuildRunsResponse ci_workflows_build_runs_get_to_many_related(id, filter_left_square_bracket_builds_right_square_bracket, sort, fields_left_square_bracket_ci_build_runs_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_ci_workflows_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, fields_left_square_bracket_scm_pull_requests_right_square_bracket, limit, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_ci_workflows_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciWorkflows |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**fields_left_square_bracket_scm_pull_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmPullRequests |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::CiBuildRunsResponse**](CiBuildRunsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_workflows_create_instance

> models::CiWorkflowResponse ci_workflows_create_instance(ci_workflow_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ci_workflow_create_request** | [**CiWorkflowCreateRequest**](CiWorkflowCreateRequest.md) | CiWorkflow representation | [required] |

### Return type

[**models::CiWorkflowResponse**](CiWorkflowResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_workflows_delete_instance

> ci_workflows_delete_instance(id)


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


## ci_workflows_get_instance

> models::CiWorkflowResponse ci_workflows_get_instance(id, fields_left_square_bracket_ci_workflows_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_workflows_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciWorkflows |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CiWorkflowResponse**](CiWorkflowResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_workflows_repository_get_to_one_related

> models::ScmRepositoryResponse ci_workflows_repository_get_to_one_related(id, fields_left_square_bracket_scm_repositories_right_square_bracket, fields_left_square_bracket_scm_providers_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ScmRepositoryResponse**](ScmRepositoryResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_workflows_update_instance

> models::CiWorkflowResponse ci_workflows_update_instance(id, ci_workflow_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**ci_workflow_update_request** | [**CiWorkflowUpdateRequest**](CiWorkflowUpdateRequest.md) | CiWorkflow representation | [required] |

### Return type

[**models::CiWorkflowResponse**](CiWorkflowResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


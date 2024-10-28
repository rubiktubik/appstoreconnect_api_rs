# \CiBuildActionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_build_actions_artifacts_get_to_many_related**](CiBuildActionsApi.md#ci_build_actions_artifacts_get_to_many_related) | **GET** /v1/ciBuildActions/{id}/artifacts | 
[**ci_build_actions_build_run_get_to_one_related**](CiBuildActionsApi.md#ci_build_actions_build_run_get_to_one_related) | **GET** /v1/ciBuildActions/{id}/buildRun | 
[**ci_build_actions_get_instance**](CiBuildActionsApi.md#ci_build_actions_get_instance) | **GET** /v1/ciBuildActions/{id} | 
[**ci_build_actions_issues_get_to_many_related**](CiBuildActionsApi.md#ci_build_actions_issues_get_to_many_related) | **GET** /v1/ciBuildActions/{id}/issues | 
[**ci_build_actions_test_results_get_to_many_related**](CiBuildActionsApi.md#ci_build_actions_test_results_get_to_many_related) | **GET** /v1/ciBuildActions/{id}/testResults | 



## ci_build_actions_artifacts_get_to_many_related

> models::CiArtifactsResponse ci_build_actions_artifacts_get_to_many_related(id, fields_left_square_bracket_ci_artifacts_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_artifacts_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciArtifacts |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::CiArtifactsResponse**](CiArtifactsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_actions_build_run_get_to_one_related

> models::CiBuildRunResponse ci_build_actions_build_run_get_to_one_related(id, fields_left_square_bracket_ci_build_runs_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_ci_workflows_right_square_bracket, fields_left_square_bracket_ci_products_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, fields_left_square_bracket_scm_pull_requests_right_square_bracket, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_ci_workflows_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciWorkflows |  |
**fields_left_square_bracket_ci_products_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciProducts |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
**fields_left_square_bracket_scm_pull_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmPullRequests |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::CiBuildRunResponse**](CiBuildRunResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_actions_get_instance

> models::CiBuildActionResponse ci_build_actions_get_instance(id, fields_left_square_bracket_ci_build_actions_right_square_bracket, fields_left_square_bracket_ci_build_runs_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_build_actions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildActions |  |
**fields_left_square_bracket_ci_build_runs_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciBuildRuns |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CiBuildActionResponse**](CiBuildActionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_actions_issues_get_to_many_related

> models::CiIssuesResponse ci_build_actions_issues_get_to_many_related(id, fields_left_square_bracket_ci_issues_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_issues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciIssues |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::CiIssuesResponse**](CiIssuesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_build_actions_test_results_get_to_many_related

> models::CiTestResultsResponse ci_build_actions_test_results_get_to_many_related(id, fields_left_square_bracket_ci_test_results_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_test_results_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciTestResults |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::CiTestResultsResponse**](CiTestResultsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


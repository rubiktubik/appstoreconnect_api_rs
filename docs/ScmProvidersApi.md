# \ScmProvidersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scm_providers_get_collection**](ScmProvidersApi.md#scm_providers_get_collection) | **GET** /v1/scmProviders | 
[**scm_providers_get_instance**](ScmProvidersApi.md#scm_providers_get_instance) | **GET** /v1/scmProviders/{id} | 
[**scm_providers_repositories_get_to_many_related**](ScmProvidersApi.md#scm_providers_repositories_get_to_many_related) | **GET** /v1/scmProviders/{id}/repositories | 



## scm_providers_get_collection

> models::ScmProvidersResponse scm_providers_get_collection(fields_left_square_bracket_scm_providers_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::ScmProvidersResponse**](ScmProvidersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scm_providers_get_instance

> models::ScmProviderResponse scm_providers_get_instance(id, fields_left_square_bracket_scm_providers_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |

### Return type

[**models::ScmProviderResponse**](ScmProviderResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scm_providers_repositories_get_to_many_related

> models::ScmRepositoriesResponse scm_providers_repositories_get_to_many_related(id, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_scm_repositories_right_square_bracket, fields_left_square_bracket_scm_providers_right_square_bracket, fields_left_square_bracket_scm_git_references_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_scm_repositories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmRepositories |  |
**fields_left_square_bracket_scm_providers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmProviders |  |
**fields_left_square_bracket_scm_git_references_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type scmGitReferences |  |
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


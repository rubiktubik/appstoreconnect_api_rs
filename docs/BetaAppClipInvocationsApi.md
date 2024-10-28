# \BetaAppClipInvocationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_app_clip_invocations_create_instance**](BetaAppClipInvocationsApi.md#beta_app_clip_invocations_create_instance) | **POST** /v1/betaAppClipInvocations | 
[**beta_app_clip_invocations_delete_instance**](BetaAppClipInvocationsApi.md#beta_app_clip_invocations_delete_instance) | **DELETE** /v1/betaAppClipInvocations/{id} | 
[**beta_app_clip_invocations_get_instance**](BetaAppClipInvocationsApi.md#beta_app_clip_invocations_get_instance) | **GET** /v1/betaAppClipInvocations/{id} | 
[**beta_app_clip_invocations_update_instance**](BetaAppClipInvocationsApi.md#beta_app_clip_invocations_update_instance) | **PATCH** /v1/betaAppClipInvocations/{id} | 



## beta_app_clip_invocations_create_instance

> models::BetaAppClipInvocationResponse beta_app_clip_invocations_create_instance(beta_app_clip_invocation_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_app_clip_invocation_create_request** | [**BetaAppClipInvocationCreateRequest**](BetaAppClipInvocationCreateRequest.md) | BetaAppClipInvocation representation | [required] |

### Return type

[**models::BetaAppClipInvocationResponse**](BetaAppClipInvocationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_clip_invocations_delete_instance

> beta_app_clip_invocations_delete_instance(id)


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


## beta_app_clip_invocations_get_instance

> models::BetaAppClipInvocationResponse beta_app_clip_invocations_get_instance(id, fields_left_square_bracket_beta_app_clip_invocations_right_square_bracket, include, limit_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_clip_invocations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppClipInvocations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaAppClipInvocationLocalizations returned (when they are included) |  |

### Return type

[**models::BetaAppClipInvocationResponse**](BetaAppClipInvocationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_clip_invocations_update_instance

> models::BetaAppClipInvocationResponse beta_app_clip_invocations_update_instance(id, beta_app_clip_invocation_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_app_clip_invocation_update_request** | [**BetaAppClipInvocationUpdateRequest**](BetaAppClipInvocationUpdateRequest.md) | BetaAppClipInvocation representation | [required] |

### Return type

[**models::BetaAppClipInvocationResponse**](BetaAppClipInvocationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


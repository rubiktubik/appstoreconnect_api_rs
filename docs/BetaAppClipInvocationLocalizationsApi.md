# \BetaAppClipInvocationLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_app_clip_invocation_localizations_create_instance**](BetaAppClipInvocationLocalizationsApi.md#beta_app_clip_invocation_localizations_create_instance) | **POST** /v1/betaAppClipInvocationLocalizations | 
[**beta_app_clip_invocation_localizations_delete_instance**](BetaAppClipInvocationLocalizationsApi.md#beta_app_clip_invocation_localizations_delete_instance) | **DELETE** /v1/betaAppClipInvocationLocalizations/{id} | 
[**beta_app_clip_invocation_localizations_update_instance**](BetaAppClipInvocationLocalizationsApi.md#beta_app_clip_invocation_localizations_update_instance) | **PATCH** /v1/betaAppClipInvocationLocalizations/{id} | 



## beta_app_clip_invocation_localizations_create_instance

> models::BetaAppClipInvocationLocalizationResponse beta_app_clip_invocation_localizations_create_instance(beta_app_clip_invocation_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_app_clip_invocation_localization_create_request** | [**BetaAppClipInvocationLocalizationCreateRequest**](BetaAppClipInvocationLocalizationCreateRequest.md) | BetaAppClipInvocationLocalization representation | [required] |

### Return type

[**models::BetaAppClipInvocationLocalizationResponse**](BetaAppClipInvocationLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_clip_invocation_localizations_delete_instance

> beta_app_clip_invocation_localizations_delete_instance(id)


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


## beta_app_clip_invocation_localizations_update_instance

> models::BetaAppClipInvocationLocalizationResponse beta_app_clip_invocation_localizations_update_instance(id, beta_app_clip_invocation_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_app_clip_invocation_localization_update_request** | [**BetaAppClipInvocationLocalizationUpdateRequest**](BetaAppClipInvocationLocalizationUpdateRequest.md) | BetaAppClipInvocationLocalization representation | [required] |

### Return type

[**models::BetaAppClipInvocationLocalizationResponse**](BetaAppClipInvocationLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


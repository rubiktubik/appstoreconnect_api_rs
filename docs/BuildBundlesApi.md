# \BuildBundlesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_bundles_app_clip_domain_cache_status_get_to_one_related**](BuildBundlesApi.md#build_bundles_app_clip_domain_cache_status_get_to_one_related) | **GET** /v1/buildBundles/{id}/appClipDomainCacheStatus | 
[**build_bundles_app_clip_domain_debug_status_get_to_one_related**](BuildBundlesApi.md#build_bundles_app_clip_domain_debug_status_get_to_one_related) | **GET** /v1/buildBundles/{id}/appClipDomainDebugStatus | 
[**build_bundles_beta_app_clip_invocations_get_to_many_related**](BuildBundlesApi.md#build_bundles_beta_app_clip_invocations_get_to_many_related) | **GET** /v1/buildBundles/{id}/betaAppClipInvocations | 
[**build_bundles_build_bundle_file_sizes_get_to_many_related**](BuildBundlesApi.md#build_bundles_build_bundle_file_sizes_get_to_many_related) | **GET** /v1/buildBundles/{id}/buildBundleFileSizes | 



## build_bundles_app_clip_domain_cache_status_get_to_one_related

> models::AppClipDomainStatusResponse build_bundles_app_clip_domain_cache_status_get_to_one_related(id, fields_left_square_bracket_app_clip_domain_statuses_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_domain_statuses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDomainStatuses |  |

### Return type

[**models::AppClipDomainStatusResponse**](AppClipDomainStatusResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_bundles_app_clip_domain_debug_status_get_to_one_related

> models::AppClipDomainStatusResponse build_bundles_app_clip_domain_debug_status_get_to_one_related(id, fields_left_square_bracket_app_clip_domain_statuses_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_domain_statuses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDomainStatuses |  |

### Return type

[**models::AppClipDomainStatusResponse**](AppClipDomainStatusResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_bundles_beta_app_clip_invocations_get_to_many_related

> models::BetaAppClipInvocationsResponse build_bundles_beta_app_clip_invocations_get_to_many_related(id, fields_left_square_bracket_beta_app_clip_invocations_right_square_bracket, fields_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket, limit, include, limit_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_clip_invocations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppClipInvocations |  |
**fields_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppClipInvocationLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_app_clip_invocation_localizations_right_square_bracket** | Option<**i32**> | maximum number of related betaAppClipInvocationLocalizations returned (when they are included) |  |

### Return type

[**models::BetaAppClipInvocationsResponse**](BetaAppClipInvocationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_bundles_build_bundle_file_sizes_get_to_many_related

> models::BuildBundleFileSizesResponse build_bundles_build_bundle_file_sizes_get_to_many_related(id, fields_left_square_bracket_build_bundle_file_sizes_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_build_bundle_file_sizes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBundleFileSizes |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BuildBundleFileSizesResponse**](BuildBundleFileSizesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


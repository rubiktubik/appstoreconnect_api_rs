# \BundleIdCapabilitiesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bundle_id_capabilities_create_instance**](BundleIdCapabilitiesApi.md#bundle_id_capabilities_create_instance) | **POST** /v1/bundleIdCapabilities | 
[**bundle_id_capabilities_delete_instance**](BundleIdCapabilitiesApi.md#bundle_id_capabilities_delete_instance) | **DELETE** /v1/bundleIdCapabilities/{id} | 
[**bundle_id_capabilities_update_instance**](BundleIdCapabilitiesApi.md#bundle_id_capabilities_update_instance) | **PATCH** /v1/bundleIdCapabilities/{id} | 



## bundle_id_capabilities_create_instance

> models::BundleIdCapabilityResponse bundle_id_capabilities_create_instance(bundle_id_capability_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id_capability_create_request** | [**BundleIdCapabilityCreateRequest**](BundleIdCapabilityCreateRequest.md) | BundleIdCapability representation | [required] |

### Return type

[**models::BundleIdCapabilityResponse**](BundleIdCapabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_id_capabilities_delete_instance

> bundle_id_capabilities_delete_instance(id)


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


## bundle_id_capabilities_update_instance

> models::BundleIdCapabilityResponse bundle_id_capabilities_update_instance(id, bundle_id_capability_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**bundle_id_capability_update_request** | [**BundleIdCapabilityUpdateRequest**](BundleIdCapabilityUpdateRequest.md) | BundleIdCapability representation | [required] |

### Return type

[**models::BundleIdCapabilityResponse**](BundleIdCapabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


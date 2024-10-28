# \BundleIdsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bundle_ids_app_get_to_one_related**](BundleIdsApi.md#bundle_ids_app_get_to_one_related) | **GET** /v1/bundleIds/{id}/app | 
[**bundle_ids_bundle_id_capabilities_get_to_many_related**](BundleIdsApi.md#bundle_ids_bundle_id_capabilities_get_to_many_related) | **GET** /v1/bundleIds/{id}/bundleIdCapabilities | 
[**bundle_ids_create_instance**](BundleIdsApi.md#bundle_ids_create_instance) | **POST** /v1/bundleIds | 
[**bundle_ids_delete_instance**](BundleIdsApi.md#bundle_ids_delete_instance) | **DELETE** /v1/bundleIds/{id} | 
[**bundle_ids_get_collection**](BundleIdsApi.md#bundle_ids_get_collection) | **GET** /v1/bundleIds | 
[**bundle_ids_get_instance**](BundleIdsApi.md#bundle_ids_get_instance) | **GET** /v1/bundleIds/{id} | 
[**bundle_ids_profiles_get_to_many_related**](BundleIdsApi.md#bundle_ids_profiles_get_to_many_related) | **GET** /v1/bundleIds/{id}/profiles | 
[**bundle_ids_update_instance**](BundleIdsApi.md#bundle_ids_update_instance) | **PATCH** /v1/bundleIds/{id} | 



## bundle_ids_app_get_to_one_related

> models::AppWithoutIncludesResponse bundle_ids_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |

### Return type

[**models::AppWithoutIncludesResponse**](AppWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_bundle_id_capabilities_get_to_many_related

> models::BundleIdCapabilitiesWithoutIncludesResponse bundle_ids_bundle_id_capabilities_get_to_many_related(id, fields_left_square_bracket_bundle_id_capabilities_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_bundle_id_capabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIdCapabilities |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BundleIdCapabilitiesWithoutIncludesResponse**](BundleIdCapabilitiesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_create_instance

> models::BundleIdResponse bundle_ids_create_instance(bundle_id_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id_create_request** | [**BundleIdCreateRequest**](BundleIdCreateRequest.md) | BundleId representation | [required] |

### Return type

[**models::BundleIdResponse**](BundleIdResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_delete_instance

> bundle_ids_delete_instance(id)


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


## bundle_ids_get_collection

> models::BundleIdsResponse bundle_ids_get_collection(filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_identifier_right_square_bracket, filter_left_square_bracket_seed_id_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_bundle_ids_right_square_bracket, fields_left_square_bracket_profiles_right_square_bracket, fields_left_square_bracket_bundle_id_capabilities_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include, limit_left_square_bracket_bundle_id_capabilities_right_square_bracket, limit_left_square_bracket_profiles_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_identifier_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'identifier' |  |
**filter_left_square_bracket_seed_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'seedId' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_bundle_ids_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIds |  |
**fields_left_square_bracket_profiles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type profiles |  |
**fields_left_square_bracket_bundle_id_capabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIdCapabilities |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_bundle_id_capabilities_right_square_bracket** | Option<**i32**> | maximum number of related bundleIdCapabilities returned (when they are included) |  |
**limit_left_square_bracket_profiles_right_square_bracket** | Option<**i32**> | maximum number of related profiles returned (when they are included) |  |

### Return type

[**models::BundleIdsResponse**](BundleIdsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_get_instance

> models::BundleIdResponse bundle_ids_get_instance(id, fields_left_square_bracket_bundle_ids_right_square_bracket, fields_left_square_bracket_profiles_right_square_bracket, fields_left_square_bracket_bundle_id_capabilities_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include, limit_left_square_bracket_bundle_id_capabilities_right_square_bracket, limit_left_square_bracket_profiles_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_bundle_ids_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIds |  |
**fields_left_square_bracket_profiles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type profiles |  |
**fields_left_square_bracket_bundle_id_capabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIdCapabilities |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_bundle_id_capabilities_right_square_bracket** | Option<**i32**> | maximum number of related bundleIdCapabilities returned (when they are included) |  |
**limit_left_square_bracket_profiles_right_square_bracket** | Option<**i32**> | maximum number of related profiles returned (when they are included) |  |

### Return type

[**models::BundleIdResponse**](BundleIdResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_profiles_get_to_many_related

> models::ProfilesWithoutIncludesResponse bundle_ids_profiles_get_to_many_related(id, fields_left_square_bracket_profiles_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_profiles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type profiles |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::ProfilesWithoutIncludesResponse**](ProfilesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_ids_update_instance

> models::BundleIdResponse bundle_ids_update_instance(id, bundle_id_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**bundle_id_update_request** | [**BundleIdUpdateRequest**](BundleIdUpdateRequest.md) | BundleId representation | [required] |

### Return type

[**models::BundleIdResponse**](BundleIdResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


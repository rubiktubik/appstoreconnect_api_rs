# \ProfilesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**profiles_bundle_id_get_to_one_related**](ProfilesApi.md#profiles_bundle_id_get_to_one_related) | **GET** /v1/profiles/{id}/bundleId | 
[**profiles_certificates_get_to_many_related**](ProfilesApi.md#profiles_certificates_get_to_many_related) | **GET** /v1/profiles/{id}/certificates | 
[**profiles_create_instance**](ProfilesApi.md#profiles_create_instance) | **POST** /v1/profiles | 
[**profiles_delete_instance**](ProfilesApi.md#profiles_delete_instance) | **DELETE** /v1/profiles/{id} | 
[**profiles_devices_get_to_many_related**](ProfilesApi.md#profiles_devices_get_to_many_related) | **GET** /v1/profiles/{id}/devices | 
[**profiles_get_collection**](ProfilesApi.md#profiles_get_collection) | **GET** /v1/profiles | 
[**profiles_get_instance**](ProfilesApi.md#profiles_get_instance) | **GET** /v1/profiles/{id} | 



## profiles_bundle_id_get_to_one_related

> models::BundleIdWithoutIncludesResponse profiles_bundle_id_get_to_one_related(id, fields_left_square_bracket_bundle_ids_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_bundle_ids_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIds |  |

### Return type

[**models::BundleIdWithoutIncludesResponse**](BundleIdWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_certificates_get_to_many_related

> models::CertificatesWithoutIncludesResponse profiles_certificates_get_to_many_related(id, fields_left_square_bracket_certificates_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_certificates_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type certificates |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::CertificatesWithoutIncludesResponse**](CertificatesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_create_instance

> models::ProfileResponse profiles_create_instance(profile_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_create_request** | [**ProfileCreateRequest**](ProfileCreateRequest.md) | Profile representation | [required] |

### Return type

[**models::ProfileResponse**](ProfileResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_delete_instance

> profiles_delete_instance(id)


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


## profiles_devices_get_to_many_related

> models::DevicesWithoutIncludesResponse profiles_devices_get_to_many_related(id, fields_left_square_bracket_devices_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_devices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type devices |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::DevicesWithoutIncludesResponse**](DevicesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_get_collection

> models::ProfilesResponse profiles_get_collection(filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_profile_type_right_square_bracket, filter_left_square_bracket_profile_state_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_profiles_right_square_bracket, fields_left_square_bracket_bundle_ids_right_square_bracket, fields_left_square_bracket_devices_right_square_bracket, fields_left_square_bracket_certificates_right_square_bracket, limit, include, limit_left_square_bracket_certificates_right_square_bracket, limit_left_square_bracket_devices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_profile_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'profileType' |  |
**filter_left_square_bracket_profile_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'profileState' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_profiles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type profiles |  |
**fields_left_square_bracket_bundle_ids_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIds |  |
**fields_left_square_bracket_devices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type devices |  |
**fields_left_square_bracket_certificates_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type certificates |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_certificates_right_square_bracket** | Option<**i32**> | maximum number of related certificates returned (when they are included) |  |
**limit_left_square_bracket_devices_right_square_bracket** | Option<**i32**> | maximum number of related devices returned (when they are included) |  |

### Return type

[**models::ProfilesResponse**](ProfilesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_get_instance

> models::ProfileResponse profiles_get_instance(id, fields_left_square_bracket_profiles_right_square_bracket, fields_left_square_bracket_bundle_ids_right_square_bracket, fields_left_square_bracket_devices_right_square_bracket, fields_left_square_bracket_certificates_right_square_bracket, include, limit_left_square_bracket_certificates_right_square_bracket, limit_left_square_bracket_devices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_profiles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type profiles |  |
**fields_left_square_bracket_bundle_ids_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type bundleIds |  |
**fields_left_square_bracket_devices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type devices |  |
**fields_left_square_bracket_certificates_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type certificates |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_certificates_right_square_bracket** | Option<**i32**> | maximum number of related certificates returned (when they are included) |  |
**limit_left_square_bracket_devices_right_square_bracket** | Option<**i32**> | maximum number of related devices returned (when they are included) |  |

### Return type

[**models::ProfileResponse**](ProfileResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


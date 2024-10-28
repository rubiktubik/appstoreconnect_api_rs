# \CiMacOsVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ci_mac_os_versions_get_collection**](CiMacOsVersionsApi.md#ci_mac_os_versions_get_collection) | **GET** /v1/ciMacOsVersions | 
[**ci_mac_os_versions_get_instance**](CiMacOsVersionsApi.md#ci_mac_os_versions_get_instance) | **GET** /v1/ciMacOsVersions/{id} | 
[**ci_mac_os_versions_xcode_versions_get_to_many_related**](CiMacOsVersionsApi.md#ci_mac_os_versions_xcode_versions_get_to_many_related) | **GET** /v1/ciMacOsVersions/{id}/xcodeVersions | 



## ci_mac_os_versions_get_collection

> models::CiMacOsVersionsResponse ci_mac_os_versions_get_collection(fields_left_square_bracket_ci_mac_os_versions_right_square_bracket, fields_left_square_bracket_ci_xcode_versions_right_square_bracket, limit, include, limit_left_square_bracket_xcode_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_ci_mac_os_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciMacOsVersions |  |
**fields_left_square_bracket_ci_xcode_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciXcodeVersions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_xcode_versions_right_square_bracket** | Option<**i32**> | maximum number of related xcodeVersions returned (when they are included) |  |

### Return type

[**models::CiMacOsVersionsResponse**](CiMacOsVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_mac_os_versions_get_instance

> models::CiMacOsVersionResponse ci_mac_os_versions_get_instance(id, fields_left_square_bracket_ci_mac_os_versions_right_square_bracket, fields_left_square_bracket_ci_xcode_versions_right_square_bracket, include, limit_left_square_bracket_xcode_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_mac_os_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciMacOsVersions |  |
**fields_left_square_bracket_ci_xcode_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciXcodeVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_xcode_versions_right_square_bracket** | Option<**i32**> | maximum number of related xcodeVersions returned (when they are included) |  |

### Return type

[**models::CiMacOsVersionResponse**](CiMacOsVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ci_mac_os_versions_xcode_versions_get_to_many_related

> models::CiXcodeVersionsResponse ci_mac_os_versions_xcode_versions_get_to_many_related(id, fields_left_square_bracket_ci_xcode_versions_right_square_bracket, fields_left_square_bracket_ci_mac_os_versions_right_square_bracket, limit, include, limit_left_square_bracket_mac_os_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_ci_xcode_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciXcodeVersions |  |
**fields_left_square_bracket_ci_mac_os_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ciMacOsVersions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_mac_os_versions_right_square_bracket** | Option<**i32**> | maximum number of related macOsVersions returned (when they are included) |  |

### Return type

[**models::CiXcodeVersionsResponse**](CiXcodeVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


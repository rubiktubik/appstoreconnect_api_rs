# \AlternativeDistributionPackagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alternative_distribution_packages_create_instance**](AlternativeDistributionPackagesApi.md#alternative_distribution_packages_create_instance) | **POST** /v1/alternativeDistributionPackages | 
[**alternative_distribution_packages_get_instance**](AlternativeDistributionPackagesApi.md#alternative_distribution_packages_get_instance) | **GET** /v1/alternativeDistributionPackages/{id} | 
[**alternative_distribution_packages_versions_get_to_many_related**](AlternativeDistributionPackagesApi.md#alternative_distribution_packages_versions_get_to_many_related) | **GET** /v1/alternativeDistributionPackages/{id}/versions | 



## alternative_distribution_packages_create_instance

> models::AlternativeDistributionPackageResponse alternative_distribution_packages_create_instance(alternative_distribution_package_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alternative_distribution_package_create_request** | [**AlternativeDistributionPackageCreateRequest**](AlternativeDistributionPackageCreateRequest.md) | AlternativeDistributionPackage representation | [required] |

### Return type

[**models::AlternativeDistributionPackageResponse**](AlternativeDistributionPackageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_packages_get_instance

> models::AlternativeDistributionPackageResponse alternative_distribution_packages_get_instance(id, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket, include, limit_left_square_bracket_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_packages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackages |  |
**fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_versions_right_square_bracket** | Option<**i32**> | maximum number of related versions returned (when they are included) |  |

### Return type

[**models::AlternativeDistributionPackageResponse**](AlternativeDistributionPackageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_packages_versions_get_to_many_related

> models::AlternativeDistributionPackageVersionsResponse alternative_distribution_packages_versions_get_to_many_related(id, filter_left_square_bracket_state_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket, fields_left_square_bracket_alternative_distribution_packages_right_square_bracket, limit, include, limit_left_square_bracket_variants_right_square_bracket, limit_left_square_bracket_deltas_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVersions |  |
**fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVariants |  |
**fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageDeltas |  |
**fields_left_square_bracket_alternative_distribution_packages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_variants_right_square_bracket** | Option<**i32**> | maximum number of related variants returned (when they are included) |  |
**limit_left_square_bracket_deltas_right_square_bracket** | Option<**i32**> | maximum number of related deltas returned (when they are included) |  |

### Return type

[**models::AlternativeDistributionPackageVersionsResponse**](AlternativeDistributionPackageVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


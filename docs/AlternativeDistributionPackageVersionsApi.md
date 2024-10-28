# \AlternativeDistributionPackageVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alternative_distribution_package_versions_deltas_get_to_many_related**](AlternativeDistributionPackageVersionsApi.md#alternative_distribution_package_versions_deltas_get_to_many_related) | **GET** /v1/alternativeDistributionPackageVersions/{id}/deltas | 
[**alternative_distribution_package_versions_get_instance**](AlternativeDistributionPackageVersionsApi.md#alternative_distribution_package_versions_get_instance) | **GET** /v1/alternativeDistributionPackageVersions/{id} | 
[**alternative_distribution_package_versions_variants_get_to_many_related**](AlternativeDistributionPackageVersionsApi.md#alternative_distribution_package_versions_variants_get_to_many_related) | **GET** /v1/alternativeDistributionPackageVersions/{id}/variants | 



## alternative_distribution_package_versions_deltas_get_to_many_related

> models::AlternativeDistributionPackageDeltasResponse alternative_distribution_package_versions_deltas_get_to_many_related(id, fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageDeltas |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AlternativeDistributionPackageDeltasResponse**](AlternativeDistributionPackageDeltasResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_package_versions_get_instance

> models::AlternativeDistributionPackageVersionResponse alternative_distribution_package_versions_get_instance(id, fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket, fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket, include, limit_left_square_bracket_deltas_right_square_bracket, limit_left_square_bracket_variants_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_package_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVersions |  |
**fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVariants |  |
**fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageDeltas |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_deltas_right_square_bracket** | Option<**i32**> | maximum number of related deltas returned (when they are included) |  |
**limit_left_square_bracket_variants_right_square_bracket** | Option<**i32**> | maximum number of related variants returned (when they are included) |  |

### Return type

[**models::AlternativeDistributionPackageVersionResponse**](AlternativeDistributionPackageVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_package_versions_variants_get_to_many_related

> models::AlternativeDistributionPackageVariantsResponse alternative_distribution_package_versions_variants_get_to_many_related(id, fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_package_variants_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageVariants |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AlternativeDistributionPackageVariantsResponse**](AlternativeDistributionPackageVariantsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


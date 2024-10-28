# \AlternativeDistributionPackageDeltasApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alternative_distribution_package_deltas_get_instance**](AlternativeDistributionPackageDeltasApi.md#alternative_distribution_package_deltas_get_instance) | **GET** /v1/alternativeDistributionPackageDeltas/{id} | 



## alternative_distribution_package_deltas_get_instance

> models::AlternativeDistributionPackageDeltaResponse alternative_distribution_package_deltas_get_instance(id, fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_package_deltas_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionPackageDeltas |  |

### Return type

[**models::AlternativeDistributionPackageDeltaResponse**](AlternativeDistributionPackageDeltaResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


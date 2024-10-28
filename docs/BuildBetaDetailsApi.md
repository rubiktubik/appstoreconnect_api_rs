# \BuildBetaDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_beta_details_build_get_to_one_related**](BuildBetaDetailsApi.md#build_beta_details_build_get_to_one_related) | **GET** /v1/buildBetaDetails/{id}/build | 
[**build_beta_details_get_collection**](BuildBetaDetailsApi.md#build_beta_details_get_collection) | **GET** /v1/buildBetaDetails | 
[**build_beta_details_get_instance**](BuildBetaDetailsApi.md#build_beta_details_get_instance) | **GET** /v1/buildBetaDetails/{id} | 
[**build_beta_details_update_instance**](BuildBetaDetailsApi.md#build_beta_details_update_instance) | **PATCH** /v1/buildBetaDetails/{id} | 



## build_beta_details_build_get_to_one_related

> models::BuildWithoutIncludesResponse build_beta_details_build_get_to_one_related(id, fields_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |

### Return type

[**models::BuildWithoutIncludesResponse**](BuildWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_beta_details_get_collection

> models::BuildBetaDetailsResponse build_beta_details_get_collection(filter_left_square_bracket_build_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_build_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'build' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BuildBetaDetailsResponse**](BuildBetaDetailsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_beta_details_get_instance

> models::BuildBetaDetailResponse build_beta_details_get_instance(id, fields_left_square_bracket_build_beta_details_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_build_beta_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type buildBetaDetails |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BuildBetaDetailResponse**](BuildBetaDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_beta_details_update_instance

> models::BuildBetaDetailResponse build_beta_details_update_instance(id, build_beta_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**build_beta_detail_update_request** | [**BuildBetaDetailUpdateRequest**](BuildBetaDetailUpdateRequest.md) | BuildBetaDetail representation | [required] |

### Return type

[**models::BuildBetaDetailResponse**](BuildBetaDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


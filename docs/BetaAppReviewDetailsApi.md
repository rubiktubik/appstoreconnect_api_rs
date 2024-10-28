# \BetaAppReviewDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_app_review_details_app_get_to_one_related**](BetaAppReviewDetailsApi.md#beta_app_review_details_app_get_to_one_related) | **GET** /v1/betaAppReviewDetails/{id}/app | 
[**beta_app_review_details_get_collection**](BetaAppReviewDetailsApi.md#beta_app_review_details_get_collection) | **GET** /v1/betaAppReviewDetails | 
[**beta_app_review_details_get_instance**](BetaAppReviewDetailsApi.md#beta_app_review_details_get_instance) | **GET** /v1/betaAppReviewDetails/{id} | 
[**beta_app_review_details_update_instance**](BetaAppReviewDetailsApi.md#beta_app_review_details_update_instance) | **PATCH** /v1/betaAppReviewDetails/{id} | 



## beta_app_review_details_app_get_to_one_related

> models::AppWithoutIncludesResponse beta_app_review_details_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


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


## beta_app_review_details_get_collection

> models::BetaAppReviewDetailsResponse beta_app_review_details_get_collection(filter_left_square_bracket_app_right_square_bracket, fields_left_square_bracket_beta_app_review_details_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_app_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) of related 'app' | [required] |
**fields_left_square_bracket_beta_app_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewDetails |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppReviewDetailsResponse**](BetaAppReviewDetailsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_review_details_get_instance

> models::BetaAppReviewDetailResponse beta_app_review_details_get_instance(id, fields_left_square_bracket_beta_app_review_details_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewDetails |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppReviewDetailResponse**](BetaAppReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_review_details_update_instance

> models::BetaAppReviewDetailResponse beta_app_review_details_update_instance(id, beta_app_review_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_app_review_detail_update_request** | [**BetaAppReviewDetailUpdateRequest**](BetaAppReviewDetailUpdateRequest.md) | BetaAppReviewDetail representation | [required] |

### Return type

[**models::BetaAppReviewDetailResponse**](BetaAppReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


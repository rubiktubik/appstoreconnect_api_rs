# \BetaAppReviewSubmissionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_app_review_submissions_build_get_to_one_related**](BetaAppReviewSubmissionsApi.md#beta_app_review_submissions_build_get_to_one_related) | **GET** /v1/betaAppReviewSubmissions/{id}/build | 
[**beta_app_review_submissions_create_instance**](BetaAppReviewSubmissionsApi.md#beta_app_review_submissions_create_instance) | **POST** /v1/betaAppReviewSubmissions | 
[**beta_app_review_submissions_get_collection**](BetaAppReviewSubmissionsApi.md#beta_app_review_submissions_get_collection) | **GET** /v1/betaAppReviewSubmissions | 
[**beta_app_review_submissions_get_instance**](BetaAppReviewSubmissionsApi.md#beta_app_review_submissions_get_instance) | **GET** /v1/betaAppReviewSubmissions/{id} | 



## beta_app_review_submissions_build_get_to_one_related

> models::BuildWithoutIncludesResponse beta_app_review_submissions_build_get_to_one_related(id, fields_left_square_bracket_builds_right_square_bracket)


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


## beta_app_review_submissions_create_instance

> models::BetaAppReviewSubmissionResponse beta_app_review_submissions_create_instance(beta_app_review_submission_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_app_review_submission_create_request** | [**BetaAppReviewSubmissionCreateRequest**](BetaAppReviewSubmissionCreateRequest.md) | BetaAppReviewSubmission representation | [required] |

### Return type

[**models::BetaAppReviewSubmissionResponse**](BetaAppReviewSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_review_submissions_get_collection

> models::BetaAppReviewSubmissionsResponse beta_app_review_submissions_get_collection(filter_left_square_bracket_build_right_square_bracket, filter_left_square_bracket_beta_review_state_right_square_bracket, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_build_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) of related 'build' | [required] |
**filter_left_square_bracket_beta_review_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'betaReviewState' |  |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppReviewSubmissionsResponse**](BetaAppReviewSubmissionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_review_submissions_get_instance

> models::BetaAppReviewSubmissionResponse beta_app_review_submissions_get_instance(id, fields_left_square_bracket_beta_app_review_submissions_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppReviewSubmissions |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppReviewSubmissionResponse**](BetaAppReviewSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


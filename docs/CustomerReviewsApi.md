# \CustomerReviewsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_reviews_get_instance**](CustomerReviewsApi.md#customer_reviews_get_instance) | **GET** /v1/customerReviews/{id} | 
[**customer_reviews_response_get_to_one_related**](CustomerReviewsApi.md#customer_reviews_response_get_to_one_related) | **GET** /v1/customerReviews/{id}/response | 



## customer_reviews_get_instance

> models::CustomerReviewResponse customer_reviews_get_instance(id, fields_left_square_bracket_customer_reviews_right_square_bracket, fields_left_square_bracket_customer_review_responses_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_customer_reviews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviews |  |
**fields_left_square_bracket_customer_review_responses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviewResponses |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CustomerReviewResponse**](CustomerReviewResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_reviews_response_get_to_one_related

> models::CustomerReviewResponseV1Response customer_reviews_response_get_to_one_related(id, fields_left_square_bracket_customer_review_responses_right_square_bracket, fields_left_square_bracket_customer_reviews_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_customer_review_responses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviewResponses |  |
**fields_left_square_bracket_customer_reviews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviews |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CustomerReviewResponseV1Response**](CustomerReviewResponseV1Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


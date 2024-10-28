# \CustomerReviewResponsesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_review_responses_create_instance**](CustomerReviewResponsesApi.md#customer_review_responses_create_instance) | **POST** /v1/customerReviewResponses | 
[**customer_review_responses_delete_instance**](CustomerReviewResponsesApi.md#customer_review_responses_delete_instance) | **DELETE** /v1/customerReviewResponses/{id} | 
[**customer_review_responses_get_instance**](CustomerReviewResponsesApi.md#customer_review_responses_get_instance) | **GET** /v1/customerReviewResponses/{id} | 



## customer_review_responses_create_instance

> models::CustomerReviewResponseV1Response customer_review_responses_create_instance(customer_review_response_v1_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_review_response_v1_create_request** | [**CustomerReviewResponseV1CreateRequest**](CustomerReviewResponseV1CreateRequest.md) | CustomerReviewResponse representation | [required] |

### Return type

[**models::CustomerReviewResponseV1Response**](CustomerReviewResponseV1Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_review_responses_delete_instance

> customer_review_responses_delete_instance(id)


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


## customer_review_responses_get_instance

> models::CustomerReviewResponseV1Response customer_review_responses_get_instance(id, fields_left_square_bracket_customer_review_responses_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_customer_review_responses_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type customerReviewResponses |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::CustomerReviewResponseV1Response**](CustomerReviewResponseV1Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


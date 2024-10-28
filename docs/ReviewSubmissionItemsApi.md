# \ReviewSubmissionItemsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**review_submission_items_create_instance**](ReviewSubmissionItemsApi.md#review_submission_items_create_instance) | **POST** /v1/reviewSubmissionItems | 
[**review_submission_items_delete_instance**](ReviewSubmissionItemsApi.md#review_submission_items_delete_instance) | **DELETE** /v1/reviewSubmissionItems/{id} | 
[**review_submission_items_update_instance**](ReviewSubmissionItemsApi.md#review_submission_items_update_instance) | **PATCH** /v1/reviewSubmissionItems/{id} | 



## review_submission_items_create_instance

> models::ReviewSubmissionItemResponse review_submission_items_create_instance(review_submission_item_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_submission_item_create_request** | [**ReviewSubmissionItemCreateRequest**](ReviewSubmissionItemCreateRequest.md) | ReviewSubmissionItem representation | [required] |

### Return type

[**models::ReviewSubmissionItemResponse**](ReviewSubmissionItemResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_submission_items_delete_instance

> review_submission_items_delete_instance(id)


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


## review_submission_items_update_instance

> models::ReviewSubmissionItemResponse review_submission_items_update_instance(id, review_submission_item_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**review_submission_item_update_request** | [**ReviewSubmissionItemUpdateRequest**](ReviewSubmissionItemUpdateRequest.md) | ReviewSubmissionItem representation | [required] |

### Return type

[**models::ReviewSubmissionItemResponse**](ReviewSubmissionItemResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


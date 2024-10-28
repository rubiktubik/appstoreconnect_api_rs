# \ReviewSubmissionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**review_submissions_create_instance**](ReviewSubmissionsApi.md#review_submissions_create_instance) | **POST** /v1/reviewSubmissions | 
[**review_submissions_get_collection**](ReviewSubmissionsApi.md#review_submissions_get_collection) | **GET** /v1/reviewSubmissions | 
[**review_submissions_get_instance**](ReviewSubmissionsApi.md#review_submissions_get_instance) | **GET** /v1/reviewSubmissions/{id} | 
[**review_submissions_items_get_to_many_related**](ReviewSubmissionsApi.md#review_submissions_items_get_to_many_related) | **GET** /v1/reviewSubmissions/{id}/items | 
[**review_submissions_update_instance**](ReviewSubmissionsApi.md#review_submissions_update_instance) | **PATCH** /v1/reviewSubmissions/{id} | 



## review_submissions_create_instance

> models::ReviewSubmissionResponse review_submissions_create_instance(review_submission_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_submission_create_request** | [**ReviewSubmissionCreateRequest**](ReviewSubmissionCreateRequest.md) | ReviewSubmission representation | [required] |

### Return type

[**models::ReviewSubmissionResponse**](ReviewSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_submissions_get_collection

> models::ReviewSubmissionsResponse review_submissions_get_collection(filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_state_right_square_bracket, fields_left_square_bracket_review_submissions_right_square_bracket, fields_left_square_bracket_review_submission_items_right_square_bracket, limit, include, limit_left_square_bracket_items_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_app_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) of related 'app' | [required] |
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**fields_left_square_bracket_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissions |  |
**fields_left_square_bracket_review_submission_items_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissionItems |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_items_right_square_bracket** | Option<**i32**> | maximum number of related items returned (when they are included) |  |

### Return type

[**models::ReviewSubmissionsResponse**](ReviewSubmissionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_submissions_get_instance

> models::ReviewSubmissionResponse review_submissions_get_instance(id, fields_left_square_bracket_review_submissions_right_square_bracket, fields_left_square_bracket_review_submission_items_right_square_bracket, include, limit_left_square_bracket_items_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_review_submissions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissions |  |
**fields_left_square_bracket_review_submission_items_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissionItems |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_items_right_square_bracket** | Option<**i32**> | maximum number of related items returned (when they are included) |  |

### Return type

[**models::ReviewSubmissionResponse**](ReviewSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_submissions_items_get_to_many_related

> models::ReviewSubmissionItemsResponse review_submissions_items_get_to_many_related(id, fields_left_square_bracket_review_submission_items_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_events_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_review_submission_items_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type reviewSubmissionItems |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageVersions |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_events_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEvents |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::ReviewSubmissionItemsResponse**](ReviewSubmissionItemsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_submissions_update_instance

> models::ReviewSubmissionResponse review_submissions_update_instance(id, review_submission_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**review_submission_update_request** | [**ReviewSubmissionUpdateRequest**](ReviewSubmissionUpdateRequest.md) | ReviewSubmission representation | [required] |

### Return type

[**models::ReviewSubmissionResponse**](ReviewSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


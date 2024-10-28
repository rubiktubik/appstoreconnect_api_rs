# \AppStoreReviewDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_review_details_app_store_review_attachments_get_to_many_related**](AppStoreReviewDetailsApi.md#app_store_review_details_app_store_review_attachments_get_to_many_related) | **GET** /v1/appStoreReviewDetails/{id}/appStoreReviewAttachments | 
[**app_store_review_details_create_instance**](AppStoreReviewDetailsApi.md#app_store_review_details_create_instance) | **POST** /v1/appStoreReviewDetails | 
[**app_store_review_details_get_instance**](AppStoreReviewDetailsApi.md#app_store_review_details_get_instance) | **GET** /v1/appStoreReviewDetails/{id} | 
[**app_store_review_details_update_instance**](AppStoreReviewDetailsApi.md#app_store_review_details_update_instance) | **PATCH** /v1/appStoreReviewDetails/{id} | 



## app_store_review_details_app_store_review_attachments_get_to_many_related

> models::AppStoreReviewAttachmentsResponse app_store_review_details_app_store_review_attachments_get_to_many_related(id, fields_left_square_bracket_app_store_review_attachments_right_square_bracket, fields_left_square_bracket_app_store_review_details_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewAttachments |  |
**fields_left_square_bracket_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewDetails |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppStoreReviewAttachmentsResponse**](AppStoreReviewAttachmentsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_review_details_create_instance

> models::AppStoreReviewDetailResponse app_store_review_details_create_instance(app_store_review_detail_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_review_detail_create_request** | [**AppStoreReviewDetailCreateRequest**](AppStoreReviewDetailCreateRequest.md) | AppStoreReviewDetail representation | [required] |

### Return type

[**models::AppStoreReviewDetailResponse**](AppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_review_details_get_instance

> models::AppStoreReviewDetailResponse app_store_review_details_get_instance(id, fields_left_square_bracket_app_store_review_details_right_square_bracket, fields_left_square_bracket_app_store_review_attachments_right_square_bracket, include, limit_left_square_bracket_app_store_review_attachments_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewDetails |  |
**fields_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewAttachments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreReviewAttachments returned (when they are included) |  |

### Return type

[**models::AppStoreReviewDetailResponse**](AppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_review_details_update_instance

> models::AppStoreReviewDetailResponse app_store_review_details_update_instance(id, app_store_review_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_review_detail_update_request** | [**AppStoreReviewDetailUpdateRequest**](AppStoreReviewDetailUpdateRequest.md) | AppStoreReviewDetail representation | [required] |

### Return type

[**models::AppStoreReviewDetailResponse**](AppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


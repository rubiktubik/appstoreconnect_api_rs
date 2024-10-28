# \AppStoreReviewAttachmentsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_review_attachments_create_instance**](AppStoreReviewAttachmentsApi.md#app_store_review_attachments_create_instance) | **POST** /v1/appStoreReviewAttachments | 
[**app_store_review_attachments_delete_instance**](AppStoreReviewAttachmentsApi.md#app_store_review_attachments_delete_instance) | **DELETE** /v1/appStoreReviewAttachments/{id} | 
[**app_store_review_attachments_get_instance**](AppStoreReviewAttachmentsApi.md#app_store_review_attachments_get_instance) | **GET** /v1/appStoreReviewAttachments/{id} | 
[**app_store_review_attachments_update_instance**](AppStoreReviewAttachmentsApi.md#app_store_review_attachments_update_instance) | **PATCH** /v1/appStoreReviewAttachments/{id} | 



## app_store_review_attachments_create_instance

> models::AppStoreReviewAttachmentResponse app_store_review_attachments_create_instance(app_store_review_attachment_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_review_attachment_create_request** | [**AppStoreReviewAttachmentCreateRequest**](AppStoreReviewAttachmentCreateRequest.md) | AppStoreReviewAttachment representation | [required] |

### Return type

[**models::AppStoreReviewAttachmentResponse**](AppStoreReviewAttachmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_review_attachments_delete_instance

> app_store_review_attachments_delete_instance(id)


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


## app_store_review_attachments_get_instance

> models::AppStoreReviewAttachmentResponse app_store_review_attachments_get_instance(id, fields_left_square_bracket_app_store_review_attachments_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_review_attachments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreReviewAttachments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppStoreReviewAttachmentResponse**](AppStoreReviewAttachmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_review_attachments_update_instance

> models::AppStoreReviewAttachmentResponse app_store_review_attachments_update_instance(id, app_store_review_attachment_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_review_attachment_update_request** | [**AppStoreReviewAttachmentUpdateRequest**](AppStoreReviewAttachmentUpdateRequest.md) | AppStoreReviewAttachment representation | [required] |

### Return type

[**models::AppStoreReviewAttachmentResponse**](AppStoreReviewAttachmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


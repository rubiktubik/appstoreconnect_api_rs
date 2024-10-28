# \AppStoreVersionSubmissionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_version_submissions_create_instance**](AppStoreVersionSubmissionsApi.md#app_store_version_submissions_create_instance) | **POST** /v1/appStoreVersionSubmissions | 
[**app_store_version_submissions_delete_instance**](AppStoreVersionSubmissionsApi.md#app_store_version_submissions_delete_instance) | **DELETE** /v1/appStoreVersionSubmissions/{id} | 



## app_store_version_submissions_create_instance

> models::AppStoreVersionSubmissionResponse app_store_version_submissions_create_instance(app_store_version_submission_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_submission_create_request** | [**AppStoreVersionSubmissionCreateRequest**](AppStoreVersionSubmissionCreateRequest.md) | AppStoreVersionSubmission representation | [required] |

### Return type

[**models::AppStoreVersionSubmissionResponse**](AppStoreVersionSubmissionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_submissions_delete_instance

> app_store_version_submissions_delete_instance(id)


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


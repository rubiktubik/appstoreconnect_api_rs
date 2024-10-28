# \AppClipAppStoreReviewDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clip_app_store_review_details_create_instance**](AppClipAppStoreReviewDetailsApi.md#app_clip_app_store_review_details_create_instance) | **POST** /v1/appClipAppStoreReviewDetails | 
[**app_clip_app_store_review_details_get_instance**](AppClipAppStoreReviewDetailsApi.md#app_clip_app_store_review_details_get_instance) | **GET** /v1/appClipAppStoreReviewDetails/{id} | 
[**app_clip_app_store_review_details_update_instance**](AppClipAppStoreReviewDetailsApi.md#app_clip_app_store_review_details_update_instance) | **PATCH** /v1/appClipAppStoreReviewDetails/{id} | 



## app_clip_app_store_review_details_create_instance

> models::AppClipAppStoreReviewDetailResponse app_clip_app_store_review_details_create_instance(app_clip_app_store_review_detail_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_clip_app_store_review_detail_create_request** | [**AppClipAppStoreReviewDetailCreateRequest**](AppClipAppStoreReviewDetailCreateRequest.md) | AppClipAppStoreReviewDetail representation | [required] |

### Return type

[**models::AppClipAppStoreReviewDetailResponse**](AppClipAppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_app_store_review_details_get_instance

> models::AppClipAppStoreReviewDetailResponse app_clip_app_store_review_details_get_instance(id, fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAppStoreReviewDetails |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipAppStoreReviewDetailResponse**](AppClipAppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_app_store_review_details_update_instance

> models::AppClipAppStoreReviewDetailResponse app_clip_app_store_review_details_update_instance(id, app_clip_app_store_review_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_app_store_review_detail_update_request** | [**AppClipAppStoreReviewDetailUpdateRequest**](AppClipAppStoreReviewDetailUpdateRequest.md) | AppClipAppStoreReviewDetail representation | [required] |

### Return type

[**models::AppClipAppStoreReviewDetailResponse**](AppClipAppStoreReviewDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


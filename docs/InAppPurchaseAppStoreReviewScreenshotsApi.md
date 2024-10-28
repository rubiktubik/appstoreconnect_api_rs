# \InAppPurchaseAppStoreReviewScreenshotsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchase_app_store_review_screenshots_create_instance**](InAppPurchaseAppStoreReviewScreenshotsApi.md#in_app_purchase_app_store_review_screenshots_create_instance) | **POST** /v1/inAppPurchaseAppStoreReviewScreenshots | 
[**in_app_purchase_app_store_review_screenshots_delete_instance**](InAppPurchaseAppStoreReviewScreenshotsApi.md#in_app_purchase_app_store_review_screenshots_delete_instance) | **DELETE** /v1/inAppPurchaseAppStoreReviewScreenshots/{id} | 
[**in_app_purchase_app_store_review_screenshots_get_instance**](InAppPurchaseAppStoreReviewScreenshotsApi.md#in_app_purchase_app_store_review_screenshots_get_instance) | **GET** /v1/inAppPurchaseAppStoreReviewScreenshots/{id} | 
[**in_app_purchase_app_store_review_screenshots_update_instance**](InAppPurchaseAppStoreReviewScreenshotsApi.md#in_app_purchase_app_store_review_screenshots_update_instance) | **PATCH** /v1/inAppPurchaseAppStoreReviewScreenshots/{id} | 



## in_app_purchase_app_store_review_screenshots_create_instance

> models::InAppPurchaseAppStoreReviewScreenshotResponse in_app_purchase_app_store_review_screenshots_create_instance(in_app_purchase_app_store_review_screenshot_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_app_store_review_screenshot_create_request** | [**InAppPurchaseAppStoreReviewScreenshotCreateRequest**](InAppPurchaseAppStoreReviewScreenshotCreateRequest.md) | InAppPurchaseAppStoreReviewScreenshot representation | [required] |

### Return type

[**models::InAppPurchaseAppStoreReviewScreenshotResponse**](InAppPurchaseAppStoreReviewScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_app_store_review_screenshots_delete_instance

> in_app_purchase_app_store_review_screenshots_delete_instance(id)


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


## in_app_purchase_app_store_review_screenshots_get_instance

> models::InAppPurchaseAppStoreReviewScreenshotResponse in_app_purchase_app_store_review_screenshots_get_instance(id, fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAppStoreReviewScreenshots |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseAppStoreReviewScreenshotResponse**](InAppPurchaseAppStoreReviewScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_app_store_review_screenshots_update_instance

> models::InAppPurchaseAppStoreReviewScreenshotResponse in_app_purchase_app_store_review_screenshots_update_instance(id, in_app_purchase_app_store_review_screenshot_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**in_app_purchase_app_store_review_screenshot_update_request** | [**InAppPurchaseAppStoreReviewScreenshotUpdateRequest**](InAppPurchaseAppStoreReviewScreenshotUpdateRequest.md) | InAppPurchaseAppStoreReviewScreenshot representation | [required] |

### Return type

[**models::InAppPurchaseAppStoreReviewScreenshotResponse**](InAppPurchaseAppStoreReviewScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \InAppPurchasesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchases_get_instance**](InAppPurchasesApi.md#in_app_purchases_get_instance) | **GET** /v1/inAppPurchases/{id} | 
[**in_app_purchases_v2_app_store_review_screenshot_get_to_one_related**](InAppPurchasesApi.md#in_app_purchases_v2_app_store_review_screenshot_get_to_one_related) | **GET** /v2/inAppPurchases/{id}/appStoreReviewScreenshot | 
[**in_app_purchases_v2_content_get_to_one_related**](InAppPurchasesApi.md#in_app_purchases_v2_content_get_to_one_related) | **GET** /v2/inAppPurchases/{id}/content | 
[**in_app_purchases_v2_create_instance**](InAppPurchasesApi.md#in_app_purchases_v2_create_instance) | **POST** /v2/inAppPurchases | 
[**in_app_purchases_v2_delete_instance**](InAppPurchasesApi.md#in_app_purchases_v2_delete_instance) | **DELETE** /v2/inAppPurchases/{id} | 
[**in_app_purchases_v2_get_instance**](InAppPurchasesApi.md#in_app_purchases_v2_get_instance) | **GET** /v2/inAppPurchases/{id} | 
[**in_app_purchases_v2_iap_price_schedule_get_to_one_related**](InAppPurchasesApi.md#in_app_purchases_v2_iap_price_schedule_get_to_one_related) | **GET** /v2/inAppPurchases/{id}/iapPriceSchedule | 
[**in_app_purchases_v2_images_get_to_many_related**](InAppPurchasesApi.md#in_app_purchases_v2_images_get_to_many_related) | **GET** /v2/inAppPurchases/{id}/images | 
[**in_app_purchases_v2_in_app_purchase_availability_get_to_one_related**](InAppPurchasesApi.md#in_app_purchases_v2_in_app_purchase_availability_get_to_one_related) | **GET** /v2/inAppPurchases/{id}/inAppPurchaseAvailability | 
[**in_app_purchases_v2_in_app_purchase_localizations_get_to_many_related**](InAppPurchasesApi.md#in_app_purchases_v2_in_app_purchase_localizations_get_to_many_related) | **GET** /v2/inAppPurchases/{id}/inAppPurchaseLocalizations | 
[**in_app_purchases_v2_price_points_get_to_many_related**](InAppPurchasesApi.md#in_app_purchases_v2_price_points_get_to_many_related) | **GET** /v2/inAppPurchases/{id}/pricePoints | 
[**in_app_purchases_v2_promoted_purchase_get_to_one_related**](InAppPurchasesApi.md#in_app_purchases_v2_promoted_purchase_get_to_one_related) | **GET** /v2/inAppPurchases/{id}/promotedPurchase | 
[**in_app_purchases_v2_update_instance**](InAppPurchasesApi.md#in_app_purchases_v2_update_instance) | **PATCH** /v2/inAppPurchases/{id} | 



## in_app_purchases_get_instance

> models::InAppPurchaseResponse in_app_purchases_get_instance(id, fields_left_square_bracket_in_app_purchases_right_square_bracket, include, limit_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_apps_right_square_bracket** | Option<**i32**> | maximum number of related apps returned (when they are included) |  |

### Return type

[**models::InAppPurchaseResponse**](InAppPurchaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_app_store_review_screenshot_get_to_one_related

> models::InAppPurchaseAppStoreReviewScreenshotResponse in_app_purchases_v2_app_store_review_screenshot_get_to_one_related(id, fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAppStoreReviewScreenshots |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseAppStoreReviewScreenshotResponse**](InAppPurchaseAppStoreReviewScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_content_get_to_one_related

> models::InAppPurchaseContentResponse in_app_purchases_v2_content_get_to_one_related(id, fields_left_square_bracket_in_app_purchase_contents_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_contents_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseContents |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseContentResponse**](InAppPurchaseContentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_create_instance

> models::InAppPurchaseV2Response in_app_purchases_v2_create_instance(in_app_purchase_v2_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_v2_create_request** | [**InAppPurchaseV2CreateRequest**](InAppPurchaseV2CreateRequest.md) | InAppPurchase representation | [required] |

### Return type

[**models::InAppPurchaseV2Response**](InAppPurchaseV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_delete_instance

> in_app_purchases_v2_delete_instance(id)


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


## in_app_purchases_v2_get_instance

> models::InAppPurchaseV2Response in_app_purchases_v2_get_instance(id, fields_left_square_bracket_in_app_purchases_right_square_bracket, fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket, fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket, fields_left_square_bracket_in_app_purchase_contents_right_square_bracket, fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket, fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket, fields_left_square_bracket_in_app_purchase_images_right_square_bracket, include, limit_left_square_bracket_images_right_square_bracket, limit_left_square_bracket_in_app_purchase_localizations_right_square_bracket, limit_left_square_bracket_price_points_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseLocalizations |  |
**fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePricePoints |  |
**fields_left_square_bracket_in_app_purchase_contents_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseContents |  |
**fields_left_square_bracket_in_app_purchase_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAppStoreReviewScreenshots |  |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePriceSchedules |  |
**fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAvailabilities |  |
**fields_left_square_bracket_in_app_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_images_right_square_bracket** | Option<**i32**> | maximum number of related images returned (when they are included) |  |
**limit_left_square_bracket_in_app_purchase_localizations_right_square_bracket** | Option<**i32**> | maximum number of related inAppPurchaseLocalizations returned (when they are included) |  |
**limit_left_square_bracket_price_points_right_square_bracket** | Option<**i32**> | maximum number of related pricePoints returned (when they are included) |  |

### Return type

[**models::InAppPurchaseV2Response**](InAppPurchaseV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_iap_price_schedule_get_to_one_related

> models::InAppPurchasePriceScheduleResponse in_app_purchases_v2_iap_price_schedule_get_to_one_related(id, fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_in_app_purchase_prices_right_square_bracket, include, limit_left_square_bracket_manual_prices_right_square_bracket, limit_left_square_bracket_automatic_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_price_schedules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePriceSchedules |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_in_app_purchase_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePrices |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_manual_prices_right_square_bracket** | Option<**i32**> | maximum number of related manualPrices returned (when they are included) |  |
**limit_left_square_bracket_automatic_prices_right_square_bracket** | Option<**i32**> | maximum number of related automaticPrices returned (when they are included) |  |

### Return type

[**models::InAppPurchasePriceScheduleResponse**](InAppPurchasePriceScheduleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_images_get_to_many_related

> models::InAppPurchaseImagesResponse in_app_purchases_v2_images_get_to_many_related(id, fields_left_square_bracket_in_app_purchase_images_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseImages |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseImagesResponse**](InAppPurchaseImagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_in_app_purchase_availability_get_to_one_related

> models::InAppPurchaseAvailabilityResponse in_app_purchases_v2_in_app_purchase_availability_get_to_one_related(id, fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_available_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseAvailabilities |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_available_territories_right_square_bracket** | Option<**i32**> | maximum number of related availableTerritories returned (when they are included) |  |

### Return type

[**models::InAppPurchaseAvailabilityResponse**](InAppPurchaseAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_in_app_purchase_localizations_get_to_many_related

> models::InAppPurchaseLocalizationsResponse in_app_purchases_v2_in_app_purchase_localizations_get_to_many_related(id, fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseLocalizations |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseLocalizationsResponse**](InAppPurchaseLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_price_points_get_to_many_related

> models::InAppPurchasePricePointsResponse in_app_purchases_v2_price_points_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_in_app_purchase_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchasePricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchasePricePointsResponse**](InAppPurchasePricePointsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_promoted_purchase_get_to_one_related

> models::PromotedPurchaseResponse in_app_purchases_v2_promoted_purchase_get_to_one_related(id, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_promoted_purchase_images_right_square_bracket, include, limit_left_square_bracket_promotion_images_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_in_app_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchases |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_promoted_purchase_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchaseImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_promotion_images_right_square_bracket** | Option<**i32**> | maximum number of related promotionImages returned (when they are included) |  |

### Return type

[**models::PromotedPurchaseResponse**](PromotedPurchaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchases_v2_update_instance

> models::InAppPurchaseV2Response in_app_purchases_v2_update_instance(id, in_app_purchase_v2_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**in_app_purchase_v2_update_request** | [**InAppPurchaseV2UpdateRequest**](InAppPurchaseV2UpdateRequest.md) | InAppPurchase representation | [required] |

### Return type

[**models::InAppPurchaseV2Response**](InAppPurchaseV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


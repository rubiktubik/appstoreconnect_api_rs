# \SubscriptionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscriptions_app_store_review_screenshot_get_to_one_related**](SubscriptionsApi.md#subscriptions_app_store_review_screenshot_get_to_one_related) | **GET** /v1/subscriptions/{id}/appStoreReviewScreenshot | 
[**subscriptions_create_instance**](SubscriptionsApi.md#subscriptions_create_instance) | **POST** /v1/subscriptions | 
[**subscriptions_delete_instance**](SubscriptionsApi.md#subscriptions_delete_instance) | **DELETE** /v1/subscriptions/{id} | 
[**subscriptions_get_instance**](SubscriptionsApi.md#subscriptions_get_instance) | **GET** /v1/subscriptions/{id} | 
[**subscriptions_images_get_to_many_related**](SubscriptionsApi.md#subscriptions_images_get_to_many_related) | **GET** /v1/subscriptions/{id}/images | 
[**subscriptions_introductory_offers_delete_to_many_relationship**](SubscriptionsApi.md#subscriptions_introductory_offers_delete_to_many_relationship) | **DELETE** /v1/subscriptions/{id}/relationships/introductoryOffers | 
[**subscriptions_introductory_offers_get_to_many_related**](SubscriptionsApi.md#subscriptions_introductory_offers_get_to_many_related) | **GET** /v1/subscriptions/{id}/introductoryOffers | 
[**subscriptions_introductory_offers_get_to_many_relationship**](SubscriptionsApi.md#subscriptions_introductory_offers_get_to_many_relationship) | **GET** /v1/subscriptions/{id}/relationships/introductoryOffers | 
[**subscriptions_offer_codes_get_to_many_related**](SubscriptionsApi.md#subscriptions_offer_codes_get_to_many_related) | **GET** /v1/subscriptions/{id}/offerCodes | 
[**subscriptions_price_points_get_to_many_related**](SubscriptionsApi.md#subscriptions_price_points_get_to_many_related) | **GET** /v1/subscriptions/{id}/pricePoints | 
[**subscriptions_prices_delete_to_many_relationship**](SubscriptionsApi.md#subscriptions_prices_delete_to_many_relationship) | **DELETE** /v1/subscriptions/{id}/relationships/prices | 
[**subscriptions_prices_get_to_many_related**](SubscriptionsApi.md#subscriptions_prices_get_to_many_related) | **GET** /v1/subscriptions/{id}/prices | 
[**subscriptions_prices_get_to_many_relationship**](SubscriptionsApi.md#subscriptions_prices_get_to_many_relationship) | **GET** /v1/subscriptions/{id}/relationships/prices | 
[**subscriptions_promoted_purchase_get_to_one_related**](SubscriptionsApi.md#subscriptions_promoted_purchase_get_to_one_related) | **GET** /v1/subscriptions/{id}/promotedPurchase | 
[**subscriptions_promotional_offers_get_to_many_related**](SubscriptionsApi.md#subscriptions_promotional_offers_get_to_many_related) | **GET** /v1/subscriptions/{id}/promotionalOffers | 
[**subscriptions_subscription_availability_get_to_one_related**](SubscriptionsApi.md#subscriptions_subscription_availability_get_to_one_related) | **GET** /v1/subscriptions/{id}/subscriptionAvailability | 
[**subscriptions_subscription_localizations_get_to_many_related**](SubscriptionsApi.md#subscriptions_subscription_localizations_get_to_many_related) | **GET** /v1/subscriptions/{id}/subscriptionLocalizations | 
[**subscriptions_update_instance**](SubscriptionsApi.md#subscriptions_update_instance) | **PATCH** /v1/subscriptions/{id} | 
[**subscriptions_win_back_offers_get_to_many_related**](SubscriptionsApi.md#subscriptions_win_back_offers_get_to_many_related) | **GET** /v1/subscriptions/{id}/winBackOffers | 



## subscriptions_app_store_review_screenshot_get_to_one_related

> models::SubscriptionAppStoreReviewScreenshotResponse subscriptions_app_store_review_screenshot_get_to_one_related(id, fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAppStoreReviewScreenshots |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionAppStoreReviewScreenshotResponse**](SubscriptionAppStoreReviewScreenshotResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_create_instance

> models::SubscriptionResponse subscriptions_create_instance(subscription_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_create_request** | [**SubscriptionCreateRequest**](SubscriptionCreateRequest.md) | Subscription representation | [required] |

### Return type

[**models::SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_delete_instance

> subscriptions_delete_instance(id)


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


## subscriptions_get_instance

> models::SubscriptionResponse subscriptions_get_instance(id, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_subscription_localizations_right_square_bracket, fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket, fields_left_square_bracket_subscription_introductory_offers_right_square_bracket, fields_left_square_bracket_subscription_promotional_offers_right_square_bracket, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, fields_left_square_bracket_subscription_prices_right_square_bracket, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_subscription_availabilities_right_square_bracket, fields_left_square_bracket_win_back_offers_right_square_bracket, fields_left_square_bracket_subscription_images_right_square_bracket, include, limit_left_square_bracket_images_right_square_bracket, limit_left_square_bracket_introductory_offers_right_square_bracket, limit_left_square_bracket_offer_codes_right_square_bracket, limit_left_square_bracket_prices_right_square_bracket, limit_left_square_bracket_promotional_offers_right_square_bracket, limit_left_square_bracket_subscription_localizations_right_square_bracket, limit_left_square_bracket_win_back_offers_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_subscription_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionLocalizations |  |
**fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAppStoreReviewScreenshots |  |
**fields_left_square_bracket_subscription_introductory_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionIntroductoryOffers |  |
**fields_left_square_bracket_subscription_promotional_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPromotionalOffers |  |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**fields_left_square_bracket_subscription_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPrices |  |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_subscription_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAvailabilities |  |
**fields_left_square_bracket_win_back_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOffers |  |
**fields_left_square_bracket_subscription_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_images_right_square_bracket** | Option<**i32**> | maximum number of related images returned (when they are included) |  |
**limit_left_square_bracket_introductory_offers_right_square_bracket** | Option<**i32**> | maximum number of related introductoryOffers returned (when they are included) |  |
**limit_left_square_bracket_offer_codes_right_square_bracket** | Option<**i32**> | maximum number of related offerCodes returned (when they are included) |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |
**limit_left_square_bracket_promotional_offers_right_square_bracket** | Option<**i32**> | maximum number of related promotionalOffers returned (when they are included) |  |
**limit_left_square_bracket_subscription_localizations_right_square_bracket** | Option<**i32**> | maximum number of related subscriptionLocalizations returned (when they are included) |  |
**limit_left_square_bracket_win_back_offers_right_square_bracket** | Option<**i32**> | maximum number of related winBackOffers returned (when they are included) |  |

### Return type

[**models::SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_images_get_to_many_related

> models::SubscriptionImagesResponse subscriptions_images_get_to_many_related(id, fields_left_square_bracket_subscription_images_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionImages |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionImagesResponse**](SubscriptionImagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_introductory_offers_delete_to_many_relationship

> subscriptions_introductory_offers_delete_to_many_relationship(id, subscription_introductory_offers_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_introductory_offers_linkages_request** | [**SubscriptionIntroductoryOffersLinkagesRequest**](SubscriptionIntroductoryOffersLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_introductory_offers_get_to_many_related

> models::SubscriptionIntroductoryOffersResponse subscriptions_introductory_offers_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_introductory_offers_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_subscription_introductory_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionIntroductoryOffers |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionIntroductoryOffersResponse**](SubscriptionIntroductoryOffersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_introductory_offers_get_to_many_relationship

> models::SubscriptionIntroductoryOffersLinkagesResponse subscriptions_introductory_offers_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::SubscriptionIntroductoryOffersLinkagesResponse**](SubscriptionIntroductoryOffersLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_offer_codes_get_to_many_related

> models::SubscriptionOfferCodesResponse subscriptions_offer_codes_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket, fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket, limit, include, limit_left_square_bracket_one_time_use_codes_right_square_bracket, limit_left_square_bracket_custom_codes_right_square_bracket, limit_left_square_bracket_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by territory |  |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_subscription_offer_code_one_time_use_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeOneTimeUseCodes |  |
**fields_left_square_bracket_subscription_offer_code_custom_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodeCustomCodes |  |
**fields_left_square_bracket_subscription_offer_code_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodePrices |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_one_time_use_codes_right_square_bracket** | Option<**i32**> | maximum number of related oneTimeUseCodes returned (when they are included) |  |
**limit_left_square_bracket_custom_codes_right_square_bracket** | Option<**i32**> | maximum number of related customCodes returned (when they are included) |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |

### Return type

[**models::SubscriptionOfferCodesResponse**](SubscriptionOfferCodesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_price_points_get_to_many_related

> models::SubscriptionPricePointsResponse subscriptions_price_points_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionPricePointsResponse**](SubscriptionPricePointsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_prices_delete_to_many_relationship

> subscriptions_prices_delete_to_many_relationship(id, subscription_prices_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_prices_linkages_request** | [**SubscriptionPricesLinkagesRequest**](SubscriptionPricesLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_prices_get_to_many_related

> models::SubscriptionPricesResponse subscriptions_prices_get_to_many_related(id, filter_left_square_bracket_subscription_price_point_right_square_bracket, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_prices_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_subscription_price_point_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'subscriptionPricePoint' |  |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_subscription_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPrices |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionPricesResponse**](SubscriptionPricesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_prices_get_to_many_relationship

> models::SubscriptionPricesLinkagesResponse subscriptions_prices_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::SubscriptionPricesLinkagesResponse**](SubscriptionPricesLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_promoted_purchase_get_to_one_related

> models::PromotedPurchaseResponse subscriptions_promoted_purchase_get_to_one_related(id, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_in_app_purchases_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_promoted_purchase_images_right_square_bracket, include, limit_left_square_bracket_promotion_images_right_square_bracket)


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


## subscriptions_promotional_offers_get_to_many_related

> models::SubscriptionPromotionalOffersResponse subscriptions_promotional_offers_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_subscription_promotional_offers_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_subscription_promotional_offer_prices_right_square_bracket, limit, include, limit_left_square_bracket_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by territory |  |
**fields_left_square_bracket_subscription_promotional_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPromotionalOffers |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_subscription_promotional_offer_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPromotionalOfferPrices |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |

### Return type

[**models::SubscriptionPromotionalOffersResponse**](SubscriptionPromotionalOffersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_subscription_availability_get_to_one_related

> models::SubscriptionAvailabilityResponse subscriptions_subscription_availability_get_to_one_related(id, fields_left_square_bracket_subscription_availabilities_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_available_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAvailabilities |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_available_territories_right_square_bracket** | Option<**i32**> | maximum number of related availableTerritories returned (when they are included) |  |

### Return type

[**models::SubscriptionAvailabilityResponse**](SubscriptionAvailabilityResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_subscription_localizations_get_to_many_related

> models::SubscriptionLocalizationsResponse subscriptions_subscription_localizations_get_to_many_related(id, fields_left_square_bracket_subscription_localizations_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionLocalizations |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionLocalizationsResponse**](SubscriptionLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_update_instance

> models::SubscriptionResponse subscriptions_update_instance(id, subscription_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_update_request** | [**SubscriptionUpdateRequest**](SubscriptionUpdateRequest.md) | Subscription representation | [required] |

### Return type

[**models::SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_win_back_offers_get_to_many_related

> models::WinBackOffersResponse subscriptions_win_back_offers_get_to_many_related(id, fields_left_square_bracket_win_back_offers_right_square_bracket, fields_left_square_bracket_win_back_offer_prices_right_square_bracket, limit, include, limit_left_square_bracket_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_win_back_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOffers |  |
**fields_left_square_bracket_win_back_offer_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOfferPrices |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |

### Return type

[**models::WinBackOffersResponse**](WinBackOffersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


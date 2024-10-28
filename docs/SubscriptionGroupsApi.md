# \SubscriptionGroupsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_groups_create_instance**](SubscriptionGroupsApi.md#subscription_groups_create_instance) | **POST** /v1/subscriptionGroups | 
[**subscription_groups_delete_instance**](SubscriptionGroupsApi.md#subscription_groups_delete_instance) | **DELETE** /v1/subscriptionGroups/{id} | 
[**subscription_groups_get_instance**](SubscriptionGroupsApi.md#subscription_groups_get_instance) | **GET** /v1/subscriptionGroups/{id} | 
[**subscription_groups_subscription_group_localizations_get_to_many_related**](SubscriptionGroupsApi.md#subscription_groups_subscription_group_localizations_get_to_many_related) | **GET** /v1/subscriptionGroups/{id}/subscriptionGroupLocalizations | 
[**subscription_groups_subscriptions_get_to_many_related**](SubscriptionGroupsApi.md#subscription_groups_subscriptions_get_to_many_related) | **GET** /v1/subscriptionGroups/{id}/subscriptions | 
[**subscription_groups_update_instance**](SubscriptionGroupsApi.md#subscription_groups_update_instance) | **PATCH** /v1/subscriptionGroups/{id} | 



## subscription_groups_create_instance

> models::SubscriptionGroupResponse subscription_groups_create_instance(subscription_group_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_group_create_request** | [**SubscriptionGroupCreateRequest**](SubscriptionGroupCreateRequest.md) | SubscriptionGroup representation | [required] |

### Return type

[**models::SubscriptionGroupResponse**](SubscriptionGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_groups_delete_instance

> subscription_groups_delete_instance(id)


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


## subscription_groups_get_instance

> models::SubscriptionGroupResponse subscription_groups_get_instance(id, fields_left_square_bracket_subscription_groups_right_square_bracket, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_subscription_group_localizations_right_square_bracket, include, limit_left_square_bracket_subscription_group_localizations_right_square_bracket, limit_left_square_bracket_subscriptions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroups |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_subscription_group_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroupLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_subscription_group_localizations_right_square_bracket** | Option<**i32**> | maximum number of related subscriptionGroupLocalizations returned (when they are included) |  |
**limit_left_square_bracket_subscriptions_right_square_bracket** | Option<**i32**> | maximum number of related subscriptions returned (when they are included) |  |

### Return type

[**models::SubscriptionGroupResponse**](SubscriptionGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_groups_subscription_group_localizations_get_to_many_related

> models::SubscriptionGroupLocalizationsResponse subscription_groups_subscription_group_localizations_get_to_many_related(id, fields_left_square_bracket_subscription_group_localizations_right_square_bracket, fields_left_square_bracket_subscription_groups_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_subscription_group_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroupLocalizations |  |
**fields_left_square_bracket_subscription_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroups |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::SubscriptionGroupLocalizationsResponse**](SubscriptionGroupLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_groups_subscriptions_get_to_many_related

> models::SubscriptionsResponse subscription_groups_subscriptions_get_to_many_related(id, filter_left_square_bracket_product_id_right_square_bracket, filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_state_right_square_bracket, sort, fields_left_square_bracket_subscriptions_right_square_bracket, fields_left_square_bracket_subscription_localizations_right_square_bracket, fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket, fields_left_square_bracket_subscription_groups_right_square_bracket, fields_left_square_bracket_subscription_introductory_offers_right_square_bracket, fields_left_square_bracket_subscription_promotional_offers_right_square_bracket, fields_left_square_bracket_subscription_offer_codes_right_square_bracket, fields_left_square_bracket_subscription_prices_right_square_bracket, fields_left_square_bracket_promoted_purchases_right_square_bracket, fields_left_square_bracket_subscription_availabilities_right_square_bracket, fields_left_square_bracket_win_back_offers_right_square_bracket, fields_left_square_bracket_subscription_images_right_square_bracket, limit, include, limit_left_square_bracket_subscription_localizations_right_square_bracket, limit_left_square_bracket_introductory_offers_right_square_bracket, limit_left_square_bracket_promotional_offers_right_square_bracket, limit_left_square_bracket_offer_codes_right_square_bracket, limit_left_square_bracket_prices_right_square_bracket, limit_left_square_bracket_win_back_offers_right_square_bracket, limit_left_square_bracket_images_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_product_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'productId' |  |
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_subscriptions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptions |  |
**fields_left_square_bracket_subscription_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionLocalizations |  |
**fields_left_square_bracket_subscription_app_store_review_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAppStoreReviewScreenshots |  |
**fields_left_square_bracket_subscription_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionGroups |  |
**fields_left_square_bracket_subscription_introductory_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionIntroductoryOffers |  |
**fields_left_square_bracket_subscription_promotional_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPromotionalOffers |  |
**fields_left_square_bracket_subscription_offer_codes_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionOfferCodes |  |
**fields_left_square_bracket_subscription_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPrices |  |
**fields_left_square_bracket_promoted_purchases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type promotedPurchases |  |
**fields_left_square_bracket_subscription_availabilities_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionAvailabilities |  |
**fields_left_square_bracket_win_back_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOffers |  |
**fields_left_square_bracket_subscription_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionImages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_subscription_localizations_right_square_bracket** | Option<**i32**> | maximum number of related subscriptionLocalizations returned (when they are included) |  |
**limit_left_square_bracket_introductory_offers_right_square_bracket** | Option<**i32**> | maximum number of related introductoryOffers returned (when they are included) |  |
**limit_left_square_bracket_promotional_offers_right_square_bracket** | Option<**i32**> | maximum number of related promotionalOffers returned (when they are included) |  |
**limit_left_square_bracket_offer_codes_right_square_bracket** | Option<**i32**> | maximum number of related offerCodes returned (when they are included) |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |
**limit_left_square_bracket_win_back_offers_right_square_bracket** | Option<**i32**> | maximum number of related winBackOffers returned (when they are included) |  |
**limit_left_square_bracket_images_right_square_bracket** | Option<**i32**> | maximum number of related images returned (when they are included) |  |

### Return type

[**models::SubscriptionsResponse**](SubscriptionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_groups_update_instance

> models::SubscriptionGroupResponse subscription_groups_update_instance(id, subscription_group_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_group_update_request** | [**SubscriptionGroupUpdateRequest**](SubscriptionGroupUpdateRequest.md) | SubscriptionGroup representation | [required] |

### Return type

[**models::SubscriptionGroupResponse**](SubscriptionGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


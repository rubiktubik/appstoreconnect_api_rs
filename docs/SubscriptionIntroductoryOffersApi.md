# \SubscriptionIntroductoryOffersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscription_introductory_offers_create_instance**](SubscriptionIntroductoryOffersApi.md#subscription_introductory_offers_create_instance) | **POST** /v1/subscriptionIntroductoryOffers | 
[**subscription_introductory_offers_delete_instance**](SubscriptionIntroductoryOffersApi.md#subscription_introductory_offers_delete_instance) | **DELETE** /v1/subscriptionIntroductoryOffers/{id} | 
[**subscription_introductory_offers_update_instance**](SubscriptionIntroductoryOffersApi.md#subscription_introductory_offers_update_instance) | **PATCH** /v1/subscriptionIntroductoryOffers/{id} | 



## subscription_introductory_offers_create_instance

> models::SubscriptionIntroductoryOfferResponse subscription_introductory_offers_create_instance(subscription_introductory_offer_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_introductory_offer_create_request** | [**SubscriptionIntroductoryOfferCreateRequest**](SubscriptionIntroductoryOfferCreateRequest.md) | SubscriptionIntroductoryOffer representation | [required] |

### Return type

[**models::SubscriptionIntroductoryOfferResponse**](SubscriptionIntroductoryOfferResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscription_introductory_offers_delete_instance

> subscription_introductory_offers_delete_instance(id)


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


## subscription_introductory_offers_update_instance

> models::SubscriptionIntroductoryOfferResponse subscription_introductory_offers_update_instance(id, subscription_introductory_offer_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**subscription_introductory_offer_update_request** | [**SubscriptionIntroductoryOfferUpdateRequest**](SubscriptionIntroductoryOfferUpdateRequest.md) | SubscriptionIntroductoryOffer representation | [required] |

### Return type

[**models::SubscriptionIntroductoryOfferResponse**](SubscriptionIntroductoryOfferResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


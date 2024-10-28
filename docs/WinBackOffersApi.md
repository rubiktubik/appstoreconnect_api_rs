# \WinBackOffersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**win_back_offers_create_instance**](WinBackOffersApi.md#win_back_offers_create_instance) | **POST** /v1/winBackOffers | 
[**win_back_offers_delete_instance**](WinBackOffersApi.md#win_back_offers_delete_instance) | **DELETE** /v1/winBackOffers/{id} | 
[**win_back_offers_get_instance**](WinBackOffersApi.md#win_back_offers_get_instance) | **GET** /v1/winBackOffers/{id} | 
[**win_back_offers_prices_get_to_many_related**](WinBackOffersApi.md#win_back_offers_prices_get_to_many_related) | **GET** /v1/winBackOffers/{id}/prices | 
[**win_back_offers_update_instance**](WinBackOffersApi.md#win_back_offers_update_instance) | **PATCH** /v1/winBackOffers/{id} | 



## win_back_offers_create_instance

> models::WinBackOfferResponse win_back_offers_create_instance(win_back_offer_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**win_back_offer_create_request** | [**WinBackOfferCreateRequest**](WinBackOfferCreateRequest.md) | WinBackOffer representation | [required] |

### Return type

[**models::WinBackOfferResponse**](WinBackOfferResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## win_back_offers_delete_instance

> win_back_offers_delete_instance(id)


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


## win_back_offers_get_instance

> models::WinBackOfferResponse win_back_offers_get_instance(id, fields_left_square_bracket_win_back_offers_right_square_bracket, fields_left_square_bracket_win_back_offer_prices_right_square_bracket, include, limit_left_square_bracket_prices_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_win_back_offers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOffers |  |
**fields_left_square_bracket_win_back_offer_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOfferPrices |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_prices_right_square_bracket** | Option<**i32**> | maximum number of related prices returned (when they are included) |  |

### Return type

[**models::WinBackOfferResponse**](WinBackOfferResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## win_back_offers_prices_get_to_many_related

> models::WinBackOfferPricesResponse win_back_offers_prices_get_to_many_related(id, filter_left_square_bracket_territory_right_square_bracket, fields_left_square_bracket_win_back_offer_prices_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, fields_left_square_bracket_subscription_price_points_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_territory_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'territory' |  |
**fields_left_square_bracket_win_back_offer_prices_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type winBackOfferPrices |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**fields_left_square_bracket_subscription_price_points_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type subscriptionPricePoints |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::WinBackOfferPricesResponse**](WinBackOfferPricesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## win_back_offers_update_instance

> models::WinBackOfferResponse win_back_offers_update_instance(id, win_back_offer_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**win_back_offer_update_request** | [**WinBackOfferUpdateRequest**](WinBackOfferUpdateRequest.md) | WinBackOffer representation | [required] |

### Return type

[**models::WinBackOfferResponse**](WinBackOfferResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


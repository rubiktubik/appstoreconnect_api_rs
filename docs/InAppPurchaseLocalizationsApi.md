# \InAppPurchaseLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**in_app_purchase_localizations_create_instance**](InAppPurchaseLocalizationsApi.md#in_app_purchase_localizations_create_instance) | **POST** /v1/inAppPurchaseLocalizations | 
[**in_app_purchase_localizations_delete_instance**](InAppPurchaseLocalizationsApi.md#in_app_purchase_localizations_delete_instance) | **DELETE** /v1/inAppPurchaseLocalizations/{id} | 
[**in_app_purchase_localizations_get_instance**](InAppPurchaseLocalizationsApi.md#in_app_purchase_localizations_get_instance) | **GET** /v1/inAppPurchaseLocalizations/{id} | 
[**in_app_purchase_localizations_update_instance**](InAppPurchaseLocalizationsApi.md#in_app_purchase_localizations_update_instance) | **PATCH** /v1/inAppPurchaseLocalizations/{id} | 



## in_app_purchase_localizations_create_instance

> models::InAppPurchaseLocalizationResponse in_app_purchase_localizations_create_instance(in_app_purchase_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_app_purchase_localization_create_request** | [**InAppPurchaseLocalizationCreateRequest**](InAppPurchaseLocalizationCreateRequest.md) | InAppPurchaseLocalization representation | [required] |

### Return type

[**models::InAppPurchaseLocalizationResponse**](InAppPurchaseLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_localizations_delete_instance

> in_app_purchase_localizations_delete_instance(id)


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


## in_app_purchase_localizations_get_instance

> models::InAppPurchaseLocalizationResponse in_app_purchase_localizations_get_instance(id, fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_in_app_purchase_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type inAppPurchaseLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::InAppPurchaseLocalizationResponse**](InAppPurchaseLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## in_app_purchase_localizations_update_instance

> models::InAppPurchaseLocalizationResponse in_app_purchase_localizations_update_instance(id, in_app_purchase_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**in_app_purchase_localization_update_request** | [**InAppPurchaseLocalizationUpdateRequest**](InAppPurchaseLocalizationUpdateRequest.md) | InAppPurchaseLocalization representation | [required] |

### Return type

[**models::InAppPurchaseLocalizationResponse**](InAppPurchaseLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


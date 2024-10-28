# \BetaAppLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_app_localizations_app_get_to_one_related**](BetaAppLocalizationsApi.md#beta_app_localizations_app_get_to_one_related) | **GET** /v1/betaAppLocalizations/{id}/app | 
[**beta_app_localizations_create_instance**](BetaAppLocalizationsApi.md#beta_app_localizations_create_instance) | **POST** /v1/betaAppLocalizations | 
[**beta_app_localizations_delete_instance**](BetaAppLocalizationsApi.md#beta_app_localizations_delete_instance) | **DELETE** /v1/betaAppLocalizations/{id} | 
[**beta_app_localizations_get_collection**](BetaAppLocalizationsApi.md#beta_app_localizations_get_collection) | **GET** /v1/betaAppLocalizations | 
[**beta_app_localizations_get_instance**](BetaAppLocalizationsApi.md#beta_app_localizations_get_instance) | **GET** /v1/betaAppLocalizations/{id} | 
[**beta_app_localizations_update_instance**](BetaAppLocalizationsApi.md#beta_app_localizations_update_instance) | **PATCH** /v1/betaAppLocalizations/{id} | 



## beta_app_localizations_app_get_to_one_related

> models::AppWithoutIncludesResponse beta_app_localizations_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |

### Return type

[**models::AppWithoutIncludesResponse**](AppWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_localizations_create_instance

> models::BetaAppLocalizationResponse beta_app_localizations_create_instance(beta_app_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_app_localization_create_request** | [**BetaAppLocalizationCreateRequest**](BetaAppLocalizationCreateRequest.md) | BetaAppLocalization representation | [required] |

### Return type

[**models::BetaAppLocalizationResponse**](BetaAppLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_localizations_delete_instance

> beta_app_localizations_delete_instance(id)


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


## beta_app_localizations_get_collection

> models::BetaAppLocalizationsResponse beta_app_localizations_get_collection(filter_left_square_bracket_locale_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, fields_left_square_bracket_beta_app_localizations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**fields_left_square_bracket_beta_app_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppLocalizations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppLocalizationsResponse**](BetaAppLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_localizations_get_instance

> models::BetaAppLocalizationResponse beta_app_localizations_get_instance(id, fields_left_square_bracket_beta_app_localizations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_app_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaAppLocalizations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaAppLocalizationResponse**](BetaAppLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_app_localizations_update_instance

> models::BetaAppLocalizationResponse beta_app_localizations_update_instance(id, beta_app_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_app_localization_update_request** | [**BetaAppLocalizationUpdateRequest**](BetaAppLocalizationUpdateRequest.md) | BetaAppLocalization representation | [required] |

### Return type

[**models::BetaAppLocalizationResponse**](BetaAppLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


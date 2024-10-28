# \AppInfoLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_info_localizations_create_instance**](AppInfoLocalizationsApi.md#app_info_localizations_create_instance) | **POST** /v1/appInfoLocalizations | 
[**app_info_localizations_delete_instance**](AppInfoLocalizationsApi.md#app_info_localizations_delete_instance) | **DELETE** /v1/appInfoLocalizations/{id} | 
[**app_info_localizations_get_instance**](AppInfoLocalizationsApi.md#app_info_localizations_get_instance) | **GET** /v1/appInfoLocalizations/{id} | 
[**app_info_localizations_update_instance**](AppInfoLocalizationsApi.md#app_info_localizations_update_instance) | **PATCH** /v1/appInfoLocalizations/{id} | 



## app_info_localizations_create_instance

> models::AppInfoLocalizationResponse app_info_localizations_create_instance(app_info_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_info_localization_create_request** | [**AppInfoLocalizationCreateRequest**](AppInfoLocalizationCreateRequest.md) | AppInfoLocalization representation | [required] |

### Return type

[**models::AppInfoLocalizationResponse**](AppInfoLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_info_localizations_delete_instance

> app_info_localizations_delete_instance(id)


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


## app_info_localizations_get_instance

> models::AppInfoLocalizationResponse app_info_localizations_get_instance(id, fields_left_square_bracket_app_info_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_info_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfoLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppInfoLocalizationResponse**](AppInfoLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_info_localizations_update_instance

> models::AppInfoLocalizationResponse app_info_localizations_update_instance(id, app_info_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_info_localization_update_request** | [**AppInfoLocalizationUpdateRequest**](AppInfoLocalizationUpdateRequest.md) | AppInfoLocalization representation | [required] |

### Return type

[**models::AppInfoLocalizationResponse**](AppInfoLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


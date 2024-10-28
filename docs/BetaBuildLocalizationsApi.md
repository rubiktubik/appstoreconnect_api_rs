# \BetaBuildLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_build_localizations_build_get_to_one_related**](BetaBuildLocalizationsApi.md#beta_build_localizations_build_get_to_one_related) | **GET** /v1/betaBuildLocalizations/{id}/build | 
[**beta_build_localizations_create_instance**](BetaBuildLocalizationsApi.md#beta_build_localizations_create_instance) | **POST** /v1/betaBuildLocalizations | 
[**beta_build_localizations_delete_instance**](BetaBuildLocalizationsApi.md#beta_build_localizations_delete_instance) | **DELETE** /v1/betaBuildLocalizations/{id} | 
[**beta_build_localizations_get_collection**](BetaBuildLocalizationsApi.md#beta_build_localizations_get_collection) | **GET** /v1/betaBuildLocalizations | 
[**beta_build_localizations_get_instance**](BetaBuildLocalizationsApi.md#beta_build_localizations_get_instance) | **GET** /v1/betaBuildLocalizations/{id} | 
[**beta_build_localizations_update_instance**](BetaBuildLocalizationsApi.md#beta_build_localizations_update_instance) | **PATCH** /v1/betaBuildLocalizations/{id} | 



## beta_build_localizations_build_get_to_one_related

> models::BuildWithoutIncludesResponse beta_build_localizations_build_get_to_one_related(id, fields_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |

### Return type

[**models::BuildWithoutIncludesResponse**](BuildWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_build_localizations_create_instance

> models::BetaBuildLocalizationResponse beta_build_localizations_create_instance(beta_build_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_build_localization_create_request** | [**BetaBuildLocalizationCreateRequest**](BetaBuildLocalizationCreateRequest.md) | BetaBuildLocalization representation | [required] |

### Return type

[**models::BetaBuildLocalizationResponse**](BetaBuildLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_build_localizations_delete_instance

> beta_build_localizations_delete_instance(id)


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


## beta_build_localizations_get_collection

> models::BetaBuildLocalizationsResponse beta_build_localizations_get_collection(filter_left_square_bracket_locale_right_square_bracket, filter_left_square_bracket_build_right_square_bracket, fields_left_square_bracket_beta_build_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**filter_left_square_bracket_build_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'build' |  |
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaBuildLocalizationsResponse**](BetaBuildLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_build_localizations_get_instance

> models::BetaBuildLocalizationResponse beta_build_localizations_get_instance(id, fields_left_square_bracket_beta_build_localizations_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_build_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaBuildLocalizations |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::BetaBuildLocalizationResponse**](BetaBuildLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_build_localizations_update_instance

> models::BetaBuildLocalizationResponse beta_build_localizations_update_instance(id, beta_build_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_build_localization_update_request** | [**BetaBuildLocalizationUpdateRequest**](BetaBuildLocalizationUpdateRequest.md) | BetaBuildLocalization representation | [required] |

### Return type

[**models::BetaBuildLocalizationResponse**](BetaBuildLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


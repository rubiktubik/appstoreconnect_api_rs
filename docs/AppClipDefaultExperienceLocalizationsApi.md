# \AppClipDefaultExperienceLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clip_default_experience_localizations_app_clip_header_image_get_to_one_related**](AppClipDefaultExperienceLocalizationsApi.md#app_clip_default_experience_localizations_app_clip_header_image_get_to_one_related) | **GET** /v1/appClipDefaultExperienceLocalizations/{id}/appClipHeaderImage | 
[**app_clip_default_experience_localizations_create_instance**](AppClipDefaultExperienceLocalizationsApi.md#app_clip_default_experience_localizations_create_instance) | **POST** /v1/appClipDefaultExperienceLocalizations | 
[**app_clip_default_experience_localizations_delete_instance**](AppClipDefaultExperienceLocalizationsApi.md#app_clip_default_experience_localizations_delete_instance) | **DELETE** /v1/appClipDefaultExperienceLocalizations/{id} | 
[**app_clip_default_experience_localizations_get_instance**](AppClipDefaultExperienceLocalizationsApi.md#app_clip_default_experience_localizations_get_instance) | **GET** /v1/appClipDefaultExperienceLocalizations/{id} | 
[**app_clip_default_experience_localizations_update_instance**](AppClipDefaultExperienceLocalizationsApi.md#app_clip_default_experience_localizations_update_instance) | **PATCH** /v1/appClipDefaultExperienceLocalizations/{id} | 



## app_clip_default_experience_localizations_app_clip_header_image_get_to_one_related

> models::AppClipHeaderImageResponse app_clip_default_experience_localizations_app_clip_header_image_get_to_one_related(id, fields_left_square_bracket_app_clip_header_images_right_square_bracket, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_header_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipHeaderImages |  |
**fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperienceLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipHeaderImageResponse**](AppClipHeaderImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experience_localizations_create_instance

> models::AppClipDefaultExperienceLocalizationResponse app_clip_default_experience_localizations_create_instance(app_clip_default_experience_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_clip_default_experience_localization_create_request** | [**AppClipDefaultExperienceLocalizationCreateRequest**](AppClipDefaultExperienceLocalizationCreateRequest.md) | AppClipDefaultExperienceLocalization representation | [required] |

### Return type

[**models::AppClipDefaultExperienceLocalizationResponse**](AppClipDefaultExperienceLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experience_localizations_delete_instance

> app_clip_default_experience_localizations_delete_instance(id)


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


## app_clip_default_experience_localizations_get_instance

> models::AppClipDefaultExperienceLocalizationResponse app_clip_default_experience_localizations_get_instance(id, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, fields_left_square_bracket_app_clip_header_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperienceLocalizations |  |
**fields_left_square_bracket_app_clip_header_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipHeaderImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppClipDefaultExperienceLocalizationResponse**](AppClipDefaultExperienceLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_default_experience_localizations_update_instance

> models::AppClipDefaultExperienceLocalizationResponse app_clip_default_experience_localizations_update_instance(id, app_clip_default_experience_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_default_experience_localization_update_request** | [**AppClipDefaultExperienceLocalizationUpdateRequest**](AppClipDefaultExperienceLocalizationUpdateRequest.md) | AppClipDefaultExperienceLocalization representation | [required] |

### Return type

[**models::AppClipDefaultExperienceLocalizationResponse**](AppClipDefaultExperienceLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


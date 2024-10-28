# \AppClipAdvancedExperiencesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clip_advanced_experiences_create_instance**](AppClipAdvancedExperiencesApi.md#app_clip_advanced_experiences_create_instance) | **POST** /v1/appClipAdvancedExperiences | 
[**app_clip_advanced_experiences_get_instance**](AppClipAdvancedExperiencesApi.md#app_clip_advanced_experiences_get_instance) | **GET** /v1/appClipAdvancedExperiences/{id} | 
[**app_clip_advanced_experiences_update_instance**](AppClipAdvancedExperiencesApi.md#app_clip_advanced_experiences_update_instance) | **PATCH** /v1/appClipAdvancedExperiences/{id} | 



## app_clip_advanced_experiences_create_instance

> models::AppClipAdvancedExperienceResponse app_clip_advanced_experiences_create_instance(app_clip_advanced_experience_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_clip_advanced_experience_create_request** | [**AppClipAdvancedExperienceCreateRequest**](AppClipAdvancedExperienceCreateRequest.md) | AppClipAdvancedExperience representation | [required] |

### Return type

[**models::AppClipAdvancedExperienceResponse**](AppClipAdvancedExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_advanced_experiences_get_instance

> models::AppClipAdvancedExperienceResponse app_clip_advanced_experiences_get_instance(id, fields_left_square_bracket_app_clip_advanced_experiences_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clip_advanced_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAdvancedExperiences |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |

### Return type

[**models::AppClipAdvancedExperienceResponse**](AppClipAdvancedExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clip_advanced_experiences_update_instance

> models::AppClipAdvancedExperienceResponse app_clip_advanced_experiences_update_instance(id, app_clip_advanced_experience_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_clip_advanced_experience_update_request** | [**AppClipAdvancedExperienceUpdateRequest**](AppClipAdvancedExperienceUpdateRequest.md) | AppClipAdvancedExperience representation | [required] |

### Return type

[**models::AppClipAdvancedExperienceResponse**](AppClipAdvancedExperienceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


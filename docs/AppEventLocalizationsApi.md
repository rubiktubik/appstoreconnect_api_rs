# \AppEventLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_event_localizations_app_event_screenshots_get_to_many_related**](AppEventLocalizationsApi.md#app_event_localizations_app_event_screenshots_get_to_many_related) | **GET** /v1/appEventLocalizations/{id}/appEventScreenshots | 
[**app_event_localizations_app_event_video_clips_get_to_many_related**](AppEventLocalizationsApi.md#app_event_localizations_app_event_video_clips_get_to_many_related) | **GET** /v1/appEventLocalizations/{id}/appEventVideoClips | 
[**app_event_localizations_create_instance**](AppEventLocalizationsApi.md#app_event_localizations_create_instance) | **POST** /v1/appEventLocalizations | 
[**app_event_localizations_delete_instance**](AppEventLocalizationsApi.md#app_event_localizations_delete_instance) | **DELETE** /v1/appEventLocalizations/{id} | 
[**app_event_localizations_get_instance**](AppEventLocalizationsApi.md#app_event_localizations_get_instance) | **GET** /v1/appEventLocalizations/{id} | 
[**app_event_localizations_update_instance**](AppEventLocalizationsApi.md#app_event_localizations_update_instance) | **PATCH** /v1/appEventLocalizations/{id} | 



## app_event_localizations_app_event_screenshots_get_to_many_related

> models::AppEventScreenshotsResponse app_event_localizations_app_event_screenshots_get_to_many_related(id, fields_left_square_bracket_app_event_screenshots_right_square_bracket, fields_left_square_bracket_app_event_localizations_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_event_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventScreenshots |  |
**fields_left_square_bracket_app_event_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppEventScreenshotsResponse**](AppEventScreenshotsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_localizations_app_event_video_clips_get_to_many_related

> models::AppEventVideoClipsResponse app_event_localizations_app_event_video_clips_get_to_many_related(id, fields_left_square_bracket_app_event_video_clips_right_square_bracket, fields_left_square_bracket_app_event_localizations_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventVideoClips |  |
**fields_left_square_bracket_app_event_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppEventVideoClipsResponse**](AppEventVideoClipsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_localizations_create_instance

> models::AppEventLocalizationResponse app_event_localizations_create_instance(app_event_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_event_localization_create_request** | [**AppEventLocalizationCreateRequest**](AppEventLocalizationCreateRequest.md) | AppEventLocalization representation | [required] |

### Return type

[**models::AppEventLocalizationResponse**](AppEventLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_localizations_delete_instance

> app_event_localizations_delete_instance(id)


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


## app_event_localizations_get_instance

> models::AppEventLocalizationResponse app_event_localizations_get_instance(id, fields_left_square_bracket_app_event_localizations_right_square_bracket, fields_left_square_bracket_app_event_screenshots_right_square_bracket, fields_left_square_bracket_app_event_video_clips_right_square_bracket, include, limit_left_square_bracket_app_event_screenshots_right_square_bracket, limit_left_square_bracket_app_event_video_clips_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_event_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventLocalizations |  |
**fields_left_square_bracket_app_event_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventScreenshots |  |
**fields_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventVideoClips |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_event_screenshots_right_square_bracket** | Option<**i32**> | maximum number of related appEventScreenshots returned (when they are included) |  |
**limit_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<**i32**> | maximum number of related appEventVideoClips returned (when they are included) |  |

### Return type

[**models::AppEventLocalizationResponse**](AppEventLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_localizations_update_instance

> models::AppEventLocalizationResponse app_event_localizations_update_instance(id, app_event_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_event_localization_update_request** | [**AppEventLocalizationUpdateRequest**](AppEventLocalizationUpdateRequest.md) | AppEventLocalization representation | [required] |

### Return type

[**models::AppEventLocalizationResponse**](AppEventLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AppEventsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_events_create_instance**](AppEventsApi.md#app_events_create_instance) | **POST** /v1/appEvents | 
[**app_events_delete_instance**](AppEventsApi.md#app_events_delete_instance) | **DELETE** /v1/appEvents/{id} | 
[**app_events_get_instance**](AppEventsApi.md#app_events_get_instance) | **GET** /v1/appEvents/{id} | 
[**app_events_localizations_get_to_many_related**](AppEventsApi.md#app_events_localizations_get_to_many_related) | **GET** /v1/appEvents/{id}/localizations | 
[**app_events_update_instance**](AppEventsApi.md#app_events_update_instance) | **PATCH** /v1/appEvents/{id} | 



## app_events_create_instance

> models::AppEventResponse app_events_create_instance(app_event_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_event_create_request** | [**AppEventCreateRequest**](AppEventCreateRequest.md) | AppEvent representation | [required] |

### Return type

[**models::AppEventResponse**](AppEventResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_events_delete_instance

> app_events_delete_instance(id)


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


## app_events_get_instance

> models::AppEventResponse app_events_get_instance(id, fields_left_square_bracket_app_events_right_square_bracket, fields_left_square_bracket_app_event_localizations_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_events_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEvents |  |
**fields_left_square_bracket_app_event_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |

### Return type

[**models::AppEventResponse**](AppEventResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_events_localizations_get_to_many_related

> models::AppEventLocalizationsResponse app_events_localizations_get_to_many_related(id, fields_left_square_bracket_app_event_localizations_right_square_bracket, fields_left_square_bracket_app_events_right_square_bracket, fields_left_square_bracket_app_event_screenshots_right_square_bracket, fields_left_square_bracket_app_event_video_clips_right_square_bracket, limit, include, limit_left_square_bracket_app_event_screenshots_right_square_bracket, limit_left_square_bracket_app_event_video_clips_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_event_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventLocalizations |  |
**fields_left_square_bracket_app_events_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEvents |  |
**fields_left_square_bracket_app_event_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventScreenshots |  |
**fields_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventVideoClips |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_event_screenshots_right_square_bracket** | Option<**i32**> | maximum number of related appEventScreenshots returned (when they are included) |  |
**limit_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<**i32**> | maximum number of related appEventVideoClips returned (when they are included) |  |

### Return type

[**models::AppEventLocalizationsResponse**](AppEventLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_events_update_instance

> models::AppEventResponse app_events_update_instance(id, app_event_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_event_update_request** | [**AppEventUpdateRequest**](AppEventUpdateRequest.md) | AppEvent representation | [required] |

### Return type

[**models::AppEventResponse**](AppEventResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


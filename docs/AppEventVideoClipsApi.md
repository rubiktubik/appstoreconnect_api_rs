# \AppEventVideoClipsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_event_video_clips_create_instance**](AppEventVideoClipsApi.md#app_event_video_clips_create_instance) | **POST** /v1/appEventVideoClips | 
[**app_event_video_clips_delete_instance**](AppEventVideoClipsApi.md#app_event_video_clips_delete_instance) | **DELETE** /v1/appEventVideoClips/{id} | 
[**app_event_video_clips_get_instance**](AppEventVideoClipsApi.md#app_event_video_clips_get_instance) | **GET** /v1/appEventVideoClips/{id} | 
[**app_event_video_clips_update_instance**](AppEventVideoClipsApi.md#app_event_video_clips_update_instance) | **PATCH** /v1/appEventVideoClips/{id} | 



## app_event_video_clips_create_instance

> models::AppEventVideoClipResponse app_event_video_clips_create_instance(app_event_video_clip_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_event_video_clip_create_request** | [**AppEventVideoClipCreateRequest**](AppEventVideoClipCreateRequest.md) | AppEventVideoClip representation | [required] |

### Return type

[**models::AppEventVideoClipResponse**](AppEventVideoClipResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_video_clips_delete_instance

> app_event_video_clips_delete_instance(id)


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


## app_event_video_clips_get_instance

> models::AppEventVideoClipResponse app_event_video_clips_get_instance(id, fields_left_square_bracket_app_event_video_clips_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_event_video_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEventVideoClips |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppEventVideoClipResponse**](AppEventVideoClipResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_event_video_clips_update_instance

> models::AppEventVideoClipResponse app_event_video_clips_update_instance(id, app_event_video_clip_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_event_video_clip_update_request** | [**AppEventVideoClipUpdateRequest**](AppEventVideoClipUpdateRequest.md) | AppEventVideoClip representation | [required] |

### Return type

[**models::AppEventVideoClipResponse**](AppEventVideoClipResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


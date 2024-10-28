# \AppClipsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_clips_app_clip_advanced_experiences_get_to_many_related**](AppClipsApi.md#app_clips_app_clip_advanced_experiences_get_to_many_related) | **GET** /v1/appClips/{id}/appClipAdvancedExperiences | 
[**app_clips_app_clip_default_experiences_get_to_many_related**](AppClipsApi.md#app_clips_app_clip_default_experiences_get_to_many_related) | **GET** /v1/appClips/{id}/appClipDefaultExperiences | 
[**app_clips_get_instance**](AppClipsApi.md#app_clips_get_instance) | **GET** /v1/appClips/{id} | 



## app_clips_app_clip_advanced_experiences_get_to_many_related

> models::AppClipAdvancedExperiencesResponse app_clips_app_clip_advanced_experiences_get_to_many_related(id, filter_left_square_bracket_status_right_square_bracket, filter_left_square_bracket_place_status_right_square_bracket, filter_left_square_bracket_action_right_square_bracket, fields_left_square_bracket_app_clip_advanced_experiences_right_square_bracket, fields_left_square_bracket_app_clips_right_square_bracket, fields_left_square_bracket_app_clip_advanced_experience_images_right_square_bracket, fields_left_square_bracket_app_clip_advanced_experience_localizations_right_square_bracket, limit, include, limit_left_square_bracket_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_status_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'status' |  |
**filter_left_square_bracket_place_status_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'placeStatus' |  |
**filter_left_square_bracket_action_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'action' |  |
**fields_left_square_bracket_app_clip_advanced_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAdvancedExperiences |  |
**fields_left_square_bracket_app_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClips |  |
**fields_left_square_bracket_app_clip_advanced_experience_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAdvancedExperienceImages |  |
**fields_left_square_bracket_app_clip_advanced_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAdvancedExperienceLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |

### Return type

[**models::AppClipAdvancedExperiencesResponse**](AppClipAdvancedExperiencesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clips_app_clip_default_experiences_get_to_many_related

> models::AppClipDefaultExperiencesResponse app_clips_app_clip_default_experiences_get_to_many_related(id, exists_left_square_bracket_release_with_app_store_version_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, fields_left_square_bracket_app_clips_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket, fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket, limit, include, limit_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**exists_left_square_bracket_release_with_app_store_version_right_square_bracket** | Option<**bool**> | filter by existence or non-existence of related 'releaseWithAppStoreVersion' |  |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**fields_left_square_bracket_app_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClips |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**fields_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperienceLocalizations |  |
**fields_left_square_bracket_app_clip_app_store_review_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipAppStoreReviewDetails |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_clip_default_experience_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appClipDefaultExperienceLocalizations returned (when they are included) |  |

### Return type

[**models::AppClipDefaultExperiencesResponse**](AppClipDefaultExperiencesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_clips_get_instance

> models::AppClipResponse app_clips_get_instance(id, fields_left_square_bracket_app_clips_right_square_bracket, fields_left_square_bracket_app_clip_default_experiences_right_square_bracket, include, limit_left_square_bracket_app_clip_default_experiences_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_clips_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClips |  |
**fields_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appClipDefaultExperiences |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_clip_default_experiences_right_square_bracket** | Option<**i32**> | maximum number of related appClipDefaultExperiences returned (when they are included) |  |

### Return type

[**models::AppClipResponse**](AppClipResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


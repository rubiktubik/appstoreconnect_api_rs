# \AppStoreVersionExperimentTreatmentLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_version_experiment_treatment_localizations_app_preview_sets_get_to_many_related**](AppStoreVersionExperimentTreatmentLocalizationsApi.md#app_store_version_experiment_treatment_localizations_app_preview_sets_get_to_many_related) | **GET** /v1/appStoreVersionExperimentTreatmentLocalizations/{id}/appPreviewSets | 
[**app_store_version_experiment_treatment_localizations_app_screenshot_sets_get_to_many_related**](AppStoreVersionExperimentTreatmentLocalizationsApi.md#app_store_version_experiment_treatment_localizations_app_screenshot_sets_get_to_many_related) | **GET** /v1/appStoreVersionExperimentTreatmentLocalizations/{id}/appScreenshotSets | 
[**app_store_version_experiment_treatment_localizations_create_instance**](AppStoreVersionExperimentTreatmentLocalizationsApi.md#app_store_version_experiment_treatment_localizations_create_instance) | **POST** /v1/appStoreVersionExperimentTreatmentLocalizations | 
[**app_store_version_experiment_treatment_localizations_delete_instance**](AppStoreVersionExperimentTreatmentLocalizationsApi.md#app_store_version_experiment_treatment_localizations_delete_instance) | **DELETE** /v1/appStoreVersionExperimentTreatmentLocalizations/{id} | 
[**app_store_version_experiment_treatment_localizations_get_instance**](AppStoreVersionExperimentTreatmentLocalizationsApi.md#app_store_version_experiment_treatment_localizations_get_instance) | **GET** /v1/appStoreVersionExperimentTreatmentLocalizations/{id} | 



## app_store_version_experiment_treatment_localizations_app_preview_sets_get_to_many_related

> models::AppPreviewSetsResponse app_store_version_experiment_treatment_localizations_app_preview_sets_get_to_many_related(id, filter_left_square_bracket_preview_type_right_square_bracket, filter_left_square_bracket_app_store_version_localization_right_square_bracket, filter_left_square_bracket_app_custom_product_page_localization_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, fields_left_square_bracket_app_previews_right_square_bracket, limit, include, limit_left_square_bracket_app_previews_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_preview_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'previewType' |  |
**filter_left_square_bracket_app_store_version_localization_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'appStoreVersionLocalization' |  |
**filter_left_square_bracket_app_custom_product_page_localization_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'appCustomProductPageLocalization' |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**fields_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionLocalizations |  |
**fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageLocalizations |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**fields_left_square_bracket_app_previews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviews |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_previews_right_square_bracket** | Option<**i32**> | maximum number of related appPreviews returned (when they are included) |  |

### Return type

[**models::AppPreviewSetsResponse**](AppPreviewSetsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatment_localizations_app_screenshot_sets_get_to_many_related

> models::AppScreenshotSetsResponse app_store_version_experiment_treatment_localizations_app_screenshot_sets_get_to_many_related(id, filter_left_square_bracket_screenshot_display_type_right_square_bracket, filter_left_square_bracket_app_store_version_localization_right_square_bracket, filter_left_square_bracket_app_custom_product_page_localization_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_store_version_localizations_right_square_bracket, fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, fields_left_square_bracket_app_screenshots_right_square_bracket, limit, include, limit_left_square_bracket_app_screenshots_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_screenshot_display_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'screenshotDisplayType' |  |
**filter_left_square_bracket_app_store_version_localization_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'appStoreVersionLocalization' |  |
**filter_left_square_bracket_app_custom_product_page_localization_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'appCustomProductPageLocalization' |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_store_version_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionLocalizations |  |
**fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageLocalizations |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**fields_left_square_bracket_app_screenshots_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshots |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_screenshots_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshots returned (when they are included) |  |

### Return type

[**models::AppScreenshotSetsResponse**](AppScreenshotSetsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatment_localizations_create_instance

> models::AppStoreVersionExperimentTreatmentLocalizationResponse app_store_version_experiment_treatment_localizations_create_instance(app_store_version_experiment_treatment_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_experiment_treatment_localization_create_request** | [**AppStoreVersionExperimentTreatmentLocalizationCreateRequest**](AppStoreVersionExperimentTreatmentLocalizationCreateRequest.md) | AppStoreVersionExperimentTreatmentLocalization representation | [required] |

### Return type

[**models::AppStoreVersionExperimentTreatmentLocalizationResponse**](AppStoreVersionExperimentTreatmentLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatment_localizations_delete_instance

> app_store_version_experiment_treatment_localizations_delete_instance(id)


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


## app_store_version_experiment_treatment_localizations_get_instance

> models::AppStoreVersionExperimentTreatmentLocalizationResponse app_store_version_experiment_treatment_localizations_get_instance(id, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, include, limit_left_square_bracket_app_preview_sets_right_square_bracket, limit_left_square_bracket_app_screenshot_sets_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_preview_sets_right_square_bracket** | Option<**i32**> | maximum number of related appPreviewSets returned (when they are included) |  |
**limit_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshotSets returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentTreatmentLocalizationResponse**](AppStoreVersionExperimentTreatmentLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


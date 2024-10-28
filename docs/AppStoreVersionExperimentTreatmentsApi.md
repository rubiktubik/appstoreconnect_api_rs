# \AppStoreVersionExperimentTreatmentsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_version_experiment_treatments_app_store_version_experiment_treatment_localizations_get_to_many_related**](AppStoreVersionExperimentTreatmentsApi.md#app_store_version_experiment_treatments_app_store_version_experiment_treatment_localizations_get_to_many_related) | **GET** /v1/appStoreVersionExperimentTreatments/{id}/appStoreVersionExperimentTreatmentLocalizations | 
[**app_store_version_experiment_treatments_create_instance**](AppStoreVersionExperimentTreatmentsApi.md#app_store_version_experiment_treatments_create_instance) | **POST** /v1/appStoreVersionExperimentTreatments | 
[**app_store_version_experiment_treatments_delete_instance**](AppStoreVersionExperimentTreatmentsApi.md#app_store_version_experiment_treatments_delete_instance) | **DELETE** /v1/appStoreVersionExperimentTreatments/{id} | 
[**app_store_version_experiment_treatments_get_instance**](AppStoreVersionExperimentTreatmentsApi.md#app_store_version_experiment_treatments_get_instance) | **GET** /v1/appStoreVersionExperimentTreatments/{id} | 
[**app_store_version_experiment_treatments_update_instance**](AppStoreVersionExperimentTreatmentsApi.md#app_store_version_experiment_treatments_update_instance) | **PATCH** /v1/appStoreVersionExperimentTreatments/{id} | 



## app_store_version_experiment_treatments_app_store_version_experiment_treatment_localizations_get_to_many_related

> models::AppStoreVersionExperimentTreatmentLocalizationsResponse app_store_version_experiment_treatments_app_store_version_experiment_treatment_localizations_get_to_many_related(id, filter_left_square_bracket_locale_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, limit, include, limit_left_square_bracket_app_screenshot_sets_right_square_bracket, limit_left_square_bracket_app_preview_sets_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshotSets returned (when they are included) |  |
**limit_left_square_bracket_app_preview_sets_right_square_bracket** | Option<**i32**> | maximum number of related appPreviewSets returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentTreatmentLocalizationsResponse**](AppStoreVersionExperimentTreatmentLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatments_create_instance

> models::AppStoreVersionExperimentTreatmentResponse app_store_version_experiment_treatments_create_instance(app_store_version_experiment_treatment_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_experiment_treatment_create_request** | [**AppStoreVersionExperimentTreatmentCreateRequest**](AppStoreVersionExperimentTreatmentCreateRequest.md) | AppStoreVersionExperimentTreatment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentTreatmentResponse**](AppStoreVersionExperimentTreatmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatments_delete_instance

> app_store_version_experiment_treatments_delete_instance(id)


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


## app_store_version_experiment_treatments_get_instance

> models::AppStoreVersionExperimentTreatmentResponse app_store_version_experiment_treatments_get_instance(id, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, include, limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatmentLocalizations returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentTreatmentResponse**](AppStoreVersionExperimentTreatmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiment_treatments_update_instance

> models::AppStoreVersionExperimentTreatmentResponse app_store_version_experiment_treatments_update_instance(id, app_store_version_experiment_treatment_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_experiment_treatment_update_request** | [**AppStoreVersionExperimentTreatmentUpdateRequest**](AppStoreVersionExperimentTreatmentUpdateRequest.md) | AppStoreVersionExperimentTreatment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentTreatmentResponse**](AppStoreVersionExperimentTreatmentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


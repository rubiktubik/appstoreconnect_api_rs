# \AppStoreVersionExperimentsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_version_experiments_app_store_version_experiment_treatments_get_to_many_related**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_app_store_version_experiment_treatments_get_to_many_related) | **GET** /v1/appStoreVersionExperiments/{id}/appStoreVersionExperimentTreatments | 
[**app_store_version_experiments_create_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_create_instance) | **POST** /v1/appStoreVersionExperiments | 
[**app_store_version_experiments_delete_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_delete_instance) | **DELETE** /v1/appStoreVersionExperiments/{id} | 
[**app_store_version_experiments_get_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_get_instance) | **GET** /v1/appStoreVersionExperiments/{id} | 
[**app_store_version_experiments_update_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_update_instance) | **PATCH** /v1/appStoreVersionExperiments/{id} | 
[**app_store_version_experiments_v2_app_store_version_experiment_treatments_get_to_many_related**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_v2_app_store_version_experiment_treatments_get_to_many_related) | **GET** /v2/appStoreVersionExperiments/{id}/appStoreVersionExperimentTreatments | 
[**app_store_version_experiments_v2_create_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_v2_create_instance) | **POST** /v2/appStoreVersionExperiments | 
[**app_store_version_experiments_v2_delete_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_v2_delete_instance) | **DELETE** /v2/appStoreVersionExperiments/{id} | 
[**app_store_version_experiments_v2_get_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_v2_get_instance) | **GET** /v2/appStoreVersionExperiments/{id} | 
[**app_store_version_experiments_v2_update_instance**](AppStoreVersionExperimentsApi.md#app_store_version_experiments_v2_update_instance) | **PATCH** /v2/appStoreVersionExperiments/{id} | 



## app_store_version_experiments_app_store_version_experiment_treatments_get_to_many_related

> models::AppStoreVersionExperimentTreatmentsResponse app_store_version_experiments_app_store_version_experiment_treatments_get_to_many_related(id, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, limit, include, limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatmentLocalizations returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentTreatmentsResponse**](AppStoreVersionExperimentTreatmentsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_create_instance

> models::AppStoreVersionExperimentResponse app_store_version_experiments_create_instance(app_store_version_experiment_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_experiment_create_request** | [**AppStoreVersionExperimentCreateRequest**](AppStoreVersionExperimentCreateRequest.md) | AppStoreVersionExperiment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentResponse**](AppStoreVersionExperimentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_delete_instance

> app_store_version_experiments_delete_instance(id)


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


## app_store_version_experiments_get_instance

> models::AppStoreVersionExperimentResponse app_store_version_experiments_get_instance(id, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, include, limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatments returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentResponse**](AppStoreVersionExperimentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_update_instance

> models::AppStoreVersionExperimentResponse app_store_version_experiments_update_instance(id, app_store_version_experiment_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_experiment_update_request** | [**AppStoreVersionExperimentUpdateRequest**](AppStoreVersionExperimentUpdateRequest.md) | AppStoreVersionExperiment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentResponse**](AppStoreVersionExperimentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_v2_app_store_version_experiment_treatments_get_to_many_related

> models::AppStoreVersionExperimentTreatmentsResponse app_store_version_experiments_v2_app_store_version_experiment_treatments_get_to_many_related(id, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket, limit, include, limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatmentLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatment_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatmentLocalizations returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentTreatmentsResponse**](AppStoreVersionExperimentTreatmentsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_v2_create_instance

> models::AppStoreVersionExperimentV2Response app_store_version_experiments_v2_create_instance(app_store_version_experiment_v2_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_experiment_v2_create_request** | [**AppStoreVersionExperimentV2CreateRequest**](AppStoreVersionExperimentV2CreateRequest.md) | AppStoreVersionExperiment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentV2Response**](AppStoreVersionExperimentV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_v2_delete_instance

> app_store_version_experiments_v2_delete_instance(id)


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


## app_store_version_experiments_v2_get_instance

> models::AppStoreVersionExperimentV2Response app_store_version_experiments_v2_get_instance(id, fields_left_square_bracket_app_store_version_experiments_right_square_bracket, fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, include, limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket, limit_left_square_bracket_control_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_store_version_experiments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperiments |  |
**fields_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersionExperimentTreatments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_store_version_experiment_treatments_right_square_bracket** | Option<**i32**> | maximum number of related appStoreVersionExperimentTreatments returned (when they are included) |  |
**limit_left_square_bracket_control_versions_right_square_bracket** | Option<**i32**> | maximum number of related controlVersions returned (when they are included) |  |

### Return type

[**models::AppStoreVersionExperimentV2Response**](AppStoreVersionExperimentV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_experiments_v2_update_instance

> models::AppStoreVersionExperimentV2Response app_store_version_experiments_v2_update_instance(id, app_store_version_experiment_v2_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_experiment_v2_update_request** | [**AppStoreVersionExperimentV2UpdateRequest**](AppStoreVersionExperimentV2UpdateRequest.md) | AppStoreVersionExperiment representation | [required] |

### Return type

[**models::AppStoreVersionExperimentV2Response**](AppStoreVersionExperimentV2Response.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


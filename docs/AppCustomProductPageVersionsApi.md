# \AppCustomProductPageVersionsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_custom_product_page_versions_app_custom_product_page_localizations_get_to_many_related**](AppCustomProductPageVersionsApi.md#app_custom_product_page_versions_app_custom_product_page_localizations_get_to_many_related) | **GET** /v1/appCustomProductPageVersions/{id}/appCustomProductPageLocalizations | 
[**app_custom_product_page_versions_create_instance**](AppCustomProductPageVersionsApi.md#app_custom_product_page_versions_create_instance) | **POST** /v1/appCustomProductPageVersions | 
[**app_custom_product_page_versions_get_instance**](AppCustomProductPageVersionsApi.md#app_custom_product_page_versions_get_instance) | **GET** /v1/appCustomProductPageVersions/{id} | 
[**app_custom_product_page_versions_update_instance**](AppCustomProductPageVersionsApi.md#app_custom_product_page_versions_update_instance) | **PATCH** /v1/appCustomProductPageVersions/{id} | 



## app_custom_product_page_versions_app_custom_product_page_localizations_get_to_many_related

> models::AppCustomProductPageLocalizationsResponse app_custom_product_page_versions_app_custom_product_page_localizations_get_to_many_related(id, filter_left_square_bracket_locale_right_square_bracket, fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket, fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket, fields_left_square_bracket_app_screenshot_sets_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, limit, include, limit_left_square_bracket_app_screenshot_sets_right_square_bracket, limit_left_square_bracket_app_preview_sets_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageLocalizations |  |
**fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageVersions |  |
**fields_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appScreenshotSets |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_screenshot_sets_right_square_bracket** | Option<**i32**> | maximum number of related appScreenshotSets returned (when they are included) |  |
**limit_left_square_bracket_app_preview_sets_right_square_bracket** | Option<**i32**> | maximum number of related appPreviewSets returned (when they are included) |  |

### Return type

[**models::AppCustomProductPageLocalizationsResponse**](AppCustomProductPageLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_page_versions_create_instance

> models::AppCustomProductPageVersionResponse app_custom_product_page_versions_create_instance(app_custom_product_page_version_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_custom_product_page_version_create_request** | [**AppCustomProductPageVersionCreateRequest**](AppCustomProductPageVersionCreateRequest.md) | AppCustomProductPageVersion representation | [required] |

### Return type

[**models::AppCustomProductPageVersionResponse**](AppCustomProductPageVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_page_versions_get_instance

> models::AppCustomProductPageVersionResponse app_custom_product_page_versions_get_instance(id, fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket, fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket, include, limit_left_square_bracket_app_custom_product_page_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageVersions |  |
**fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appCustomProductPageLocalizations returned (when they are included) |  |

### Return type

[**models::AppCustomProductPageVersionResponse**](AppCustomProductPageVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_page_versions_update_instance

> models::AppCustomProductPageVersionResponse app_custom_product_page_versions_update_instance(id, app_custom_product_page_version_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_custom_product_page_version_update_request** | [**AppCustomProductPageVersionUpdateRequest**](AppCustomProductPageVersionUpdateRequest.md) | AppCustomProductPageVersion representation | [required] |

### Return type

[**models::AppCustomProductPageVersionResponse**](AppCustomProductPageVersionResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


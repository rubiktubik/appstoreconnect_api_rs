# \AppCustomProductPagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_custom_product_pages_app_custom_product_page_versions_get_to_many_related**](AppCustomProductPagesApi.md#app_custom_product_pages_app_custom_product_page_versions_get_to_many_related) | **GET** /v1/appCustomProductPages/{id}/appCustomProductPageVersions | 
[**app_custom_product_pages_create_instance**](AppCustomProductPagesApi.md#app_custom_product_pages_create_instance) | **POST** /v1/appCustomProductPages | 
[**app_custom_product_pages_delete_instance**](AppCustomProductPagesApi.md#app_custom_product_pages_delete_instance) | **DELETE** /v1/appCustomProductPages/{id} | 
[**app_custom_product_pages_get_instance**](AppCustomProductPagesApi.md#app_custom_product_pages_get_instance) | **GET** /v1/appCustomProductPages/{id} | 
[**app_custom_product_pages_update_instance**](AppCustomProductPagesApi.md#app_custom_product_pages_update_instance) | **PATCH** /v1/appCustomProductPages/{id} | 



## app_custom_product_pages_app_custom_product_page_versions_get_to_many_related

> models::AppCustomProductPageVersionsResponse app_custom_product_pages_app_custom_product_page_versions_get_to_many_related(id, filter_left_square_bracket_state_right_square_bracket, fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket, fields_left_square_bracket_app_custom_product_pages_right_square_bracket, fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket, limit, include, limit_left_square_bracket_app_custom_product_page_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_state_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'state' |  |
**fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageVersions |  |
**fields_left_square_bracket_app_custom_product_pages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPages |  |
**fields_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageLocalizations |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_custom_product_page_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appCustomProductPageLocalizations returned (when they are included) |  |

### Return type

[**models::AppCustomProductPageVersionsResponse**](AppCustomProductPageVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_pages_create_instance

> models::AppCustomProductPageResponse app_custom_product_pages_create_instance(app_custom_product_page_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_custom_product_page_create_request** | [**AppCustomProductPageCreateRequest**](AppCustomProductPageCreateRequest.md) | AppCustomProductPage representation | [required] |

### Return type

[**models::AppCustomProductPageResponse**](AppCustomProductPageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_pages_delete_instance

> app_custom_product_pages_delete_instance(id)


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


## app_custom_product_pages_get_instance

> models::AppCustomProductPageResponse app_custom_product_pages_get_instance(id, fields_left_square_bracket_app_custom_product_pages_right_square_bracket, fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket, include, limit_left_square_bracket_app_custom_product_page_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_custom_product_pages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPages |  |
**fields_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCustomProductPageVersions |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_custom_product_page_versions_right_square_bracket** | Option<**i32**> | maximum number of related appCustomProductPageVersions returned (when they are included) |  |

### Return type

[**models::AppCustomProductPageResponse**](AppCustomProductPageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_custom_product_pages_update_instance

> models::AppCustomProductPageResponse app_custom_product_pages_update_instance(id, app_custom_product_page_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_custom_product_page_update_request** | [**AppCustomProductPageUpdateRequest**](AppCustomProductPageUpdateRequest.md) | AppCustomProductPage representation | [required] |

### Return type

[**models::AppCustomProductPageResponse**](AppCustomProductPageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


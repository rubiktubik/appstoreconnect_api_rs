# \AppCategoriesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_categories_get_collection**](AppCategoriesApi.md#app_categories_get_collection) | **GET** /v1/appCategories | 
[**app_categories_get_instance**](AppCategoriesApi.md#app_categories_get_instance) | **GET** /v1/appCategories/{id} | 
[**app_categories_parent_get_to_one_related**](AppCategoriesApi.md#app_categories_parent_get_to_one_related) | **GET** /v1/appCategories/{id}/parent | 
[**app_categories_subcategories_get_to_many_related**](AppCategoriesApi.md#app_categories_subcategories_get_to_many_related) | **GET** /v1/appCategories/{id}/subcategories | 



## app_categories_get_collection

> models::AppCategoriesResponse app_categories_get_collection(filter_left_square_bracket_platforms_right_square_bracket, exists_left_square_bracket_parent_right_square_bracket, fields_left_square_bracket_app_categories_right_square_bracket, limit, include, limit_left_square_bracket_subcategories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_platforms_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platforms' |  |
**exists_left_square_bracket_parent_right_square_bracket** | Option<**bool**> | filter by existence or non-existence of related 'parent' |  |
**fields_left_square_bracket_app_categories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCategories |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_subcategories_right_square_bracket** | Option<**i32**> | maximum number of related subcategories returned (when they are included) |  |

### Return type

[**models::AppCategoriesResponse**](AppCategoriesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_categories_get_instance

> models::AppCategoryResponse app_categories_get_instance(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_categories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCategories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_subcategories_right_square_bracket** | Option<**i32**> | maximum number of related subcategories returned (when they are included) |  |

### Return type

[**models::AppCategoryResponse**](AppCategoryResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_categories_parent_get_to_one_related

> models::AppCategoryWithoutIncludesResponse app_categories_parent_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_categories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCategories |  |

### Return type

[**models::AppCategoryWithoutIncludesResponse**](AppCategoryWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_categories_subcategories_get_to_many_related

> models::AppCategoriesWithoutIncludesResponse app_categories_subcategories_get_to_many_related(id, fields_left_square_bracket_app_categories_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_categories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCategories |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AppCategoriesWithoutIncludesResponse**](AppCategoriesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


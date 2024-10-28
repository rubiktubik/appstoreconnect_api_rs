# \AppInfosApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_infos_age_rating_declaration_get_to_one_related**](AppInfosApi.md#app_infos_age_rating_declaration_get_to_one_related) | **GET** /v1/appInfos/{id}/ageRatingDeclaration | 
[**app_infos_app_info_localizations_get_to_many_related**](AppInfosApi.md#app_infos_app_info_localizations_get_to_many_related) | **GET** /v1/appInfos/{id}/appInfoLocalizations | 
[**app_infos_get_instance**](AppInfosApi.md#app_infos_get_instance) | **GET** /v1/appInfos/{id} | 
[**app_infos_primary_category_get_to_one_related**](AppInfosApi.md#app_infos_primary_category_get_to_one_related) | **GET** /v1/appInfos/{id}/primaryCategory | 
[**app_infos_primary_subcategory_one_get_to_one_related**](AppInfosApi.md#app_infos_primary_subcategory_one_get_to_one_related) | **GET** /v1/appInfos/{id}/primarySubcategoryOne | 
[**app_infos_primary_subcategory_two_get_to_one_related**](AppInfosApi.md#app_infos_primary_subcategory_two_get_to_one_related) | **GET** /v1/appInfos/{id}/primarySubcategoryTwo | 
[**app_infos_secondary_category_get_to_one_related**](AppInfosApi.md#app_infos_secondary_category_get_to_one_related) | **GET** /v1/appInfos/{id}/secondaryCategory | 
[**app_infos_secondary_subcategory_one_get_to_one_related**](AppInfosApi.md#app_infos_secondary_subcategory_one_get_to_one_related) | **GET** /v1/appInfos/{id}/secondarySubcategoryOne | 
[**app_infos_secondary_subcategory_two_get_to_one_related**](AppInfosApi.md#app_infos_secondary_subcategory_two_get_to_one_related) | **GET** /v1/appInfos/{id}/secondarySubcategoryTwo | 
[**app_infos_update_instance**](AppInfosApi.md#app_infos_update_instance) | **PATCH** /v1/appInfos/{id} | 



## app_infos_age_rating_declaration_get_to_one_related

> models::AgeRatingDeclarationResponse app_infos_age_rating_declaration_get_to_one_related(id, fields_left_square_bracket_age_rating_declarations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_age_rating_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ageRatingDeclarations |  |

### Return type

[**models::AgeRatingDeclarationResponse**](AgeRatingDeclarationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_infos_app_info_localizations_get_to_many_related

> models::AppInfoLocalizationsResponse app_infos_app_info_localizations_get_to_many_related(id, filter_left_square_bracket_locale_right_square_bracket, fields_left_square_bracket_app_info_localizations_right_square_bracket, fields_left_square_bracket_app_infos_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_locale_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'locale' |  |
**fields_left_square_bracket_app_info_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfoLocalizations |  |
**fields_left_square_bracket_app_infos_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfos |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppInfoLocalizationsResponse**](AppInfoLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_infos_get_instance

> models::AppInfoResponse app_infos_get_instance(id, fields_left_square_bracket_app_infos_right_square_bracket, fields_left_square_bracket_age_rating_declarations_right_square_bracket, fields_left_square_bracket_app_info_localizations_right_square_bracket, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_app_info_localizations_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_infos_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfos |  |
**fields_left_square_bracket_age_rating_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type ageRatingDeclarations |  |
**fields_left_square_bracket_app_info_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appInfoLocalizations |  |
**fields_left_square_bracket_app_categories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appCategories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_info_localizations_right_square_bracket** | Option<**i32**> | maximum number of related appInfoLocalizations returned (when they are included) |  |

### Return type

[**models::AppInfoResponse**](AppInfoResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_infos_primary_category_get_to_one_related

> models::AppCategoryResponse app_infos_primary_category_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_primary_subcategory_one_get_to_one_related

> models::AppCategoryResponse app_infos_primary_subcategory_one_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_primary_subcategory_two_get_to_one_related

> models::AppCategoryResponse app_infos_primary_subcategory_two_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_secondary_category_get_to_one_related

> models::AppCategoryResponse app_infos_secondary_category_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_secondary_subcategory_one_get_to_one_related

> models::AppCategoryResponse app_infos_secondary_subcategory_one_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_secondary_subcategory_two_get_to_one_related

> models::AppCategoryResponse app_infos_secondary_subcategory_two_get_to_one_related(id, fields_left_square_bracket_app_categories_right_square_bracket, include, limit_left_square_bracket_subcategories_right_square_bracket)


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


## app_infos_update_instance

> models::AppInfoResponse app_infos_update_instance(id, app_info_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_info_update_request** | [**AppInfoUpdateRequest**](AppInfoUpdateRequest.md) | AppInfo representation | [required] |

### Return type

[**models::AppInfoResponse**](AppInfoResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AppPreviewSetsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_preview_sets_app_previews_get_to_many_related**](AppPreviewSetsApi.md#app_preview_sets_app_previews_get_to_many_related) | **GET** /v1/appPreviewSets/{id}/appPreviews | 
[**app_preview_sets_app_previews_get_to_many_relationship**](AppPreviewSetsApi.md#app_preview_sets_app_previews_get_to_many_relationship) | **GET** /v1/appPreviewSets/{id}/relationships/appPreviews | 
[**app_preview_sets_app_previews_replace_to_many_relationship**](AppPreviewSetsApi.md#app_preview_sets_app_previews_replace_to_many_relationship) | **PATCH** /v1/appPreviewSets/{id}/relationships/appPreviews | 
[**app_preview_sets_create_instance**](AppPreviewSetsApi.md#app_preview_sets_create_instance) | **POST** /v1/appPreviewSets | 
[**app_preview_sets_delete_instance**](AppPreviewSetsApi.md#app_preview_sets_delete_instance) | **DELETE** /v1/appPreviewSets/{id} | 
[**app_preview_sets_get_instance**](AppPreviewSetsApi.md#app_preview_sets_get_instance) | **GET** /v1/appPreviewSets/{id} | 



## app_preview_sets_app_previews_get_to_many_related

> models::AppPreviewsResponse app_preview_sets_app_previews_get_to_many_related(id, fields_left_square_bracket_app_previews_right_square_bracket, fields_left_square_bracket_app_preview_sets_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_previews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviews |  |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::AppPreviewsResponse**](AppPreviewsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_preview_sets_app_previews_get_to_many_relationship

> models::AppPreviewSetAppPreviewsLinkagesResponse app_preview_sets_app_previews_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AppPreviewSetAppPreviewsLinkagesResponse**](AppPreviewSetAppPreviewsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_preview_sets_app_previews_replace_to_many_relationship

> app_preview_sets_app_previews_replace_to_many_relationship(id, app_preview_set_app_previews_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_preview_set_app_previews_linkages_request** | [**AppPreviewSetAppPreviewsLinkagesRequest**](AppPreviewSetAppPreviewsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_preview_sets_create_instance

> models::AppPreviewSetResponse app_preview_sets_create_instance(app_preview_set_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_preview_set_create_request** | [**AppPreviewSetCreateRequest**](AppPreviewSetCreateRequest.md) | AppPreviewSet representation | [required] |

### Return type

[**models::AppPreviewSetResponse**](AppPreviewSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_preview_sets_delete_instance

> app_preview_sets_delete_instance(id)


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


## app_preview_sets_get_instance

> models::AppPreviewSetResponse app_preview_sets_get_instance(id, fields_left_square_bracket_app_preview_sets_right_square_bracket, fields_left_square_bracket_app_previews_right_square_bracket, include, limit_left_square_bracket_app_previews_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_preview_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviewSets |  |
**fields_left_square_bracket_app_previews_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appPreviews |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_app_previews_right_square_bracket** | Option<**i32**> | maximum number of related appPreviews returned (when they are included) |  |

### Return type

[**models::AppPreviewSetResponse**](AppPreviewSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


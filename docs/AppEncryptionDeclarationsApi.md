# \AppEncryptionDeclarationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_encryption_declarations_app_encryption_declaration_document_get_to_one_related**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_app_encryption_declaration_document_get_to_one_related) | **GET** /v1/appEncryptionDeclarations/{id}/appEncryptionDeclarationDocument | 
[**app_encryption_declarations_app_get_to_one_related**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_app_get_to_one_related) | **GET** /v1/appEncryptionDeclarations/{id}/app | 
[**app_encryption_declarations_builds_create_to_many_relationship**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_builds_create_to_many_relationship) | **POST** /v1/appEncryptionDeclarations/{id}/relationships/builds | 
[**app_encryption_declarations_create_instance**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_create_instance) | **POST** /v1/appEncryptionDeclarations | 
[**app_encryption_declarations_get_collection**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_get_collection) | **GET** /v1/appEncryptionDeclarations | 
[**app_encryption_declarations_get_instance**](AppEncryptionDeclarationsApi.md#app_encryption_declarations_get_instance) | **GET** /v1/appEncryptionDeclarations/{id} | 



## app_encryption_declarations_app_encryption_declaration_document_get_to_one_related

> models::AppEncryptionDeclarationDocumentResponse app_encryption_declarations_app_encryption_declaration_document_get_to_one_related(id, fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarationDocuments |  |

### Return type

[**models::AppEncryptionDeclarationDocumentResponse**](AppEncryptionDeclarationDocumentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declarations_app_get_to_one_related

> models::AppWithoutIncludesResponse app_encryption_declarations_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |

### Return type

[**models::AppWithoutIncludesResponse**](AppWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declarations_builds_create_to_many_relationship

> app_encryption_declarations_builds_create_to_many_relationship(id, app_encryption_declaration_builds_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_encryption_declaration_builds_linkages_request** | [**AppEncryptionDeclarationBuildsLinkagesRequest**](AppEncryptionDeclarationBuildsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declarations_create_instance

> models::AppEncryptionDeclarationResponse app_encryption_declarations_create_instance(app_encryption_declaration_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_encryption_declaration_create_request** | [**AppEncryptionDeclarationCreateRequest**](AppEncryptionDeclarationCreateRequest.md) | AppEncryptionDeclaration representation | [required] |

### Return type

[**models::AppEncryptionDeclarationResponse**](AppEncryptionDeclarationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declarations_get_collection

> models::AppEncryptionDeclarationsResponse app_encryption_declarations_get_collection(filter_left_square_bracket_platform_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket, limit, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_platform_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'platform' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarationDocuments |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::AppEncryptionDeclarationsResponse**](AppEncryptionDeclarationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declarations_get_instance

> models::AppEncryptionDeclarationResponse app_encryption_declarations_get_instance(id, fields_left_square_bracket_app_encryption_declarations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket, include, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_app_encryption_declarations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appEncryptionDeclarationDocuments |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::AppEncryptionDeclarationResponse**](AppEncryptionDeclarationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AppEncryptionDeclarationDocumentsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_encryption_declaration_documents_create_instance**](AppEncryptionDeclarationDocumentsApi.md#app_encryption_declaration_documents_create_instance) | **POST** /v1/appEncryptionDeclarationDocuments | 
[**app_encryption_declaration_documents_get_instance**](AppEncryptionDeclarationDocumentsApi.md#app_encryption_declaration_documents_get_instance) | **GET** /v1/appEncryptionDeclarationDocuments/{id} | 
[**app_encryption_declaration_documents_update_instance**](AppEncryptionDeclarationDocumentsApi.md#app_encryption_declaration_documents_update_instance) | **PATCH** /v1/appEncryptionDeclarationDocuments/{id} | 



## app_encryption_declaration_documents_create_instance

> models::AppEncryptionDeclarationDocumentResponse app_encryption_declaration_documents_create_instance(app_encryption_declaration_document_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_encryption_declaration_document_create_request** | [**AppEncryptionDeclarationDocumentCreateRequest**](AppEncryptionDeclarationDocumentCreateRequest.md) | AppEncryptionDeclarationDocument representation | [required] |

### Return type

[**models::AppEncryptionDeclarationDocumentResponse**](AppEncryptionDeclarationDocumentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_encryption_declaration_documents_get_instance

> models::AppEncryptionDeclarationDocumentResponse app_encryption_declaration_documents_get_instance(id, fields_left_square_bracket_app_encryption_declaration_documents_right_square_bracket)


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


## app_encryption_declaration_documents_update_instance

> models::AppEncryptionDeclarationDocumentResponse app_encryption_declaration_documents_update_instance(id, app_encryption_declaration_document_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_encryption_declaration_document_update_request** | [**AppEncryptionDeclarationDocumentUpdateRequest**](AppEncryptionDeclarationDocumentUpdateRequest.md) | AppEncryptionDeclarationDocument representation | [required] |

### Return type

[**models::AppEncryptionDeclarationDocumentResponse**](AppEncryptionDeclarationDocumentResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


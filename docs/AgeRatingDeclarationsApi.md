# \AgeRatingDeclarationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**age_rating_declarations_update_instance**](AgeRatingDeclarationsApi.md#age_rating_declarations_update_instance) | **PATCH** /v1/ageRatingDeclarations/{id} | 



## age_rating_declarations_update_instance

> models::AgeRatingDeclarationResponse age_rating_declarations_update_instance(id, age_rating_declaration_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**age_rating_declaration_update_request** | [**AgeRatingDeclarationUpdateRequest**](AgeRatingDeclarationUpdateRequest.md) | AgeRatingDeclaration representation | [required] |

### Return type

[**models::AgeRatingDeclarationResponse**](AgeRatingDeclarationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \EndUserLicenseAgreementsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**end_user_license_agreements_create_instance**](EndUserLicenseAgreementsApi.md#end_user_license_agreements_create_instance) | **POST** /v1/endUserLicenseAgreements | 
[**end_user_license_agreements_delete_instance**](EndUserLicenseAgreementsApi.md#end_user_license_agreements_delete_instance) | **DELETE** /v1/endUserLicenseAgreements/{id} | 
[**end_user_license_agreements_get_instance**](EndUserLicenseAgreementsApi.md#end_user_license_agreements_get_instance) | **GET** /v1/endUserLicenseAgreements/{id} | 
[**end_user_license_agreements_territories_get_to_many_related**](EndUserLicenseAgreementsApi.md#end_user_license_agreements_territories_get_to_many_related) | **GET** /v1/endUserLicenseAgreements/{id}/territories | 
[**end_user_license_agreements_update_instance**](EndUserLicenseAgreementsApi.md#end_user_license_agreements_update_instance) | **PATCH** /v1/endUserLicenseAgreements/{id} | 



## end_user_license_agreements_create_instance

> models::EndUserLicenseAgreementResponse end_user_license_agreements_create_instance(end_user_license_agreement_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_user_license_agreement_create_request** | [**EndUserLicenseAgreementCreateRequest**](EndUserLicenseAgreementCreateRequest.md) | EndUserLicenseAgreement representation | [required] |

### Return type

[**models::EndUserLicenseAgreementResponse**](EndUserLicenseAgreementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_user_license_agreements_delete_instance

> end_user_license_agreements_delete_instance(id)


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


## end_user_license_agreements_get_instance

> models::EndUserLicenseAgreementResponse end_user_license_agreements_get_instance(id, fields_left_square_bracket_end_user_license_agreements_right_square_bracket, fields_left_square_bracket_territories_right_square_bracket, include, limit_left_square_bracket_territories_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_end_user_license_agreements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type endUserLicenseAgreements |  |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_territories_right_square_bracket** | Option<**i32**> | maximum number of related territories returned (when they are included) |  |

### Return type

[**models::EndUserLicenseAgreementResponse**](EndUserLicenseAgreementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_user_license_agreements_territories_get_to_many_related

> models::TerritoriesWithoutIncludesResponse end_user_license_agreements_territories_get_to_many_related(id, fields_left_square_bracket_territories_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_territories_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type territories |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::TerritoriesWithoutIncludesResponse**](TerritoriesWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_user_license_agreements_update_instance

> models::EndUserLicenseAgreementResponse end_user_license_agreements_update_instance(id, end_user_license_agreement_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**end_user_license_agreement_update_request** | [**EndUserLicenseAgreementUpdateRequest**](EndUserLicenseAgreementUpdateRequest.md) | EndUserLicenseAgreement representation | [required] |

### Return type

[**models::EndUserLicenseAgreementResponse**](EndUserLicenseAgreementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


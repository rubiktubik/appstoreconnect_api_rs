# \CertificatesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**certificates_create_instance**](CertificatesApi.md#certificates_create_instance) | **POST** /v1/certificates | 
[**certificates_delete_instance**](CertificatesApi.md#certificates_delete_instance) | **DELETE** /v1/certificates/{id} | 
[**certificates_get_collection**](CertificatesApi.md#certificates_get_collection) | **GET** /v1/certificates | 
[**certificates_get_instance**](CertificatesApi.md#certificates_get_instance) | **GET** /v1/certificates/{id} | 



## certificates_create_instance

> models::CertificateResponse certificates_create_instance(certificate_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_create_request** | [**CertificateCreateRequest**](CertificateCreateRequest.md) | Certificate representation | [required] |

### Return type

[**models::CertificateResponse**](CertificateResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## certificates_delete_instance

> certificates_delete_instance(id)


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


## certificates_get_collection

> models::CertificatesResponse certificates_get_collection(filter_left_square_bracket_display_name_right_square_bracket, filter_left_square_bracket_certificate_type_right_square_bracket, filter_left_square_bracket_serial_number_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_certificates_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_display_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'displayName' |  |
**filter_left_square_bracket_certificate_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'certificateType' |  |
**filter_left_square_bracket_serial_number_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'serialNumber' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_certificates_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type certificates |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::CertificatesResponse**](CertificatesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## certificates_get_instance

> models::CertificateResponse certificates_get_instance(id, fields_left_square_bracket_certificates_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_certificates_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type certificates |  |

### Return type

[**models::CertificateResponse**](CertificateResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


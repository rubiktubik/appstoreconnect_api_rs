# \AlternativeDistributionDomainsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alternative_distribution_domains_create_instance**](AlternativeDistributionDomainsApi.md#alternative_distribution_domains_create_instance) | **POST** /v1/alternativeDistributionDomains | 
[**alternative_distribution_domains_delete_instance**](AlternativeDistributionDomainsApi.md#alternative_distribution_domains_delete_instance) | **DELETE** /v1/alternativeDistributionDomains/{id} | 
[**alternative_distribution_domains_get_collection**](AlternativeDistributionDomainsApi.md#alternative_distribution_domains_get_collection) | **GET** /v1/alternativeDistributionDomains | 
[**alternative_distribution_domains_get_instance**](AlternativeDistributionDomainsApi.md#alternative_distribution_domains_get_instance) | **GET** /v1/alternativeDistributionDomains/{id} | 



## alternative_distribution_domains_create_instance

> models::AlternativeDistributionDomainResponse alternative_distribution_domains_create_instance(alternative_distribution_domain_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alternative_distribution_domain_create_request** | [**AlternativeDistributionDomainCreateRequest**](AlternativeDistributionDomainCreateRequest.md) | AlternativeDistributionDomain representation | [required] |

### Return type

[**models::AlternativeDistributionDomainResponse**](AlternativeDistributionDomainResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_domains_delete_instance

> alternative_distribution_domains_delete_instance(id)


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


## alternative_distribution_domains_get_collection

> models::AlternativeDistributionDomainsResponse alternative_distribution_domains_get_collection(fields_left_square_bracket_alternative_distribution_domains_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_alternative_distribution_domains_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionDomains |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AlternativeDistributionDomainsResponse**](AlternativeDistributionDomainsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alternative_distribution_domains_get_instance

> models::AlternativeDistributionDomainResponse alternative_distribution_domains_get_instance(id, fields_left_square_bracket_alternative_distribution_domains_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_alternative_distribution_domains_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type alternativeDistributionDomains |  |

### Return type

[**models::AlternativeDistributionDomainResponse**](AlternativeDistributionDomainResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


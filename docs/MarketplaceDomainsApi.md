# \MarketplaceDomainsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**marketplace_domains_create_instance**](MarketplaceDomainsApi.md#marketplace_domains_create_instance) | **POST** /v1/marketplaceDomains | 
[**marketplace_domains_delete_instance**](MarketplaceDomainsApi.md#marketplace_domains_delete_instance) | **DELETE** /v1/marketplaceDomains/{id} | 
[**marketplace_domains_get_collection**](MarketplaceDomainsApi.md#marketplace_domains_get_collection) | **GET** /v1/marketplaceDomains | 
[**marketplace_domains_get_instance**](MarketplaceDomainsApi.md#marketplace_domains_get_instance) | **GET** /v1/marketplaceDomains/{id} | 



## marketplace_domains_create_instance

> models::MarketplaceDomainResponse marketplace_domains_create_instance(marketplace_domain_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_domain_create_request** | [**MarketplaceDomainCreateRequest**](MarketplaceDomainCreateRequest.md) | MarketplaceDomain representation | [required] |

### Return type

[**models::MarketplaceDomainResponse**](MarketplaceDomainResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## marketplace_domains_delete_instance

> marketplace_domains_delete_instance(id)


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


## marketplace_domains_get_collection

> models::MarketplaceDomainsResponse marketplace_domains_get_collection(fields_left_square_bracket_marketplace_domains_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_marketplace_domains_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type marketplaceDomains |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::MarketplaceDomainsResponse**](MarketplaceDomainsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## marketplace_domains_get_instance

> models::MarketplaceDomainResponse marketplace_domains_get_instance(id, fields_left_square_bracket_marketplace_domains_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_marketplace_domains_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type marketplaceDomains |  |

### Return type

[**models::MarketplaceDomainResponse**](MarketplaceDomainResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


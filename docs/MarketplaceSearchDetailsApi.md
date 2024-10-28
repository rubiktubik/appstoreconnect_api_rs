# \MarketplaceSearchDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**marketplace_search_details_create_instance**](MarketplaceSearchDetailsApi.md#marketplace_search_details_create_instance) | **POST** /v1/marketplaceSearchDetails | 
[**marketplace_search_details_delete_instance**](MarketplaceSearchDetailsApi.md#marketplace_search_details_delete_instance) | **DELETE** /v1/marketplaceSearchDetails/{id} | 
[**marketplace_search_details_update_instance**](MarketplaceSearchDetailsApi.md#marketplace_search_details_update_instance) | **PATCH** /v1/marketplaceSearchDetails/{id} | 



## marketplace_search_details_create_instance

> models::MarketplaceSearchDetailResponse marketplace_search_details_create_instance(marketplace_search_detail_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_search_detail_create_request** | [**MarketplaceSearchDetailCreateRequest**](MarketplaceSearchDetailCreateRequest.md) | MarketplaceSearchDetail representation | [required] |

### Return type

[**models::MarketplaceSearchDetailResponse**](MarketplaceSearchDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## marketplace_search_details_delete_instance

> marketplace_search_details_delete_instance(id)


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


## marketplace_search_details_update_instance

> models::MarketplaceSearchDetailResponse marketplace_search_details_update_instance(id, marketplace_search_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**marketplace_search_detail_update_request** | [**MarketplaceSearchDetailUpdateRequest**](MarketplaceSearchDetailUpdateRequest.md) | MarketplaceSearchDetail representation | [required] |

### Return type

[**models::MarketplaceSearchDetailResponse**](MarketplaceSearchDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


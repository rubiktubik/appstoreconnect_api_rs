# \MarketplaceWebhooksApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**marketplace_webhooks_create_instance**](MarketplaceWebhooksApi.md#marketplace_webhooks_create_instance) | **POST** /v1/marketplaceWebhooks | 
[**marketplace_webhooks_delete_instance**](MarketplaceWebhooksApi.md#marketplace_webhooks_delete_instance) | **DELETE** /v1/marketplaceWebhooks/{id} | 
[**marketplace_webhooks_get_collection**](MarketplaceWebhooksApi.md#marketplace_webhooks_get_collection) | **GET** /v1/marketplaceWebhooks | 
[**marketplace_webhooks_update_instance**](MarketplaceWebhooksApi.md#marketplace_webhooks_update_instance) | **PATCH** /v1/marketplaceWebhooks/{id} | 



## marketplace_webhooks_create_instance

> models::MarketplaceWebhookResponse marketplace_webhooks_create_instance(marketplace_webhook_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_webhook_create_request** | [**MarketplaceWebhookCreateRequest**](MarketplaceWebhookCreateRequest.md) | MarketplaceWebhook representation | [required] |

### Return type

[**models::MarketplaceWebhookResponse**](MarketplaceWebhookResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## marketplace_webhooks_delete_instance

> marketplace_webhooks_delete_instance(id)


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


## marketplace_webhooks_get_collection

> models::MarketplaceWebhooksResponse marketplace_webhooks_get_collection(fields_left_square_bracket_marketplace_webhooks_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_marketplace_webhooks_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type marketplaceWebhooks |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::MarketplaceWebhooksResponse**](MarketplaceWebhooksResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## marketplace_webhooks_update_instance

> models::MarketplaceWebhookResponse marketplace_webhooks_update_instance(id, marketplace_webhook_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**marketplace_webhook_update_request** | [**MarketplaceWebhookUpdateRequest**](MarketplaceWebhookUpdateRequest.md) | MarketplaceWebhook representation | [required] |

### Return type

[**models::MarketplaceWebhookResponse**](MarketplaceWebhookResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


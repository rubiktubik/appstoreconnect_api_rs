# \RoutingAppCoveragesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**routing_app_coverages_create_instance**](RoutingAppCoveragesApi.md#routing_app_coverages_create_instance) | **POST** /v1/routingAppCoverages | 
[**routing_app_coverages_delete_instance**](RoutingAppCoveragesApi.md#routing_app_coverages_delete_instance) | **DELETE** /v1/routingAppCoverages/{id} | 
[**routing_app_coverages_get_instance**](RoutingAppCoveragesApi.md#routing_app_coverages_get_instance) | **GET** /v1/routingAppCoverages/{id} | 
[**routing_app_coverages_update_instance**](RoutingAppCoveragesApi.md#routing_app_coverages_update_instance) | **PATCH** /v1/routingAppCoverages/{id} | 



## routing_app_coverages_create_instance

> models::RoutingAppCoverageResponse routing_app_coverages_create_instance(routing_app_coverage_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routing_app_coverage_create_request** | [**RoutingAppCoverageCreateRequest**](RoutingAppCoverageCreateRequest.md) | RoutingAppCoverage representation | [required] |

### Return type

[**models::RoutingAppCoverageResponse**](RoutingAppCoverageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## routing_app_coverages_delete_instance

> routing_app_coverages_delete_instance(id)


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


## routing_app_coverages_get_instance

> models::RoutingAppCoverageResponse routing_app_coverages_get_instance(id, fields_left_square_bracket_routing_app_coverages_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_routing_app_coverages_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type routingAppCoverages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::RoutingAppCoverageResponse**](RoutingAppCoverageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## routing_app_coverages_update_instance

> models::RoutingAppCoverageResponse routing_app_coverages_update_instance(id, routing_app_coverage_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**routing_app_coverage_update_request** | [**RoutingAppCoverageUpdateRequest**](RoutingAppCoverageUpdateRequest.md) | RoutingAppCoverage representation | [required] |

### Return type

[**models::RoutingAppCoverageResponse**](RoutingAppCoverageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AnalyticsReportRequestsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_report_requests_create_instance**](AnalyticsReportRequestsApi.md#analytics_report_requests_create_instance) | **POST** /v1/analyticsReportRequests | 
[**analytics_report_requests_delete_instance**](AnalyticsReportRequestsApi.md#analytics_report_requests_delete_instance) | **DELETE** /v1/analyticsReportRequests/{id} | 
[**analytics_report_requests_get_instance**](AnalyticsReportRequestsApi.md#analytics_report_requests_get_instance) | **GET** /v1/analyticsReportRequests/{id} | 
[**analytics_report_requests_reports_get_to_many_related**](AnalyticsReportRequestsApi.md#analytics_report_requests_reports_get_to_many_related) | **GET** /v1/analyticsReportRequests/{id}/reports | 



## analytics_report_requests_create_instance

> models::AnalyticsReportRequestResponse analytics_report_requests_create_instance(analytics_report_request_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_report_request_create_request** | [**AnalyticsReportRequestCreateRequest**](AnalyticsReportRequestCreateRequest.md) | AnalyticsReportRequest representation | [required] |

### Return type

[**models::AnalyticsReportRequestResponse**](AnalyticsReportRequestResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_report_requests_delete_instance

> analytics_report_requests_delete_instance(id)


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


## analytics_report_requests_get_instance

> models::AnalyticsReportRequestResponse analytics_report_requests_get_instance(id, fields_left_square_bracket_analytics_report_requests_right_square_bracket, fields_left_square_bracket_analytics_reports_right_square_bracket, include, limit_left_square_bracket_reports_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_analytics_report_requests_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReportRequests |  |
**fields_left_square_bracket_analytics_reports_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReports |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_reports_right_square_bracket** | Option<**i32**> | maximum number of related reports returned (when they are included) |  |

### Return type

[**models::AnalyticsReportRequestResponse**](AnalyticsReportRequestResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_report_requests_reports_get_to_many_related

> models::AnalyticsReportsResponse analytics_report_requests_reports_get_to_many_related(id, filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_category_right_square_bracket, fields_left_square_bracket_analytics_reports_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_category_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'category' |  |
**fields_left_square_bracket_analytics_reports_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReports |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AnalyticsReportsResponse**](AnalyticsReportsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


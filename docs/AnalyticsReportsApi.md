# \AnalyticsReportsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_reports_get_instance**](AnalyticsReportsApi.md#analytics_reports_get_instance) | **GET** /v1/analyticsReports/{id} | 
[**analytics_reports_instances_get_to_many_related**](AnalyticsReportsApi.md#analytics_reports_instances_get_to_many_related) | **GET** /v1/analyticsReports/{id}/instances | 



## analytics_reports_get_instance

> models::AnalyticsReportResponse analytics_reports_get_instance(id, fields_left_square_bracket_analytics_reports_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_analytics_reports_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReports |  |

### Return type

[**models::AnalyticsReportResponse**](AnalyticsReportResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_reports_instances_get_to_many_related

> models::AnalyticsReportInstancesResponse analytics_reports_instances_get_to_many_related(id, filter_left_square_bracket_granularity_right_square_bracket, filter_left_square_bracket_processing_date_right_square_bracket, fields_left_square_bracket_analytics_report_instances_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_granularity_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'granularity' |  |
**filter_left_square_bracket_processing_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'processingDate' |  |
**fields_left_square_bracket_analytics_report_instances_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReportInstances |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AnalyticsReportInstancesResponse**](AnalyticsReportInstancesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AnalyticsReportInstancesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_report_instances_get_instance**](AnalyticsReportInstancesApi.md#analytics_report_instances_get_instance) | **GET** /v1/analyticsReportInstances/{id} | 
[**analytics_report_instances_segments_get_to_many_related**](AnalyticsReportInstancesApi.md#analytics_report_instances_segments_get_to_many_related) | **GET** /v1/analyticsReportInstances/{id}/segments | 



## analytics_report_instances_get_instance

> models::AnalyticsReportInstanceResponse analytics_report_instances_get_instance(id, fields_left_square_bracket_analytics_report_instances_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_analytics_report_instances_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReportInstances |  |

### Return type

[**models::AnalyticsReportInstanceResponse**](AnalyticsReportInstanceResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analytics_report_instances_segments_get_to_many_related

> models::AnalyticsReportSegmentsResponse analytics_report_instances_segments_get_to_many_related(id, fields_left_square_bracket_analytics_report_segments_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_analytics_report_segments_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type analyticsReportSegments |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AnalyticsReportSegmentsResponse**](AnalyticsReportSegmentsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


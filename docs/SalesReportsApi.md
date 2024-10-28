# \SalesReportsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sales_reports_get_collection**](SalesReportsApi.md#sales_reports_get_collection) | **GET** /v1/salesReports | 



## sales_reports_get_collection

> std::path::PathBuf sales_reports_get_collection(filter_left_square_bracket_vendor_number_right_square_bracket, filter_left_square_bracket_report_type_right_square_bracket, filter_left_square_bracket_report_sub_type_right_square_bracket, filter_left_square_bracket_frequency_right_square_bracket, filter_left_square_bracket_report_date_right_square_bracket, filter_left_square_bracket_version_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_vendor_number_right_square_bracket** | [**Vec<String>**](String.md) | filter by attribute 'vendorNumber' | [required] |
**filter_left_square_bracket_report_type_right_square_bracket** | [**Vec<String>**](String.md) | filter by attribute 'reportType' | [required] |
**filter_left_square_bracket_report_sub_type_right_square_bracket** | [**Vec<String>**](String.md) | filter by attribute 'reportSubType' | [required] |
**filter_left_square_bracket_frequency_right_square_bracket** | [**Vec<String>**](String.md) | filter by attribute 'frequency' | [required] |
**filter_left_square_bracket_report_date_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'reportDate' |  |
**filter_left_square_bracket_version_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'version' |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/a-gzip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


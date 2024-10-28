# \BetaTestersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_testers_apps_delete_to_many_relationship**](BetaTestersApi.md#beta_testers_apps_delete_to_many_relationship) | **DELETE** /v1/betaTesters/{id}/relationships/apps | 
[**beta_testers_apps_get_to_many_related**](BetaTestersApi.md#beta_testers_apps_get_to_many_related) | **GET** /v1/betaTesters/{id}/apps | 
[**beta_testers_apps_get_to_many_relationship**](BetaTestersApi.md#beta_testers_apps_get_to_many_relationship) | **GET** /v1/betaTesters/{id}/relationships/apps | 
[**beta_testers_beta_groups_create_to_many_relationship**](BetaTestersApi.md#beta_testers_beta_groups_create_to_many_relationship) | **POST** /v1/betaTesters/{id}/relationships/betaGroups | 
[**beta_testers_beta_groups_delete_to_many_relationship**](BetaTestersApi.md#beta_testers_beta_groups_delete_to_many_relationship) | **DELETE** /v1/betaTesters/{id}/relationships/betaGroups | 
[**beta_testers_beta_groups_get_to_many_related**](BetaTestersApi.md#beta_testers_beta_groups_get_to_many_related) | **GET** /v1/betaTesters/{id}/betaGroups | 
[**beta_testers_beta_groups_get_to_many_relationship**](BetaTestersApi.md#beta_testers_beta_groups_get_to_many_relationship) | **GET** /v1/betaTesters/{id}/relationships/betaGroups | 
[**beta_testers_beta_tester_usages_get_metrics**](BetaTestersApi.md#beta_testers_beta_tester_usages_get_metrics) | **GET** /v1/betaTesters/{id}/metrics/betaTesterUsages | 
[**beta_testers_builds_create_to_many_relationship**](BetaTestersApi.md#beta_testers_builds_create_to_many_relationship) | **POST** /v1/betaTesters/{id}/relationships/builds | 
[**beta_testers_builds_delete_to_many_relationship**](BetaTestersApi.md#beta_testers_builds_delete_to_many_relationship) | **DELETE** /v1/betaTesters/{id}/relationships/builds | 
[**beta_testers_builds_get_to_many_related**](BetaTestersApi.md#beta_testers_builds_get_to_many_related) | **GET** /v1/betaTesters/{id}/builds | 
[**beta_testers_builds_get_to_many_relationship**](BetaTestersApi.md#beta_testers_builds_get_to_many_relationship) | **GET** /v1/betaTesters/{id}/relationships/builds | 
[**beta_testers_create_instance**](BetaTestersApi.md#beta_testers_create_instance) | **POST** /v1/betaTesters | 
[**beta_testers_delete_instance**](BetaTestersApi.md#beta_testers_delete_instance) | **DELETE** /v1/betaTesters/{id} | 
[**beta_testers_get_collection**](BetaTestersApi.md#beta_testers_get_collection) | **GET** /v1/betaTesters | 
[**beta_testers_get_instance**](BetaTestersApi.md#beta_testers_get_instance) | **GET** /v1/betaTesters/{id} | 



## beta_testers_apps_delete_to_many_relationship

> beta_testers_apps_delete_to_many_relationship(id, beta_tester_apps_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_tester_apps_linkages_request** | [**BetaTesterAppsLinkagesRequest**](BetaTesterAppsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_apps_get_to_many_related

> models::AppsWithoutIncludesResponse beta_testers_apps_get_to_many_related(id, fields_left_square_bracket_apps_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AppsWithoutIncludesResponse**](AppsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_apps_get_to_many_relationship

> models::BetaTesterAppsLinkagesResponse beta_testers_apps_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaTesterAppsLinkagesResponse**](BetaTesterAppsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_beta_groups_create_to_many_relationship

> beta_testers_beta_groups_create_to_many_relationship(id, beta_tester_beta_groups_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_tester_beta_groups_linkages_request** | [**BetaTesterBetaGroupsLinkagesRequest**](BetaTesterBetaGroupsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_beta_groups_delete_to_many_relationship

> beta_testers_beta_groups_delete_to_many_relationship(id, beta_tester_beta_groups_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_tester_beta_groups_linkages_request** | [**BetaTesterBetaGroupsLinkagesRequest**](BetaTesterBetaGroupsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_beta_groups_get_to_many_related

> models::BetaGroupsWithoutIncludesResponse beta_testers_beta_groups_get_to_many_related(id, fields_left_square_bracket_beta_groups_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaGroupsWithoutIncludesResponse**](BetaGroupsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_beta_groups_get_to_many_relationship

> models::BetaTesterBetaGroupsLinkagesResponse beta_testers_beta_groups_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaTesterBetaGroupsLinkagesResponse**](BetaTesterBetaGroupsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_beta_tester_usages_get_metrics

> models::BetaTesterUsagesV1MetricResponse beta_testers_beta_tester_usages_get_metrics(id, filter_left_square_bracket_apps_right_square_bracket, period, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_apps_right_square_bracket** | **String** | filter by 'apps' relationship dimension | [required] |
**period** | Option<**String**> | the duration of the reporting period |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::BetaTesterUsagesV1MetricResponse**](BetaTesterUsagesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_builds_create_to_many_relationship

> beta_testers_builds_create_to_many_relationship(id, beta_tester_builds_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_tester_builds_linkages_request** | [**BetaTesterBuildsLinkagesRequest**](BetaTesterBuildsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_builds_delete_to_many_relationship

> beta_testers_builds_delete_to_many_relationship(id, beta_tester_builds_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_tester_builds_linkages_request** | [**BetaTesterBuildsLinkagesRequest**](BetaTesterBuildsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_builds_get_to_many_related

> models::BuildsWithoutIncludesResponse beta_testers_builds_get_to_many_related(id, fields_left_square_bracket_builds_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BuildsWithoutIncludesResponse**](BuildsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_builds_get_to_many_relationship

> models::BetaTesterBuildsLinkagesResponse beta_testers_builds_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaTesterBuildsLinkagesResponse**](BetaTesterBuildsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_create_instance

> models::BetaTesterResponse beta_testers_create_instance(beta_tester_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_tester_create_request** | [**BetaTesterCreateRequest**](BetaTesterCreateRequest.md) | BetaTester representation | [required] |

### Return type

[**models::BetaTesterResponse**](BetaTesterResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_delete_instance

> beta_testers_delete_instance(id)


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


## beta_testers_get_collection

> models::BetaTestersResponse beta_testers_get_collection(filter_left_square_bracket_first_name_right_square_bracket, filter_left_square_bracket_last_name_right_square_bracket, filter_left_square_bracket_email_right_square_bracket, filter_left_square_bracket_invite_type_right_square_bracket, filter_left_square_bracket_apps_right_square_bracket, filter_left_square_bracket_beta_groups_right_square_bracket, filter_left_square_bracket_builds_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_beta_testers_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, limit, include, limit_left_square_bracket_apps_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_first_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'firstName' |  |
**filter_left_square_bracket_last_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'lastName' |  |
**filter_left_square_bracket_email_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'email' |  |
**filter_left_square_bracket_invite_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'inviteType' |  |
**filter_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'apps' |  |
**filter_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'betaGroups' |  |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_apps_right_square_bracket** | Option<**i32**> | maximum number of related apps returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::BetaTestersResponse**](BetaTestersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_testers_get_instance

> models::BetaTesterResponse beta_testers_get_instance(id, fields_left_square_bracket_beta_testers_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, include, limit_left_square_bracket_apps_right_square_bracket, limit_left_square_bracket_beta_groups_right_square_bracket, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_apps_right_square_bracket** | Option<**i32**> | maximum number of related apps returned (when they are included) |  |
**limit_left_square_bracket_beta_groups_right_square_bracket** | Option<**i32**> | maximum number of related betaGroups returned (when they are included) |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::BetaTesterResponse**](BetaTesterResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


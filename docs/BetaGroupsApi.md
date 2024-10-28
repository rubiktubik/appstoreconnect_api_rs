# \BetaGroupsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_groups_app_get_to_one_related**](BetaGroupsApi.md#beta_groups_app_get_to_one_related) | **GET** /v1/betaGroups/{id}/app | 
[**beta_groups_beta_tester_usages_get_metrics**](BetaGroupsApi.md#beta_groups_beta_tester_usages_get_metrics) | **GET** /v1/betaGroups/{id}/metrics/betaTesterUsages | 
[**beta_groups_beta_testers_create_to_many_relationship**](BetaGroupsApi.md#beta_groups_beta_testers_create_to_many_relationship) | **POST** /v1/betaGroups/{id}/relationships/betaTesters | 
[**beta_groups_beta_testers_delete_to_many_relationship**](BetaGroupsApi.md#beta_groups_beta_testers_delete_to_many_relationship) | **DELETE** /v1/betaGroups/{id}/relationships/betaTesters | 
[**beta_groups_beta_testers_get_to_many_related**](BetaGroupsApi.md#beta_groups_beta_testers_get_to_many_related) | **GET** /v1/betaGroups/{id}/betaTesters | 
[**beta_groups_beta_testers_get_to_many_relationship**](BetaGroupsApi.md#beta_groups_beta_testers_get_to_many_relationship) | **GET** /v1/betaGroups/{id}/relationships/betaTesters | 
[**beta_groups_builds_create_to_many_relationship**](BetaGroupsApi.md#beta_groups_builds_create_to_many_relationship) | **POST** /v1/betaGroups/{id}/relationships/builds | 
[**beta_groups_builds_delete_to_many_relationship**](BetaGroupsApi.md#beta_groups_builds_delete_to_many_relationship) | **DELETE** /v1/betaGroups/{id}/relationships/builds | 
[**beta_groups_builds_get_to_many_related**](BetaGroupsApi.md#beta_groups_builds_get_to_many_related) | **GET** /v1/betaGroups/{id}/builds | 
[**beta_groups_builds_get_to_many_relationship**](BetaGroupsApi.md#beta_groups_builds_get_to_many_relationship) | **GET** /v1/betaGroups/{id}/relationships/builds | 
[**beta_groups_create_instance**](BetaGroupsApi.md#beta_groups_create_instance) | **POST** /v1/betaGroups | 
[**beta_groups_delete_instance**](BetaGroupsApi.md#beta_groups_delete_instance) | **DELETE** /v1/betaGroups/{id} | 
[**beta_groups_get_collection**](BetaGroupsApi.md#beta_groups_get_collection) | **GET** /v1/betaGroups | 
[**beta_groups_get_instance**](BetaGroupsApi.md#beta_groups_get_instance) | **GET** /v1/betaGroups/{id} | 
[**beta_groups_update_instance**](BetaGroupsApi.md#beta_groups_update_instance) | **PATCH** /v1/betaGroups/{id} | 



## beta_groups_app_get_to_one_related

> models::AppWithoutIncludesResponse beta_groups_app_get_to_one_related(id, fields_left_square_bracket_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |

### Return type

[**models::AppWithoutIncludesResponse**](AppWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_beta_tester_usages_get_metrics

> models::AppsBetaTesterUsagesV1MetricResponse beta_groups_beta_tester_usages_get_metrics(id, period, group_by, filter_left_square_bracket_beta_testers_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**period** | Option<**String**> | the duration of the reporting period |  |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_beta_testers_right_square_bracket** | Option<**String**> | filter by 'betaTesters' relationship dimension |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::AppsBetaTesterUsagesV1MetricResponse**](AppsBetaTesterUsagesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_beta_testers_create_to_many_relationship

> beta_groups_beta_testers_create_to_many_relationship(id, beta_group_beta_testers_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_group_beta_testers_linkages_request** | [**BetaGroupBetaTestersLinkagesRequest**](BetaGroupBetaTestersLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_beta_testers_delete_to_many_relationship

> beta_groups_beta_testers_delete_to_many_relationship(id, beta_group_beta_testers_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_group_beta_testers_linkages_request** | [**BetaGroupBetaTestersLinkagesRequest**](BetaGroupBetaTestersLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_beta_testers_get_to_many_related

> models::BetaTestersWithoutIncludesResponse beta_groups_beta_testers_get_to_many_related(id, fields_left_square_bracket_beta_testers_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaTestersWithoutIncludesResponse**](BetaTestersWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_beta_testers_get_to_many_relationship

> models::BetaGroupBetaTestersLinkagesResponse beta_groups_beta_testers_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaGroupBetaTestersLinkagesResponse**](BetaGroupBetaTestersLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_builds_create_to_many_relationship

> beta_groups_builds_create_to_many_relationship(id, beta_group_builds_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_group_builds_linkages_request** | [**BetaGroupBuildsLinkagesRequest**](BetaGroupBuildsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_builds_delete_to_many_relationship

> beta_groups_builds_delete_to_many_relationship(id, beta_group_builds_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_group_builds_linkages_request** | [**BetaGroupBuildsLinkagesRequest**](BetaGroupBuildsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_builds_get_to_many_related

> models::BuildsWithoutIncludesResponse beta_groups_builds_get_to_many_related(id, fields_left_square_bracket_builds_right_square_bracket, limit)


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


## beta_groups_builds_get_to_many_relationship

> models::BetaGroupBuildsLinkagesResponse beta_groups_builds_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::BetaGroupBuildsLinkagesResponse**](BetaGroupBuildsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_create_instance

> models::BetaGroupResponse beta_groups_create_instance(beta_group_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_group_create_request** | [**BetaGroupCreateRequest**](BetaGroupCreateRequest.md) | BetaGroup representation | [required] |

### Return type

[**models::BetaGroupResponse**](BetaGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_delete_instance

> beta_groups_delete_instance(id)


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


## beta_groups_get_collection

> models::BetaGroupsResponse beta_groups_get_collection(filter_left_square_bracket_name_right_square_bracket, filter_left_square_bracket_is_internal_group_right_square_bracket, filter_left_square_bracket_public_link_enabled_right_square_bracket, filter_left_square_bracket_public_link_limit_enabled_right_square_bracket, filter_left_square_bracket_public_link_right_square_bracket, filter_left_square_bracket_app_right_square_bracket, filter_left_square_bracket_builds_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, sort, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_beta_testers_right_square_bracket, limit, include, limit_left_square_bracket_beta_testers_right_square_bracket, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'name' |  |
**filter_left_square_bracket_is_internal_group_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'isInternalGroup' |  |
**filter_left_square_bracket_public_link_enabled_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'publicLinkEnabled' |  |
**filter_left_square_bracket_public_link_limit_enabled_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'publicLinkLimitEnabled' |  |
**filter_left_square_bracket_public_link_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'publicLink' |  |
**filter_left_square_bracket_app_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'app' |  |
**filter_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'builds' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_testers_right_square_bracket** | Option<**i32**> | maximum number of related betaTesters returned (when they are included) |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::BetaGroupsResponse**](BetaGroupsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_get_instance

> models::BetaGroupResponse beta_groups_get_instance(id, fields_left_square_bracket_beta_groups_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_builds_right_square_bracket, fields_left_square_bracket_beta_testers_right_square_bracket, include, limit_left_square_bracket_beta_testers_right_square_bracket, limit_left_square_bracket_builds_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_beta_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaGroups |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_builds_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type builds |  |
**fields_left_square_bracket_beta_testers_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type betaTesters |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_beta_testers_right_square_bracket** | Option<**i32**> | maximum number of related betaTesters returned (when they are included) |  |
**limit_left_square_bracket_builds_right_square_bracket** | Option<**i32**> | maximum number of related builds returned (when they are included) |  |

### Return type

[**models::BetaGroupResponse**](BetaGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_groups_update_instance

> models::BetaGroupResponse beta_groups_update_instance(id, beta_group_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**beta_group_update_request** | [**BetaGroupUpdateRequest**](BetaGroupUpdateRequest.md) | BetaGroup representation | [required] |

### Return type

[**models::BetaGroupResponse**](BetaGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \MetricsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_beta_tester_usages_get_metrics_0**](MetricsApi.md#apps_beta_tester_usages_get_metrics_0) | **GET** /v1/apps/{id}/metrics/betaTesterUsages | 
[**beta_groups_beta_tester_usages_get_metrics_0**](MetricsApi.md#beta_groups_beta_tester_usages_get_metrics_0) | **GET** /v1/betaGroups/{id}/metrics/betaTesterUsages | 
[**beta_testers_beta_tester_usages_get_metrics_0**](MetricsApi.md#beta_testers_beta_tester_usages_get_metrics_0) | **GET** /v1/betaTesters/{id}/metrics/betaTesterUsages | 
[**builds_beta_build_usages_get_metrics_0**](MetricsApi.md#builds_beta_build_usages_get_metrics_0) | **GET** /v1/builds/{id}/metrics/betaBuildUsages | 
[**game_center_details_classic_matchmaking_requests_get_metrics_0**](MetricsApi.md#game_center_details_classic_matchmaking_requests_get_metrics_0) | **GET** /v1/gameCenterDetails/{id}/metrics/classicMatchmakingRequests | 
[**game_center_details_rule_based_matchmaking_requests_get_metrics_0**](MetricsApi.md#game_center_details_rule_based_matchmaking_requests_get_metrics_0) | **GET** /v1/gameCenterDetails/{id}/metrics/ruleBasedMatchmakingRequests | 
[**game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics_0**](MetricsApi.md#game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics_0) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/experimentMatchmakingQueueSizes | 
[**game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics_0**](MetricsApi.md#game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics_0) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/experimentMatchmakingRequests | 
[**game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics_0**](MetricsApi.md#game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics_0) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingQueueSizes | 
[**game_center_matchmaking_queues_matchmaking_requests_get_metrics_0**](MetricsApi.md#game_center_matchmaking_queues_matchmaking_requests_get_metrics_0) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingRequests | 
[**game_center_matchmaking_queues_matchmaking_sessions_get_metrics_0**](MetricsApi.md#game_center_matchmaking_queues_matchmaking_sessions_get_metrics_0) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingSessions | 
[**game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics_0**](MetricsApi.md#game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics_0) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingBooleanRuleResults | 
[**game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics_0**](MetricsApi.md#game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics_0) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingNumberRuleResults | 
[**game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics_0**](MetricsApi.md#game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics_0) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingRuleErrors | 



## apps_beta_tester_usages_get_metrics_0

> models::AppsBetaTesterUsagesV1MetricResponse apps_beta_tester_usages_get_metrics_0(id, period, group_by, filter_left_square_bracket_beta_testers_right_square_bracket, limit)


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


## beta_groups_beta_tester_usages_get_metrics_0

> models::AppsBetaTesterUsagesV1MetricResponse beta_groups_beta_tester_usages_get_metrics_0(id, period, group_by, filter_left_square_bracket_beta_testers_right_square_bracket, limit)


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


## beta_testers_beta_tester_usages_get_metrics_0

> models::BetaTesterUsagesV1MetricResponse beta_testers_beta_tester_usages_get_metrics_0(id, filter_left_square_bracket_apps_right_square_bracket, period, limit)


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


## builds_beta_build_usages_get_metrics_0

> models::BetaBuildUsagesV1MetricResponse builds_beta_build_usages_get_metrics_0(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::BetaBuildUsagesV1MetricResponse**](BetaBuildUsagesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_classic_matchmaking_requests_get_metrics_0

> models::GameCenterMatchmakingAppRequestsV1MetricResponse game_center_details_classic_matchmaking_requests_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingAppRequestsV1MetricResponse**](GameCenterMatchmakingAppRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_rule_based_matchmaking_requests_get_metrics_0

> models::GameCenterMatchmakingAppRequestsV1MetricResponse game_center_details_rule_based_matchmaking_requests_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingAppRequestsV1MetricResponse**](GameCenterMatchmakingAppRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics_0

> models::GameCenterMatchmakingQueueSizesV1MetricResponse game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics_0(id, granularity, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingQueueSizesV1MetricResponse**](GameCenterMatchmakingQueueSizesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics_0

> models::GameCenterMatchmakingQueueRequestsV1MetricResponse game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**filter_left_square_bracket_game_center_detail_right_square_bracket** | Option<**String**> | filter by 'gameCenterDetail' relationship dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingQueueRequestsV1MetricResponse**](GameCenterMatchmakingQueueRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics_0

> models::GameCenterMatchmakingQueueSizesV1MetricResponse game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics_0(id, granularity, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingQueueSizesV1MetricResponse**](GameCenterMatchmakingQueueSizesV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_matchmaking_requests_get_metrics_0

> models::GameCenterMatchmakingQueueRequestsV1MetricResponse game_center_matchmaking_queues_matchmaking_requests_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**filter_left_square_bracket_game_center_detail_right_square_bracket** | Option<**String**> | filter by 'gameCenterDetail' relationship dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingQueueRequestsV1MetricResponse**](GameCenterMatchmakingQueueRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_matchmaking_sessions_get_metrics_0

> models::GameCenterMatchmakingSessionsV1MetricResponse game_center_matchmaking_queues_matchmaking_sessions_get_metrics_0(id, granularity, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingSessionsV1MetricResponse**](GameCenterMatchmakingSessionsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics_0

> models::GameCenterMatchmakingBooleanRuleResultsV1MetricResponse game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket** | Option<**String**> | filter by 'gameCenterMatchmakingQueue' relationship dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingBooleanRuleResultsV1MetricResponse**](GameCenterMatchmakingBooleanRuleResultsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics_0

> models::GameCenterMatchmakingNumberRuleResultsV1MetricResponse game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket** | Option<**String**> | filter by 'gameCenterMatchmakingQueue' relationship dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingNumberRuleResultsV1MetricResponse**](GameCenterMatchmakingNumberRuleResultsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics_0

> models::GameCenterMatchmakingRuleErrorsV1MetricResponse game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics_0(id, granularity, group_by, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket** | Option<**String**> | filter by 'gameCenterMatchmakingQueue' relationship dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingRuleErrorsV1MetricResponse**](GameCenterMatchmakingRuleErrorsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


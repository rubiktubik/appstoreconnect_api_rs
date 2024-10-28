# \GameCenterMatchmakingQueuesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_matchmaking_queues_create_instance**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_create_instance) | **POST** /v1/gameCenterMatchmakingQueues | 
[**game_center_matchmaking_queues_delete_instance**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_delete_instance) | **DELETE** /v1/gameCenterMatchmakingQueues/{id} | 
[**game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/experimentMatchmakingQueueSizes | 
[**game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/experimentMatchmakingRequests | 
[**game_center_matchmaking_queues_get_collection**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_get_collection) | **GET** /v1/gameCenterMatchmakingQueues | 
[**game_center_matchmaking_queues_get_instance**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_get_instance) | **GET** /v1/gameCenterMatchmakingQueues/{id} | 
[**game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingQueueSizes | 
[**game_center_matchmaking_queues_matchmaking_requests_get_metrics**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_matchmaking_requests_get_metrics) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingRequests | 
[**game_center_matchmaking_queues_matchmaking_sessions_get_metrics**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_matchmaking_sessions_get_metrics) | **GET** /v1/gameCenterMatchmakingQueues/{id}/metrics/matchmakingSessions | 
[**game_center_matchmaking_queues_update_instance**](GameCenterMatchmakingQueuesApi.md#game_center_matchmaking_queues_update_instance) | **PATCH** /v1/gameCenterMatchmakingQueues/{id} | 



## game_center_matchmaking_queues_create_instance

> models::GameCenterMatchmakingQueueResponse game_center_matchmaking_queues_create_instance(game_center_matchmaking_queue_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_matchmaking_queue_create_request** | [**GameCenterMatchmakingQueueCreateRequest**](GameCenterMatchmakingQueueCreateRequest.md) | GameCenterMatchmakingQueue representation | [required] |

### Return type

[**models::GameCenterMatchmakingQueueResponse**](GameCenterMatchmakingQueueResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_delete_instance

> game_center_matchmaking_queues_delete_instance(id)


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


## game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics

> models::GameCenterMatchmakingQueueSizesV1MetricResponse game_center_matchmaking_queues_experiment_matchmaking_queue_sizes_get_metrics(id, granularity, sort, limit)


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


## game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics

> models::GameCenterMatchmakingQueueRequestsV1MetricResponse game_center_matchmaking_queues_experiment_matchmaking_requests_get_metrics(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, sort, limit)


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


## game_center_matchmaking_queues_get_collection

> models::GameCenterMatchmakingQueuesResponse game_center_matchmaking_queues_get_collection(fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingQueues |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterMatchmakingQueuesResponse**](GameCenterMatchmakingQueuesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_get_instance

> models::GameCenterMatchmakingQueueResponse game_center_matchmaking_queues_get_instance(id, fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingQueues |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterMatchmakingQueueResponse**](GameCenterMatchmakingQueueResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics

> models::GameCenterMatchmakingQueueSizesV1MetricResponse game_center_matchmaking_queues_matchmaking_queue_sizes_get_metrics(id, granularity, sort, limit)


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


## game_center_matchmaking_queues_matchmaking_requests_get_metrics

> models::GameCenterMatchmakingQueueRequestsV1MetricResponse game_center_matchmaking_queues_matchmaking_requests_get_metrics(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, sort, limit)


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


## game_center_matchmaking_queues_matchmaking_sessions_get_metrics

> models::GameCenterMatchmakingSessionsV1MetricResponse game_center_matchmaking_queues_matchmaking_sessions_get_metrics(id, granularity, sort, limit)


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


## game_center_matchmaking_queues_update_instance

> models::GameCenterMatchmakingQueueResponse game_center_matchmaking_queues_update_instance(id, game_center_matchmaking_queue_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_matchmaking_queue_update_request** | [**GameCenterMatchmakingQueueUpdateRequest**](GameCenterMatchmakingQueueUpdateRequest.md) | GameCenterMatchmakingQueue representation | [required] |

### Return type

[**models::GameCenterMatchmakingQueueResponse**](GameCenterMatchmakingQueueResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


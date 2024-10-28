# \GameCenterMatchmakingRulesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_matchmaking_rules_create_instance**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_create_instance) | **POST** /v1/gameCenterMatchmakingRules | 
[**game_center_matchmaking_rules_delete_instance**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_delete_instance) | **DELETE** /v1/gameCenterMatchmakingRules/{id} | 
[**game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingBooleanRuleResults | 
[**game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingNumberRuleResults | 
[**game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics) | **GET** /v1/gameCenterMatchmakingRules/{id}/metrics/matchmakingRuleErrors | 
[**game_center_matchmaking_rules_update_instance**](GameCenterMatchmakingRulesApi.md#game_center_matchmaking_rules_update_instance) | **PATCH** /v1/gameCenterMatchmakingRules/{id} | 



## game_center_matchmaking_rules_create_instance

> models::GameCenterMatchmakingRuleResponse game_center_matchmaking_rules_create_instance(game_center_matchmaking_rule_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_matchmaking_rule_create_request** | [**GameCenterMatchmakingRuleCreateRequest**](GameCenterMatchmakingRuleCreateRequest.md) | GameCenterMatchmakingRule representation | [required] |

### Return type

[**models::GameCenterMatchmakingRuleResponse**](GameCenterMatchmakingRuleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rules_delete_instance

> game_center_matchmaking_rules_delete_instance(id)


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


## game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics

> models::GameCenterMatchmakingBooleanRuleResultsV1MetricResponse game_center_matchmaking_rules_matchmaking_boolean_rule_results_get_metrics(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


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


## game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics

> models::GameCenterMatchmakingNumberRuleResultsV1MetricResponse game_center_matchmaking_rules_matchmaking_number_rule_results_get_metrics(id, granularity, group_by, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


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


## game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics

> models::GameCenterMatchmakingRuleErrorsV1MetricResponse game_center_matchmaking_rules_matchmaking_rule_errors_get_metrics(id, granularity, group_by, filter_left_square_bracket_game_center_matchmaking_queue_right_square_bracket, sort, limit)


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


## game_center_matchmaking_rules_update_instance

> models::GameCenterMatchmakingRuleResponse game_center_matchmaking_rules_update_instance(id, game_center_matchmaking_rule_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_matchmaking_rule_update_request** | [**GameCenterMatchmakingRuleUpdateRequest**](GameCenterMatchmakingRuleUpdateRequest.md) | GameCenterMatchmakingRule representation | [required] |

### Return type

[**models::GameCenterMatchmakingRuleResponse**](GameCenterMatchmakingRuleResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


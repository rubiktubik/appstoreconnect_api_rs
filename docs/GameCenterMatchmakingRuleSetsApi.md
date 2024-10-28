# \GameCenterMatchmakingRuleSetsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_matchmaking_rule_sets_create_instance**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_create_instance) | **POST** /v1/gameCenterMatchmakingRuleSets | 
[**game_center_matchmaking_rule_sets_delete_instance**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_delete_instance) | **DELETE** /v1/gameCenterMatchmakingRuleSets/{id} | 
[**game_center_matchmaking_rule_sets_get_collection**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_get_collection) | **GET** /v1/gameCenterMatchmakingRuleSets | 
[**game_center_matchmaking_rule_sets_get_instance**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_get_instance) | **GET** /v1/gameCenterMatchmakingRuleSets/{id} | 
[**game_center_matchmaking_rule_sets_matchmaking_queues_get_to_many_related**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_matchmaking_queues_get_to_many_related) | **GET** /v1/gameCenterMatchmakingRuleSets/{id}/matchmakingQueues | 
[**game_center_matchmaking_rule_sets_rules_get_to_many_related**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_rules_get_to_many_related) | **GET** /v1/gameCenterMatchmakingRuleSets/{id}/rules | 
[**game_center_matchmaking_rule_sets_teams_get_to_many_related**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_teams_get_to_many_related) | **GET** /v1/gameCenterMatchmakingRuleSets/{id}/teams | 
[**game_center_matchmaking_rule_sets_update_instance**](GameCenterMatchmakingRuleSetsApi.md#game_center_matchmaking_rule_sets_update_instance) | **PATCH** /v1/gameCenterMatchmakingRuleSets/{id} | 



## game_center_matchmaking_rule_sets_create_instance

> models::GameCenterMatchmakingRuleSetResponse game_center_matchmaking_rule_sets_create_instance(game_center_matchmaking_rule_set_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_matchmaking_rule_set_create_request** | [**GameCenterMatchmakingRuleSetCreateRequest**](GameCenterMatchmakingRuleSetCreateRequest.md) | GameCenterMatchmakingRuleSet representation | [required] |

### Return type

[**models::GameCenterMatchmakingRuleSetResponse**](GameCenterMatchmakingRuleSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rule_sets_delete_instance

> game_center_matchmaking_rule_sets_delete_instance(id)


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


## game_center_matchmaking_rule_sets_get_collection

> models::GameCenterMatchmakingRuleSetsResponse game_center_matchmaking_rule_sets_get_collection(fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket, limit, include, limit_left_square_bracket_matchmaking_queues_right_square_bracket, limit_left_square_bracket_rules_right_square_bracket, limit_left_square_bracket_teams_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRuleSets |  |
**fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingTeams |  |
**fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRules |  |
**fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingQueues |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_matchmaking_queues_right_square_bracket** | Option<**i32**> | maximum number of related matchmakingQueues returned (when they are included) |  |
**limit_left_square_bracket_rules_right_square_bracket** | Option<**i32**> | maximum number of related rules returned (when they are included) |  |
**limit_left_square_bracket_teams_right_square_bracket** | Option<**i32**> | maximum number of related teams returned (when they are included) |  |

### Return type

[**models::GameCenterMatchmakingRuleSetsResponse**](GameCenterMatchmakingRuleSetsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rule_sets_get_instance

> models::GameCenterMatchmakingRuleSetResponse game_center_matchmaking_rule_sets_get_instance(id, fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket, include, limit_left_square_bracket_matchmaking_queues_right_square_bracket, limit_left_square_bracket_rules_right_square_bracket, limit_left_square_bracket_teams_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRuleSets |  |
**fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingTeams |  |
**fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRules |  |
**fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingQueues |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_matchmaking_queues_right_square_bracket** | Option<**i32**> | maximum number of related matchmakingQueues returned (when they are included) |  |
**limit_left_square_bracket_rules_right_square_bracket** | Option<**i32**> | maximum number of related rules returned (when they are included) |  |
**limit_left_square_bracket_teams_right_square_bracket** | Option<**i32**> | maximum number of related teams returned (when they are included) |  |

### Return type

[**models::GameCenterMatchmakingRuleSetResponse**](GameCenterMatchmakingRuleSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rule_sets_matchmaking_queues_get_to_many_related

> models::GameCenterMatchmakingQueuesResponse game_center_matchmaking_rule_sets_matchmaking_queues_get_to_many_related(id, fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket, fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_matchmaking_queues_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingQueues |  |
**fields_left_square_bracket_game_center_matchmaking_rule_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRuleSets |  |
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


## game_center_matchmaking_rule_sets_rules_get_to_many_related

> models::GameCenterMatchmakingRulesResponse game_center_matchmaking_rule_sets_rules_get_to_many_related(id, fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_matchmaking_rules_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingRules |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterMatchmakingRulesResponse**](GameCenterMatchmakingRulesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rule_sets_teams_get_to_many_related

> models::GameCenterMatchmakingTeamsResponse game_center_matchmaking_rule_sets_teams_get_to_many_related(id, fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_matchmaking_teams_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterMatchmakingTeams |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterMatchmakingTeamsResponse**](GameCenterMatchmakingTeamsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_rule_sets_update_instance

> models::GameCenterMatchmakingRuleSetResponse game_center_matchmaking_rule_sets_update_instance(id, game_center_matchmaking_rule_set_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_matchmaking_rule_set_update_request** | [**GameCenterMatchmakingRuleSetUpdateRequest**](GameCenterMatchmakingRuleSetUpdateRequest.md) | GameCenterMatchmakingRuleSet representation | [required] |

### Return type

[**models::GameCenterMatchmakingRuleSetResponse**](GameCenterMatchmakingRuleSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \GameCenterMatchmakingTeamsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_matchmaking_teams_create_instance**](GameCenterMatchmakingTeamsApi.md#game_center_matchmaking_teams_create_instance) | **POST** /v1/gameCenterMatchmakingTeams | 
[**game_center_matchmaking_teams_delete_instance**](GameCenterMatchmakingTeamsApi.md#game_center_matchmaking_teams_delete_instance) | **DELETE** /v1/gameCenterMatchmakingTeams/{id} | 
[**game_center_matchmaking_teams_update_instance**](GameCenterMatchmakingTeamsApi.md#game_center_matchmaking_teams_update_instance) | **PATCH** /v1/gameCenterMatchmakingTeams/{id} | 



## game_center_matchmaking_teams_create_instance

> models::GameCenterMatchmakingTeamResponse game_center_matchmaking_teams_create_instance(game_center_matchmaking_team_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_matchmaking_team_create_request** | [**GameCenterMatchmakingTeamCreateRequest**](GameCenterMatchmakingTeamCreateRequest.md) | GameCenterMatchmakingTeam representation | [required] |

### Return type

[**models::GameCenterMatchmakingTeamResponse**](GameCenterMatchmakingTeamResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_matchmaking_teams_delete_instance

> game_center_matchmaking_teams_delete_instance(id)


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


## game_center_matchmaking_teams_update_instance

> models::GameCenterMatchmakingTeamResponse game_center_matchmaking_teams_update_instance(id, game_center_matchmaking_team_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_matchmaking_team_update_request** | [**GameCenterMatchmakingTeamUpdateRequest**](GameCenterMatchmakingTeamUpdateRequest.md) | GameCenterMatchmakingTeam representation | [required] |

### Return type

[**models::GameCenterMatchmakingTeamResponse**](GameCenterMatchmakingTeamResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


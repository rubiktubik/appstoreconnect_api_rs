# \GameCenterLeaderboardSetReleasesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_set_releases_create_instance**](GameCenterLeaderboardSetReleasesApi.md#game_center_leaderboard_set_releases_create_instance) | **POST** /v1/gameCenterLeaderboardSetReleases | 
[**game_center_leaderboard_set_releases_delete_instance**](GameCenterLeaderboardSetReleasesApi.md#game_center_leaderboard_set_releases_delete_instance) | **DELETE** /v1/gameCenterLeaderboardSetReleases/{id} | 
[**game_center_leaderboard_set_releases_get_instance**](GameCenterLeaderboardSetReleasesApi.md#game_center_leaderboard_set_releases_get_instance) | **GET** /v1/gameCenterLeaderboardSetReleases/{id} | 



## game_center_leaderboard_set_releases_create_instance

> models::GameCenterLeaderboardSetReleaseResponse game_center_leaderboard_set_releases_create_instance(game_center_leaderboard_set_release_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_set_release_create_request** | [**GameCenterLeaderboardSetReleaseCreateRequest**](GameCenterLeaderboardSetReleaseCreateRequest.md) | GameCenterLeaderboardSetRelease representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetReleaseResponse**](GameCenterLeaderboardSetReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_releases_delete_instance

> game_center_leaderboard_set_releases_delete_instance(id)


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


## game_center_leaderboard_set_releases_get_instance

> models::GameCenterLeaderboardSetReleaseResponse game_center_leaderboard_set_releases_get_instance(id, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetReleaseResponse**](GameCenterLeaderboardSetReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


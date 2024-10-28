# \GameCenterAchievementReleasesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_achievement_releases_create_instance**](GameCenterAchievementReleasesApi.md#game_center_achievement_releases_create_instance) | **POST** /v1/gameCenterAchievementReleases | 
[**game_center_achievement_releases_delete_instance**](GameCenterAchievementReleasesApi.md#game_center_achievement_releases_delete_instance) | **DELETE** /v1/gameCenterAchievementReleases/{id} | 
[**game_center_achievement_releases_get_instance**](GameCenterAchievementReleasesApi.md#game_center_achievement_releases_get_instance) | **GET** /v1/gameCenterAchievementReleases/{id} | 



## game_center_achievement_releases_create_instance

> models::GameCenterAchievementReleaseResponse game_center_achievement_releases_create_instance(game_center_achievement_release_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_achievement_release_create_request** | [**GameCenterAchievementReleaseCreateRequest**](GameCenterAchievementReleaseCreateRequest.md) | GameCenterAchievementRelease representation | [required] |

### Return type

[**models::GameCenterAchievementReleaseResponse**](GameCenterAchievementReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievement_releases_delete_instance

> game_center_achievement_releases_delete_instance(id)


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


## game_center_achievement_releases_get_instance

> models::GameCenterAchievementReleaseResponse game_center_achievement_releases_get_instance(id, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterAchievementReleaseResponse**](GameCenterAchievementReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


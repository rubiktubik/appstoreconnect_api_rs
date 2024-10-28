# \GameCenterLeaderboardSetImagesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_set_images_create_instance**](GameCenterLeaderboardSetImagesApi.md#game_center_leaderboard_set_images_create_instance) | **POST** /v1/gameCenterLeaderboardSetImages | 
[**game_center_leaderboard_set_images_delete_instance**](GameCenterLeaderboardSetImagesApi.md#game_center_leaderboard_set_images_delete_instance) | **DELETE** /v1/gameCenterLeaderboardSetImages/{id} | 
[**game_center_leaderboard_set_images_get_instance**](GameCenterLeaderboardSetImagesApi.md#game_center_leaderboard_set_images_get_instance) | **GET** /v1/gameCenterLeaderboardSetImages/{id} | 
[**game_center_leaderboard_set_images_update_instance**](GameCenterLeaderboardSetImagesApi.md#game_center_leaderboard_set_images_update_instance) | **PATCH** /v1/gameCenterLeaderboardSetImages/{id} | 



## game_center_leaderboard_set_images_create_instance

> models::GameCenterLeaderboardSetImageResponse game_center_leaderboard_set_images_create_instance(game_center_leaderboard_set_image_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_set_image_create_request** | [**GameCenterLeaderboardSetImageCreateRequest**](GameCenterLeaderboardSetImageCreateRequest.md) | GameCenterLeaderboardSetImage representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetImageResponse**](GameCenterLeaderboardSetImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_images_delete_instance

> game_center_leaderboard_set_images_delete_instance(id)


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


## game_center_leaderboard_set_images_get_instance

> models::GameCenterLeaderboardSetImageResponse game_center_leaderboard_set_images_get_instance(id, fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetImageResponse**](GameCenterLeaderboardSetImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_images_update_instance

> models::GameCenterLeaderboardSetImageResponse game_center_leaderboard_set_images_update_instance(id, game_center_leaderboard_set_image_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_image_update_request** | [**GameCenterLeaderboardSetImageUpdateRequest**](GameCenterLeaderboardSetImageUpdateRequest.md) | GameCenterLeaderboardSetImage representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetImageResponse**](GameCenterLeaderboardSetImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


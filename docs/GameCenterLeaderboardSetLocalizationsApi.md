# \GameCenterLeaderboardSetLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_set_localizations_create_instance**](GameCenterLeaderboardSetLocalizationsApi.md#game_center_leaderboard_set_localizations_create_instance) | **POST** /v1/gameCenterLeaderboardSetLocalizations | 
[**game_center_leaderboard_set_localizations_delete_instance**](GameCenterLeaderboardSetLocalizationsApi.md#game_center_leaderboard_set_localizations_delete_instance) | **DELETE** /v1/gameCenterLeaderboardSetLocalizations/{id} | 
[**game_center_leaderboard_set_localizations_game_center_leaderboard_set_image_get_to_one_related**](GameCenterLeaderboardSetLocalizationsApi.md#game_center_leaderboard_set_localizations_game_center_leaderboard_set_image_get_to_one_related) | **GET** /v1/gameCenterLeaderboardSetLocalizations/{id}/gameCenterLeaderboardSetImage | 
[**game_center_leaderboard_set_localizations_get_instance**](GameCenterLeaderboardSetLocalizationsApi.md#game_center_leaderboard_set_localizations_get_instance) | **GET** /v1/gameCenterLeaderboardSetLocalizations/{id} | 
[**game_center_leaderboard_set_localizations_update_instance**](GameCenterLeaderboardSetLocalizationsApi.md#game_center_leaderboard_set_localizations_update_instance) | **PATCH** /v1/gameCenterLeaderboardSetLocalizations/{id} | 



## game_center_leaderboard_set_localizations_create_instance

> models::GameCenterLeaderboardSetLocalizationResponse game_center_leaderboard_set_localizations_create_instance(game_center_leaderboard_set_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_set_localization_create_request** | [**GameCenterLeaderboardSetLocalizationCreateRequest**](GameCenterLeaderboardSetLocalizationCreateRequest.md) | GameCenterLeaderboardSetLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetLocalizationResponse**](GameCenterLeaderboardSetLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_localizations_delete_instance

> game_center_leaderboard_set_localizations_delete_instance(id)


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


## game_center_leaderboard_set_localizations_game_center_leaderboard_set_image_get_to_one_related

> models::GameCenterLeaderboardSetImageResponse game_center_leaderboard_set_localizations_game_center_leaderboard_set_image_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetImages |  |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetImageResponse**](GameCenterLeaderboardSetImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_localizations_get_instance

> models::GameCenterLeaderboardSetLocalizationResponse game_center_leaderboard_set_localizations_get_instance(id, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetLocalizationResponse**](GameCenterLeaderboardSetLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_localizations_update_instance

> models::GameCenterLeaderboardSetLocalizationResponse game_center_leaderboard_set_localizations_update_instance(id, game_center_leaderboard_set_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_localization_update_request** | [**GameCenterLeaderboardSetLocalizationUpdateRequest**](GameCenterLeaderboardSetLocalizationUpdateRequest.md) | GameCenterLeaderboardSetLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetLocalizationResponse**](GameCenterLeaderboardSetLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \GameCenterLeaderboardLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_localizations_create_instance**](GameCenterLeaderboardLocalizationsApi.md#game_center_leaderboard_localizations_create_instance) | **POST** /v1/gameCenterLeaderboardLocalizations | 
[**game_center_leaderboard_localizations_delete_instance**](GameCenterLeaderboardLocalizationsApi.md#game_center_leaderboard_localizations_delete_instance) | **DELETE** /v1/gameCenterLeaderboardLocalizations/{id} | 
[**game_center_leaderboard_localizations_game_center_leaderboard_image_get_to_one_related**](GameCenterLeaderboardLocalizationsApi.md#game_center_leaderboard_localizations_game_center_leaderboard_image_get_to_one_related) | **GET** /v1/gameCenterLeaderboardLocalizations/{id}/gameCenterLeaderboardImage | 
[**game_center_leaderboard_localizations_get_instance**](GameCenterLeaderboardLocalizationsApi.md#game_center_leaderboard_localizations_get_instance) | **GET** /v1/gameCenterLeaderboardLocalizations/{id} | 
[**game_center_leaderboard_localizations_update_instance**](GameCenterLeaderboardLocalizationsApi.md#game_center_leaderboard_localizations_update_instance) | **PATCH** /v1/gameCenterLeaderboardLocalizations/{id} | 



## game_center_leaderboard_localizations_create_instance

> models::GameCenterLeaderboardLocalizationResponse game_center_leaderboard_localizations_create_instance(game_center_leaderboard_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_localization_create_request** | [**GameCenterLeaderboardLocalizationCreateRequest**](GameCenterLeaderboardLocalizationCreateRequest.md) | GameCenterLeaderboardLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardLocalizationResponse**](GameCenterLeaderboardLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_localizations_delete_instance

> game_center_leaderboard_localizations_delete_instance(id)


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


## game_center_leaderboard_localizations_game_center_leaderboard_image_get_to_one_related

> models::GameCenterLeaderboardImageResponse game_center_leaderboard_localizations_game_center_leaderboard_image_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardImages |  |
**fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardImageResponse**](GameCenterLeaderboardImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_localizations_get_instance

> models::GameCenterLeaderboardLocalizationResponse game_center_leaderboard_localizations_get_instance(id, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardLocalizationResponse**](GameCenterLeaderboardLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_localizations_update_instance

> models::GameCenterLeaderboardLocalizationResponse game_center_leaderboard_localizations_update_instance(id, game_center_leaderboard_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_localization_update_request** | [**GameCenterLeaderboardLocalizationUpdateRequest**](GameCenterLeaderboardLocalizationUpdateRequest.md) | GameCenterLeaderboardLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardLocalizationResponse**](GameCenterLeaderboardLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


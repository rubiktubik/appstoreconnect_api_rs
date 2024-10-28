# \GameCenterAchievementLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_achievement_localizations_create_instance**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_create_instance) | **POST** /v1/gameCenterAchievementLocalizations | 
[**game_center_achievement_localizations_delete_instance**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_delete_instance) | **DELETE** /v1/gameCenterAchievementLocalizations/{id} | 
[**game_center_achievement_localizations_game_center_achievement_get_to_one_related**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_game_center_achievement_get_to_one_related) | **GET** /v1/gameCenterAchievementLocalizations/{id}/gameCenterAchievement | 
[**game_center_achievement_localizations_game_center_achievement_image_get_to_one_related**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_game_center_achievement_image_get_to_one_related) | **GET** /v1/gameCenterAchievementLocalizations/{id}/gameCenterAchievementImage | 
[**game_center_achievement_localizations_get_instance**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_get_instance) | **GET** /v1/gameCenterAchievementLocalizations/{id} | 
[**game_center_achievement_localizations_update_instance**](GameCenterAchievementLocalizationsApi.md#game_center_achievement_localizations_update_instance) | **PATCH** /v1/gameCenterAchievementLocalizations/{id} | 



## game_center_achievement_localizations_create_instance

> models::GameCenterAchievementLocalizationResponse game_center_achievement_localizations_create_instance(game_center_achievement_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_achievement_localization_create_request** | [**GameCenterAchievementLocalizationCreateRequest**](GameCenterAchievementLocalizationCreateRequest.md) | GameCenterAchievementLocalization representation | [required] |

### Return type

[**models::GameCenterAchievementLocalizationResponse**](GameCenterAchievementLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievement_localizations_delete_instance

> game_center_achievement_localizations_delete_instance(id)


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


## game_center_achievement_localizations_game_center_achievement_get_to_one_related

> models::GameCenterAchievementResponse game_center_achievement_localizations_game_center_achievement_get_to_one_related(id, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementLocalizations |  |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterAchievementResponse**](GameCenterAchievementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievement_localizations_game_center_achievement_image_get_to_one_related

> models::GameCenterAchievementImageResponse game_center_achievement_localizations_game_center_achievement_image_get_to_one_related(id, fields_left_square_bracket_game_center_achievement_images_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievement_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementImages |  |
**fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementLocalizations |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterAchievementImageResponse**](GameCenterAchievementImageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievement_localizations_get_instance

> models::GameCenterAchievementLocalizationResponse game_center_achievement_localizations_get_instance(id, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_achievement_images_right_square_bracket, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementLocalizations |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_achievement_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementImages |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterAchievementLocalizationResponse**](GameCenterAchievementLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievement_localizations_update_instance

> models::GameCenterAchievementLocalizationResponse game_center_achievement_localizations_update_instance(id, game_center_achievement_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_achievement_localization_update_request** | [**GameCenterAchievementLocalizationUpdateRequest**](GameCenterAchievementLocalizationUpdateRequest.md) | GameCenterAchievementLocalization representation | [required] |

### Return type

[**models::GameCenterAchievementLocalizationResponse**](GameCenterAchievementLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \GameCenterLeaderboardSetMemberLocalizationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_set_member_localizations_create_instance**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_create_instance) | **POST** /v1/gameCenterLeaderboardSetMemberLocalizations | 
[**game_center_leaderboard_set_member_localizations_delete_instance**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_delete_instance) | **DELETE** /v1/gameCenterLeaderboardSetMemberLocalizations/{id} | 
[**game_center_leaderboard_set_member_localizations_game_center_leaderboard_get_to_one_related**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_game_center_leaderboard_get_to_one_related) | **GET** /v1/gameCenterLeaderboardSetMemberLocalizations/{id}/gameCenterLeaderboard | 
[**game_center_leaderboard_set_member_localizations_game_center_leaderboard_set_get_to_one_related**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_game_center_leaderboard_set_get_to_one_related) | **GET** /v1/gameCenterLeaderboardSetMemberLocalizations/{id}/gameCenterLeaderboardSet | 
[**game_center_leaderboard_set_member_localizations_get_collection**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_get_collection) | **GET** /v1/gameCenterLeaderboardSetMemberLocalizations | 
[**game_center_leaderboard_set_member_localizations_update_instance**](GameCenterLeaderboardSetMemberLocalizationsApi.md#game_center_leaderboard_set_member_localizations_update_instance) | **PATCH** /v1/gameCenterLeaderboardSetMemberLocalizations/{id} | 



## game_center_leaderboard_set_member_localizations_create_instance

> models::GameCenterLeaderboardSetMemberLocalizationResponse game_center_leaderboard_set_member_localizations_create_instance(game_center_leaderboard_set_member_localization_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_set_member_localization_create_request** | [**GameCenterLeaderboardSetMemberLocalizationCreateRequest**](GameCenterLeaderboardSetMemberLocalizationCreateRequest.md) | GameCenterLeaderboardSetMemberLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetMemberLocalizationResponse**](GameCenterLeaderboardSetMemberLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_member_localizations_delete_instance

> game_center_leaderboard_set_member_localizations_delete_instance(id)


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


## game_center_leaderboard_set_member_localizations_game_center_leaderboard_get_to_one_related

> models::GameCenterLeaderboardResponse game_center_leaderboard_set_member_localizations_game_center_leaderboard_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterLeaderboardResponse**](GameCenterLeaderboardResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_member_localizations_game_center_leaderboard_set_get_to_one_related

> models::GameCenterLeaderboardSetResponse game_center_leaderboard_set_member_localizations_game_center_leaderboard_set_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterLeaderboardSetResponse**](GameCenterLeaderboardSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_member_localizations_get_collection

> models::GameCenterLeaderboardSetMemberLocalizationsResponse game_center_leaderboard_set_member_localizations_get_collection(filter_left_square_bracket_game_center_leaderboard_set_right_square_bracket, filter_left_square_bracket_game_center_leaderboard_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_member_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_game_center_leaderboard_set_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) of related 'gameCenterLeaderboardSet' | [required] |
**filter_left_square_bracket_game_center_leaderboard_right_square_bracket** | [**Vec<String>**](String.md) | filter by id(s) of related 'gameCenterLeaderboard' | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_member_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetMemberLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetMemberLocalizationsResponse**](GameCenterLeaderboardSetMemberLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_set_member_localizations_update_instance

> models::GameCenterLeaderboardSetMemberLocalizationResponse game_center_leaderboard_set_member_localizations_update_instance(id, game_center_leaderboard_set_member_localization_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_member_localization_update_request** | [**GameCenterLeaderboardSetMemberLocalizationUpdateRequest**](GameCenterLeaderboardSetMemberLocalizationUpdateRequest.md) | GameCenterLeaderboardSetMemberLocalization representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetMemberLocalizationResponse**](GameCenterLeaderboardSetMemberLocalizationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


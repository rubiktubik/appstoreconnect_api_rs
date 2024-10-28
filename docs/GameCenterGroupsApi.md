# \GameCenterGroupsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_groups_create_instance**](GameCenterGroupsApi.md#game_center_groups_create_instance) | **POST** /v1/gameCenterGroups | 
[**game_center_groups_delete_instance**](GameCenterGroupsApi.md#game_center_groups_delete_instance) | **DELETE** /v1/gameCenterGroups/{id} | 
[**game_center_groups_game_center_achievements_get_to_many_related**](GameCenterGroupsApi.md#game_center_groups_game_center_achievements_get_to_many_related) | **GET** /v1/gameCenterGroups/{id}/gameCenterAchievements | 
[**game_center_groups_game_center_achievements_get_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_achievements_get_to_many_relationship) | **GET** /v1/gameCenterGroups/{id}/relationships/gameCenterAchievements | 
[**game_center_groups_game_center_achievements_replace_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_achievements_replace_to_many_relationship) | **PATCH** /v1/gameCenterGroups/{id}/relationships/gameCenterAchievements | 
[**game_center_groups_game_center_details_get_to_many_related**](GameCenterGroupsApi.md#game_center_groups_game_center_details_get_to_many_related) | **GET** /v1/gameCenterGroups/{id}/gameCenterDetails | 
[**game_center_groups_game_center_leaderboard_sets_get_to_many_related**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboard_sets_get_to_many_related) | **GET** /v1/gameCenterGroups/{id}/gameCenterLeaderboardSets | 
[**game_center_groups_game_center_leaderboard_sets_get_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboard_sets_get_to_many_relationship) | **GET** /v1/gameCenterGroups/{id}/relationships/gameCenterLeaderboardSets | 
[**game_center_groups_game_center_leaderboard_sets_replace_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboard_sets_replace_to_many_relationship) | **PATCH** /v1/gameCenterGroups/{id}/relationships/gameCenterLeaderboardSets | 
[**game_center_groups_game_center_leaderboards_get_to_many_related**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboards_get_to_many_related) | **GET** /v1/gameCenterGroups/{id}/gameCenterLeaderboards | 
[**game_center_groups_game_center_leaderboards_get_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboards_get_to_many_relationship) | **GET** /v1/gameCenterGroups/{id}/relationships/gameCenterLeaderboards | 
[**game_center_groups_game_center_leaderboards_replace_to_many_relationship**](GameCenterGroupsApi.md#game_center_groups_game_center_leaderboards_replace_to_many_relationship) | **PATCH** /v1/gameCenterGroups/{id}/relationships/gameCenterLeaderboards | 
[**game_center_groups_get_collection**](GameCenterGroupsApi.md#game_center_groups_get_collection) | **GET** /v1/gameCenterGroups | 
[**game_center_groups_get_instance**](GameCenterGroupsApi.md#game_center_groups_get_instance) | **GET** /v1/gameCenterGroups/{id} | 
[**game_center_groups_update_instance**](GameCenterGroupsApi.md#game_center_groups_update_instance) | **PATCH** /v1/gameCenterGroups/{id} | 



## game_center_groups_create_instance

> models::GameCenterGroupResponse game_center_groups_create_instance(game_center_group_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_group_create_request** | [**GameCenterGroupCreateRequest**](GameCenterGroupCreateRequest.md) | GameCenterGroup representation | [required] |

### Return type

[**models::GameCenterGroupResponse**](GameCenterGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_delete_instance

> game_center_groups_delete_instance(id)


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


## game_center_groups_game_center_achievements_get_to_many_related

> models::GameCenterAchievementsResponse game_center_groups_game_center_achievements_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_archived_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, limit, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_reference_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'referenceName' |  |
**filter_left_square_bracket_archived_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'archived' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementLocalizations |  |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterAchievementsResponse**](GameCenterAchievementsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_achievements_get_to_many_relationship

> models::GameCenterGroupGameCenterAchievementsLinkagesResponse game_center_groups_game_center_achievements_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterGroupGameCenterAchievementsLinkagesResponse**](GameCenterGroupGameCenterAchievementsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_achievements_replace_to_many_relationship

> game_center_groups_game_center_achievements_replace_to_many_relationship(id, game_center_group_game_center_achievements_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_group_game_center_achievements_linkages_request** | [**GameCenterGroupGameCenterAchievementsLinkagesRequest**](GameCenterGroupGameCenterAchievementsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_details_get_to_many_related

> models::GameCenterDetailsResponse game_center_groups_game_center_details_get_to_many_related(id, filter_left_square_bracket_game_center_app_versions_period_enabled_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, limit, include, limit_left_square_bracket_game_center_app_versions_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_game_center_achievements_right_square_bracket, limit_left_square_bracket_achievement_releases_right_square_bracket, limit_left_square_bracket_leaderboard_releases_right_square_bracket, limit_left_square_bracket_leaderboard_set_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_game_center_app_versions_period_enabled_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'gameCenterAppVersions.enabled' |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardReleases |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAppVersions returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_game_center_achievements_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAchievements returned (when they are included) |  |
**limit_left_square_bracket_achievement_releases_right_square_bracket** | Option<**i32**> | maximum number of related achievementReleases returned (when they are included) |  |
**limit_left_square_bracket_leaderboard_releases_right_square_bracket** | Option<**i32**> | maximum number of related leaderboardReleases returned (when they are included) |  |
**limit_left_square_bracket_leaderboard_set_releases_right_square_bracket** | Option<**i32**> | maximum number of related leaderboardSetReleases returned (when they are included) |  |

### Return type

[**models::GameCenterDetailsResponse**](GameCenterDetailsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboard_sets_get_to_many_related

> models::GameCenterLeaderboardSetsResponse game_center_groups_game_center_leaderboard_sets_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, limit, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_reference_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'referenceName' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterLeaderboardSetsResponse**](GameCenterLeaderboardSetsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboard_sets_get_to_many_relationship

> models::GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse game_center_groups_game_center_leaderboard_sets_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse**](GameCenterGroupGameCenterLeaderboardSetsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboard_sets_replace_to_many_relationship

> game_center_groups_game_center_leaderboard_sets_replace_to_many_relationship(id, game_center_group_game_center_leaderboard_sets_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_group_game_center_leaderboard_sets_linkages_request** | [**GameCenterGroupGameCenterLeaderboardSetsLinkagesRequest**](GameCenterGroupGameCenterLeaderboardSetsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboards_get_to_many_related

> models::GameCenterLeaderboardsResponse game_center_groups_game_center_leaderboards_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_archived_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, limit, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_reference_name_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'referenceName' |  |
**filter_left_square_bracket_archived_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'archived' |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardReleases |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterLeaderboardsResponse**](GameCenterLeaderboardsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboards_get_to_many_relationship

> models::GameCenterGroupGameCenterLeaderboardsLinkagesResponse game_center_groups_game_center_leaderboards_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterGroupGameCenterLeaderboardsLinkagesResponse**](GameCenterGroupGameCenterLeaderboardsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_game_center_leaderboards_replace_to_many_relationship

> game_center_groups_game_center_leaderboards_replace_to_many_relationship(id, game_center_group_game_center_leaderboards_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_group_game_center_leaderboards_linkages_request** | [**GameCenterGroupGameCenterLeaderboardsLinkagesRequest**](GameCenterGroupGameCenterLeaderboardsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_get_collection

> models::GameCenterGroupsResponse game_center_groups_get_collection(filter_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, limit, include, limit_left_square_bracket_game_center_achievements_right_square_bracket, limit_left_square_bracket_game_center_details_right_square_bracket, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterDetails' |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_achievements_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAchievements returned (when they are included) |  |
**limit_left_square_bracket_game_center_details_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterDetails returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |

### Return type

[**models::GameCenterGroupsResponse**](GameCenterGroupsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_get_instance

> models::GameCenterGroupResponse game_center_groups_get_instance(id, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, include, limit_left_square_bracket_game_center_achievements_right_square_bracket, limit_left_square_bracket_game_center_details_right_square_bracket, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_achievements_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAchievements returned (when they are included) |  |
**limit_left_square_bracket_game_center_details_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterDetails returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |

### Return type

[**models::GameCenterGroupResponse**](GameCenterGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_groups_update_instance

> models::GameCenterGroupResponse game_center_groups_update_instance(id, game_center_group_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_group_update_request** | [**GameCenterGroupUpdateRequest**](GameCenterGroupUpdateRequest.md) | GameCenterGroup representation | [required] |

### Return type

[**models::GameCenterGroupResponse**](GameCenterGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


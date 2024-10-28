# \GameCenterDetailsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_details_achievement_releases_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_achievement_releases_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/achievementReleases | 
[**game_center_details_classic_matchmaking_requests_get_metrics**](GameCenterDetailsApi.md#game_center_details_classic_matchmaking_requests_get_metrics) | **GET** /v1/gameCenterDetails/{id}/metrics/classicMatchmakingRequests | 
[**game_center_details_create_instance**](GameCenterDetailsApi.md#game_center_details_create_instance) | **POST** /v1/gameCenterDetails | 
[**game_center_details_game_center_achievements_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_game_center_achievements_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/gameCenterAchievements | 
[**game_center_details_game_center_achievements_get_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_achievements_get_to_many_relationship) | **GET** /v1/gameCenterDetails/{id}/relationships/gameCenterAchievements | 
[**game_center_details_game_center_achievements_replace_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_achievements_replace_to_many_relationship) | **PATCH** /v1/gameCenterDetails/{id}/relationships/gameCenterAchievements | 
[**game_center_details_game_center_app_versions_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_game_center_app_versions_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/gameCenterAppVersions | 
[**game_center_details_game_center_group_get_to_one_related**](GameCenterDetailsApi.md#game_center_details_game_center_group_get_to_one_related) | **GET** /v1/gameCenterDetails/{id}/gameCenterGroup | 
[**game_center_details_game_center_leaderboard_sets_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboard_sets_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/gameCenterLeaderboardSets | 
[**game_center_details_game_center_leaderboard_sets_get_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboard_sets_get_to_many_relationship) | **GET** /v1/gameCenterDetails/{id}/relationships/gameCenterLeaderboardSets | 
[**game_center_details_game_center_leaderboard_sets_replace_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboard_sets_replace_to_many_relationship) | **PATCH** /v1/gameCenterDetails/{id}/relationships/gameCenterLeaderboardSets | 
[**game_center_details_game_center_leaderboards_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboards_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/gameCenterLeaderboards | 
[**game_center_details_game_center_leaderboards_get_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboards_get_to_many_relationship) | **GET** /v1/gameCenterDetails/{id}/relationships/gameCenterLeaderboards | 
[**game_center_details_game_center_leaderboards_replace_to_many_relationship**](GameCenterDetailsApi.md#game_center_details_game_center_leaderboards_replace_to_many_relationship) | **PATCH** /v1/gameCenterDetails/{id}/relationships/gameCenterLeaderboards | 
[**game_center_details_get_instance**](GameCenterDetailsApi.md#game_center_details_get_instance) | **GET** /v1/gameCenterDetails/{id} | 
[**game_center_details_leaderboard_releases_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_leaderboard_releases_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/leaderboardReleases | 
[**game_center_details_leaderboard_set_releases_get_to_many_related**](GameCenterDetailsApi.md#game_center_details_leaderboard_set_releases_get_to_many_related) | **GET** /v1/gameCenterDetails/{id}/leaderboardSetReleases | 
[**game_center_details_rule_based_matchmaking_requests_get_metrics**](GameCenterDetailsApi.md#game_center_details_rule_based_matchmaking_requests_get_metrics) | **GET** /v1/gameCenterDetails/{id}/metrics/ruleBasedMatchmakingRequests | 
[**game_center_details_update_instance**](GameCenterDetailsApi.md#game_center_details_update_instance) | **PATCH** /v1/gameCenterDetails/{id} | 



## game_center_details_achievement_releases_get_to_many_related

> models::GameCenterAchievementReleasesResponse game_center_details_achievement_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_achievement_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_achievement_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterAchievement' |  |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterAchievementReleasesResponse**](GameCenterAchievementReleasesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_classic_matchmaking_requests_get_metrics

> models::GameCenterMatchmakingAppRequestsV1MetricResponse game_center_details_classic_matchmaking_requests_get_metrics(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingAppRequestsV1MetricResponse**](GameCenterMatchmakingAppRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_create_instance

> models::GameCenterDetailResponse game_center_details_create_instance(game_center_detail_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_detail_create_request** | [**GameCenterDetailCreateRequest**](GameCenterDetailCreateRequest.md) | GameCenterDetail representation | [required] |

### Return type

[**models::GameCenterDetailResponse**](GameCenterDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_achievements_get_to_many_related

> models::GameCenterAchievementsResponse game_center_details_game_center_achievements_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_archived_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, limit, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_details_game_center_achievements_get_to_many_relationship

> models::GameCenterDetailGameCenterAchievementsLinkagesResponse game_center_details_game_center_achievements_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterDetailGameCenterAchievementsLinkagesResponse**](GameCenterDetailGameCenterAchievementsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_achievements_replace_to_many_relationship

> game_center_details_game_center_achievements_replace_to_many_relationship(id, game_center_detail_game_center_achievements_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_detail_game_center_achievements_linkages_request** | [**GameCenterDetailGameCenterAchievementsLinkagesRequest**](GameCenterDetailGameCenterAchievementsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_app_versions_get_to_many_related

> models::GameCenterAppVersionsResponse game_center_details_game_center_app_versions_get_to_many_related(id, filter_left_square_bracket_enabled_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_app_store_versions_right_square_bracket, limit, include, limit_left_square_bracket_compatibility_versions_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_enabled_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'enabled' |  |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_app_store_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type appStoreVersions |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_compatibility_versions_right_square_bracket** | Option<**i32**> | maximum number of related compatibilityVersions returned (when they are included) |  |

### Return type

[**models::GameCenterAppVersionsResponse**](GameCenterAppVersionsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_group_get_to_one_related

> models::GameCenterGroupResponse game_center_details_game_center_group_get_to_one_related(id, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, include, limit_left_square_bracket_game_center_details_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_game_center_achievements_right_square_bracket)


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
**limit_left_square_bracket_game_center_details_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterDetails returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_game_center_achievements_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAchievements returned (when they are included) |  |

### Return type

[**models::GameCenterGroupResponse**](GameCenterGroupResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_leaderboard_sets_get_to_many_related

> models::GameCenterLeaderboardSetsResponse game_center_details_game_center_leaderboard_sets_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, limit, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_details_game_center_leaderboard_sets_get_to_many_relationship

> models::GameCenterDetailGameCenterLeaderboardSetsLinkagesResponse game_center_details_game_center_leaderboard_sets_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterDetailGameCenterLeaderboardSetsLinkagesResponse**](GameCenterDetailGameCenterLeaderboardSetsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_leaderboard_sets_replace_to_many_relationship

> game_center_details_game_center_leaderboard_sets_replace_to_many_relationship(id, game_center_detail_game_center_leaderboard_sets_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_detail_game_center_leaderboard_sets_linkages_request** | [**GameCenterDetailGameCenterLeaderboardSetsLinkagesRequest**](GameCenterDetailGameCenterLeaderboardSetsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_leaderboards_get_to_many_related

> models::GameCenterLeaderboardsResponse game_center_details_game_center_leaderboards_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_archived_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, limit, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_details_game_center_leaderboards_get_to_many_relationship

> models::GameCenterDetailGameCenterLeaderboardsLinkagesResponse game_center_details_game_center_leaderboards_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterDetailGameCenterLeaderboardsLinkagesResponse**](GameCenterDetailGameCenterLeaderboardsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_game_center_leaderboards_replace_to_many_relationship

> game_center_details_game_center_leaderboards_replace_to_many_relationship(id, game_center_detail_game_center_leaderboards_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_detail_game_center_leaderboards_linkages_request** | [**GameCenterDetailGameCenterLeaderboardsLinkagesRequest**](GameCenterDetailGameCenterLeaderboardsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_get_instance

> models::GameCenterDetailResponse game_center_details_get_instance(id, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_app_versions_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, include, limit_left_square_bracket_achievement_releases_right_square_bracket, limit_left_square_bracket_game_center_achievements_right_square_bracket, limit_left_square_bracket_game_center_app_versions_right_square_bracket, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_leaderboard_releases_right_square_bracket, limit_left_square_bracket_leaderboard_set_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAppVersions |  |
**fields_left_square_bracket_game_center_groups_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterGroups |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_achievement_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementReleases |  |
**fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardReleases |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_achievement_releases_right_square_bracket** | Option<**i32**> | maximum number of related achievementReleases returned (when they are included) |  |
**limit_left_square_bracket_game_center_achievements_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAchievements returned (when they are included) |  |
**limit_left_square_bracket_game_center_app_versions_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterAppVersions returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboardSets returned (when they are included) |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_leaderboard_releases_right_square_bracket** | Option<**i32**> | maximum number of related leaderboardReleases returned (when they are included) |  |
**limit_left_square_bracket_leaderboard_set_releases_right_square_bracket** | Option<**i32**> | maximum number of related leaderboardSetReleases returned (when they are included) |  |

### Return type

[**models::GameCenterDetailResponse**](GameCenterDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_leaderboard_releases_get_to_many_related

> models::GameCenterLeaderboardReleasesResponse game_center_details_leaderboard_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_leaderboard_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_leaderboard_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterLeaderboard' |  |
**fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardReleases |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardReleasesResponse**](GameCenterLeaderboardReleasesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_leaderboard_set_releases_get_to_many_related

> models::GameCenterLeaderboardSetReleasesResponse game_center_details_leaderboard_set_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_leaderboard_set_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_leaderboard_set_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterLeaderboardSet' |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**fields_left_square_bracket_game_center_details_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterDetails |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetReleasesResponse**](GameCenterLeaderboardSetReleasesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_rule_based_matchmaking_requests_get_metrics

> models::GameCenterMatchmakingAppRequestsV1MetricResponse game_center_details_rule_based_matchmaking_requests_get_metrics(id, granularity, group_by, filter_left_square_bracket_result_right_square_bracket, sort, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**granularity** | **String** | the granularity of the per-group dataset | [required] |
**group_by** | Option<[**Vec<String>**](String.md)> | the dimension by which to group the results |  |
**filter_left_square_bracket_result_right_square_bracket** | Option<**String**> | filter by 'result' attribute dimension |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; metrics will be sorted as specified |  |
**limit** | Option<**i32**> | maximum number of groups to return per page |  |

### Return type

[**models::GameCenterMatchmakingAppRequestsV1MetricResponse**](GameCenterMatchmakingAppRequestsV1MetricResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_details_update_instance

> models::GameCenterDetailResponse game_center_details_update_instance(id, game_center_detail_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_detail_update_request** | [**GameCenterDetailUpdateRequest**](GameCenterDetailUpdateRequest.md) | GameCenterDetail representation | [required] |

### Return type

[**models::GameCenterDetailResponse**](GameCenterDetailResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


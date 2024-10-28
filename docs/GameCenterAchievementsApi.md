# \GameCenterAchievementsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_achievements_create_instance**](GameCenterAchievementsApi.md#game_center_achievements_create_instance) | **POST** /v1/gameCenterAchievements | 
[**game_center_achievements_delete_instance**](GameCenterAchievementsApi.md#game_center_achievements_delete_instance) | **DELETE** /v1/gameCenterAchievements/{id} | 
[**game_center_achievements_get_instance**](GameCenterAchievementsApi.md#game_center_achievements_get_instance) | **GET** /v1/gameCenterAchievements/{id} | 
[**game_center_achievements_group_achievement_get_to_one_related**](GameCenterAchievementsApi.md#game_center_achievements_group_achievement_get_to_one_related) | **GET** /v1/gameCenterAchievements/{id}/groupAchievement | 
[**game_center_achievements_group_achievement_get_to_one_relationship**](GameCenterAchievementsApi.md#game_center_achievements_group_achievement_get_to_one_relationship) | **GET** /v1/gameCenterAchievements/{id}/relationships/groupAchievement | 
[**game_center_achievements_group_achievement_update_to_one_relationship**](GameCenterAchievementsApi.md#game_center_achievements_group_achievement_update_to_one_relationship) | **PATCH** /v1/gameCenterAchievements/{id}/relationships/groupAchievement | 
[**game_center_achievements_localizations_get_to_many_related**](GameCenterAchievementsApi.md#game_center_achievements_localizations_get_to_many_related) | **GET** /v1/gameCenterAchievements/{id}/localizations | 
[**game_center_achievements_releases_get_to_many_related**](GameCenterAchievementsApi.md#game_center_achievements_releases_get_to_many_related) | **GET** /v1/gameCenterAchievements/{id}/releases | 
[**game_center_achievements_update_instance**](GameCenterAchievementsApi.md#game_center_achievements_update_instance) | **PATCH** /v1/gameCenterAchievements/{id} | 



## game_center_achievements_create_instance

> models::GameCenterAchievementResponse game_center_achievements_create_instance(game_center_achievement_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_achievement_create_request** | [**GameCenterAchievementCreateRequest**](GameCenterAchievementCreateRequest.md) | GameCenterAchievement representation | [required] |

### Return type

[**models::GameCenterAchievementResponse**](GameCenterAchievementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievements_delete_instance

> game_center_achievements_delete_instance(id)


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


## game_center_achievements_get_instance

> models::GameCenterAchievementResponse game_center_achievements_get_instance(id, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
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


## game_center_achievements_group_achievement_get_to_one_related

> models::GameCenterAchievementResponse game_center_achievements_group_achievement_get_to_one_related(id, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_achievements_group_achievement_get_to_one_relationship

> models::GameCenterAchievementGroupAchievementLinkageResponse game_center_achievements_group_achievement_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::GameCenterAchievementGroupAchievementLinkageResponse**](GameCenterAchievementGroupAchievementLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievements_group_achievement_update_to_one_relationship

> game_center_achievements_group_achievement_update_to_one_relationship(id, game_center_achievement_group_achievement_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_achievement_group_achievement_linkage_request** | [**GameCenterAchievementGroupAchievementLinkageRequest**](GameCenterAchievementGroupAchievementLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievements_localizations_get_to_many_related

> models::GameCenterAchievementLocalizationsResponse game_center_achievements_localizations_get_to_many_related(id, fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, fields_left_square_bracket_game_center_achievement_images_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_achievement_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementLocalizations |  |
**fields_left_square_bracket_game_center_achievements_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievements |  |
**fields_left_square_bracket_game_center_achievement_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterAchievementImages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterAchievementLocalizationsResponse**](GameCenterAchievementLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_achievements_releases_get_to_many_related

> models::GameCenterAchievementReleasesResponse game_center_achievements_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, fields_left_square_bracket_game_center_achievement_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_achievements_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_detail_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterDetail' |  |
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


## game_center_achievements_update_instance

> models::GameCenterAchievementResponse game_center_achievements_update_instance(id, game_center_achievement_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_achievement_update_request** | [**GameCenterAchievementUpdateRequest**](GameCenterAchievementUpdateRequest.md) | GameCenterAchievement representation | [required] |

### Return type

[**models::GameCenterAchievementResponse**](GameCenterAchievementResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


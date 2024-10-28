# \GameCenterLeaderboardsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboards_create_instance**](GameCenterLeaderboardsApi.md#game_center_leaderboards_create_instance) | **POST** /v1/gameCenterLeaderboards | 
[**game_center_leaderboards_delete_instance**](GameCenterLeaderboardsApi.md#game_center_leaderboards_delete_instance) | **DELETE** /v1/gameCenterLeaderboards/{id} | 
[**game_center_leaderboards_get_instance**](GameCenterLeaderboardsApi.md#game_center_leaderboards_get_instance) | **GET** /v1/gameCenterLeaderboards/{id} | 
[**game_center_leaderboards_group_leaderboard_get_to_one_related**](GameCenterLeaderboardsApi.md#game_center_leaderboards_group_leaderboard_get_to_one_related) | **GET** /v1/gameCenterLeaderboards/{id}/groupLeaderboard | 
[**game_center_leaderboards_group_leaderboard_get_to_one_relationship**](GameCenterLeaderboardsApi.md#game_center_leaderboards_group_leaderboard_get_to_one_relationship) | **GET** /v1/gameCenterLeaderboards/{id}/relationships/groupLeaderboard | 
[**game_center_leaderboards_group_leaderboard_update_to_one_relationship**](GameCenterLeaderboardsApi.md#game_center_leaderboards_group_leaderboard_update_to_one_relationship) | **PATCH** /v1/gameCenterLeaderboards/{id}/relationships/groupLeaderboard | 
[**game_center_leaderboards_localizations_get_to_many_related**](GameCenterLeaderboardsApi.md#game_center_leaderboards_localizations_get_to_many_related) | **GET** /v1/gameCenterLeaderboards/{id}/localizations | 
[**game_center_leaderboards_releases_get_to_many_related**](GameCenterLeaderboardsApi.md#game_center_leaderboards_releases_get_to_many_related) | **GET** /v1/gameCenterLeaderboards/{id}/releases | 
[**game_center_leaderboards_update_instance**](GameCenterLeaderboardsApi.md#game_center_leaderboards_update_instance) | **PATCH** /v1/gameCenterLeaderboards/{id} | 



## game_center_leaderboards_create_instance

> models::GameCenterLeaderboardResponse game_center_leaderboards_create_instance(game_center_leaderboard_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_create_request** | [**GameCenterLeaderboardCreateRequest**](GameCenterLeaderboardCreateRequest.md) | GameCenterLeaderboard representation | [required] |

### Return type

[**models::GameCenterLeaderboardResponse**](GameCenterLeaderboardResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboards_delete_instance

> game_center_leaderboards_delete_instance(id)


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


## game_center_leaderboards_get_instance

> models::GameCenterLeaderboardResponse game_center_leaderboards_get_instance(id, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
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


## game_center_leaderboards_group_leaderboard_get_to_one_related

> models::GameCenterLeaderboardResponse game_center_leaderboards_group_leaderboard_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_leaderboards_group_leaderboard_get_to_one_relationship

> models::GameCenterLeaderboardGroupLeaderboardLinkageResponse game_center_leaderboards_group_leaderboard_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::GameCenterLeaderboardGroupLeaderboardLinkageResponse**](GameCenterLeaderboardGroupLeaderboardLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboards_group_leaderboard_update_to_one_relationship

> game_center_leaderboards_group_leaderboard_update_to_one_relationship(id, game_center_leaderboard_group_leaderboard_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_group_leaderboard_linkage_request** | [**GameCenterLeaderboardGroupLeaderboardLinkageRequest**](GameCenterLeaderboardGroupLeaderboardLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboards_localizations_get_to_many_related

> models::GameCenterLeaderboardLocalizationsResponse game_center_leaderboards_localizations_get_to_many_related(id, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardLocalizations |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardImages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardLocalizationsResponse**](GameCenterLeaderboardLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboards_releases_get_to_many_related

> models::GameCenterLeaderboardReleasesResponse game_center_leaderboards_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_detail_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterDetail' |  |
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


## game_center_leaderboards_update_instance

> models::GameCenterLeaderboardResponse game_center_leaderboards_update_instance(id, game_center_leaderboard_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_update_request** | [**GameCenterLeaderboardUpdateRequest**](GameCenterLeaderboardUpdateRequest.md) | GameCenterLeaderboard representation | [required] |

### Return type

[**models::GameCenterLeaderboardResponse**](GameCenterLeaderboardResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


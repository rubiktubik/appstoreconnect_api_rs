# \GameCenterLeaderboardSetsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**game_center_leaderboard_sets_create_instance**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_create_instance) | **POST** /v1/gameCenterLeaderboardSets | 
[**game_center_leaderboard_sets_delete_instance**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_delete_instance) | **DELETE** /v1/gameCenterLeaderboardSets/{id} | 
[**game_center_leaderboard_sets_game_center_leaderboards_create_to_many_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_game_center_leaderboards_create_to_many_relationship) | **POST** /v1/gameCenterLeaderboardSets/{id}/relationships/gameCenterLeaderboards | 
[**game_center_leaderboard_sets_game_center_leaderboards_delete_to_many_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_game_center_leaderboards_delete_to_many_relationship) | **DELETE** /v1/gameCenterLeaderboardSets/{id}/relationships/gameCenterLeaderboards | 
[**game_center_leaderboard_sets_game_center_leaderboards_get_to_many_related**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_game_center_leaderboards_get_to_many_related) | **GET** /v1/gameCenterLeaderboardSets/{id}/gameCenterLeaderboards | 
[**game_center_leaderboard_sets_game_center_leaderboards_get_to_many_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_game_center_leaderboards_get_to_many_relationship) | **GET** /v1/gameCenterLeaderboardSets/{id}/relationships/gameCenterLeaderboards | 
[**game_center_leaderboard_sets_game_center_leaderboards_replace_to_many_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_game_center_leaderboards_replace_to_many_relationship) | **PATCH** /v1/gameCenterLeaderboardSets/{id}/relationships/gameCenterLeaderboards | 
[**game_center_leaderboard_sets_get_instance**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_get_instance) | **GET** /v1/gameCenterLeaderboardSets/{id} | 
[**game_center_leaderboard_sets_group_leaderboard_set_get_to_one_related**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_group_leaderboard_set_get_to_one_related) | **GET** /v1/gameCenterLeaderboardSets/{id}/groupLeaderboardSet | 
[**game_center_leaderboard_sets_group_leaderboard_set_get_to_one_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_group_leaderboard_set_get_to_one_relationship) | **GET** /v1/gameCenterLeaderboardSets/{id}/relationships/groupLeaderboardSet | 
[**game_center_leaderboard_sets_group_leaderboard_set_update_to_one_relationship**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_group_leaderboard_set_update_to_one_relationship) | **PATCH** /v1/gameCenterLeaderboardSets/{id}/relationships/groupLeaderboardSet | 
[**game_center_leaderboard_sets_localizations_get_to_many_related**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_localizations_get_to_many_related) | **GET** /v1/gameCenterLeaderboardSets/{id}/localizations | 
[**game_center_leaderboard_sets_releases_get_to_many_related**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_releases_get_to_many_related) | **GET** /v1/gameCenterLeaderboardSets/{id}/releases | 
[**game_center_leaderboard_sets_update_instance**](GameCenterLeaderboardSetsApi.md#game_center_leaderboard_sets_update_instance) | **PATCH** /v1/gameCenterLeaderboardSets/{id} | 



## game_center_leaderboard_sets_create_instance

> models::GameCenterLeaderboardSetResponse game_center_leaderboard_sets_create_instance(game_center_leaderboard_set_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_center_leaderboard_set_create_request** | [**GameCenterLeaderboardSetCreateRequest**](GameCenterLeaderboardSetCreateRequest.md) | GameCenterLeaderboardSet representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetResponse**](GameCenterLeaderboardSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_delete_instance

> game_center_leaderboard_sets_delete_instance(id)


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


## game_center_leaderboard_sets_game_center_leaderboards_create_to_many_relationship

> game_center_leaderboard_sets_game_center_leaderboards_create_to_many_relationship(id, game_center_leaderboard_set_game_center_leaderboards_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_game_center_leaderboards_linkages_request** | [**GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest**](GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_game_center_leaderboards_delete_to_many_relationship

> game_center_leaderboard_sets_game_center_leaderboards_delete_to_many_relationship(id, game_center_leaderboard_set_game_center_leaderboards_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_game_center_leaderboards_linkages_request** | [**GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest**](GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_game_center_leaderboards_get_to_many_related

> models::GameCenterLeaderboardsResponse game_center_leaderboard_sets_game_center_leaderboards_get_to_many_related(id, filter_left_square_bracket_reference_name_right_square_bracket, filter_left_square_bracket_archived_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_releases_right_square_bracket, limit, include, limit_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_leaderboard_sets_game_center_leaderboards_get_to_many_relationship

> models::GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesResponse game_center_leaderboard_sets_game_center_leaderboards_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesResponse**](GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_game_center_leaderboards_replace_to_many_relationship

> game_center_leaderboard_sets_game_center_leaderboards_replace_to_many_relationship(id, game_center_leaderboard_set_game_center_leaderboards_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_game_center_leaderboards_linkages_request** | [**GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest**](GameCenterLeaderboardSetGameCenterLeaderboardsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_get_instance

> models::GameCenterLeaderboardSetResponse game_center_leaderboard_sets_get_instance(id, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, include, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**fields_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboards |  |
**fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetReleases |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_game_center_leaderboards_right_square_bracket** | Option<**i32**> | maximum number of related gameCenterLeaderboards returned (when they are included) |  |
**limit_left_square_bracket_localizations_right_square_bracket** | Option<**i32**> | maximum number of related localizations returned (when they are included) |  |
**limit_left_square_bracket_releases_right_square_bracket** | Option<**i32**> | maximum number of related releases returned (when they are included) |  |

### Return type

[**models::GameCenterLeaderboardSetResponse**](GameCenterLeaderboardSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_group_leaderboard_set_get_to_one_related

> models::GameCenterLeaderboardSetResponse game_center_leaderboard_sets_group_leaderboard_set_get_to_one_related(id, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_groups_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboards_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, include, limit_left_square_bracket_localizations_right_square_bracket, limit_left_square_bracket_game_center_leaderboards_right_square_bracket, limit_left_square_bracket_releases_right_square_bracket)


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


## game_center_leaderboard_sets_group_leaderboard_set_get_to_one_relationship

> models::GameCenterLeaderboardSetGroupLeaderboardSetLinkageResponse game_center_leaderboard_sets_group_leaderboard_set_get_to_one_relationship(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |

### Return type

[**models::GameCenterLeaderboardSetGroupLeaderboardSetLinkageResponse**](GameCenterLeaderboardSetGroupLeaderboardSetLinkageResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_group_leaderboard_set_update_to_one_relationship

> game_center_leaderboard_sets_group_leaderboard_set_update_to_one_relationship(id, game_center_leaderboard_set_group_leaderboard_set_linkage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_group_leaderboard_set_linkage_request** | [**GameCenterLeaderboardSetGroupLeaderboardSetLinkageRequest**](GameCenterLeaderboardSetGroupLeaderboardSetLinkageRequest.md) | Related linkage | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_localizations_get_to_many_related

> models::GameCenterLeaderboardSetLocalizationsResponse game_center_leaderboard_sets_localizations_get_to_many_related(id, fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_game_center_leaderboard_set_localizations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetLocalizations |  |
**fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSets |  |
**fields_left_square_bracket_game_center_leaderboard_set_images_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type gameCenterLeaderboardSetImages |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |

### Return type

[**models::GameCenterLeaderboardSetLocalizationsResponse**](GameCenterLeaderboardSetLocalizationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_center_leaderboard_sets_releases_get_to_many_related

> models::GameCenterLeaderboardSetReleasesResponse game_center_leaderboard_sets_releases_get_to_many_related(id, filter_left_square_bracket_live_right_square_bracket, filter_left_square_bracket_game_center_detail_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_set_releases_right_square_bracket, fields_left_square_bracket_game_center_details_right_square_bracket, fields_left_square_bracket_game_center_leaderboard_sets_right_square_bracket, limit, include)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**filter_left_square_bracket_live_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'live' |  |
**filter_left_square_bracket_game_center_detail_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'gameCenterDetail' |  |
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


## game_center_leaderboard_sets_update_instance

> models::GameCenterLeaderboardSetResponse game_center_leaderboard_sets_update_instance(id, game_center_leaderboard_set_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**game_center_leaderboard_set_update_request** | [**GameCenterLeaderboardSetUpdateRequest**](GameCenterLeaderboardSetUpdateRequest.md) | GameCenterLeaderboardSet representation | [required] |

### Return type

[**models::GameCenterLeaderboardSetResponse**](GameCenterLeaderboardSetResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


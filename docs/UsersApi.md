# \UsersApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_delete_instance**](UsersApi.md#users_delete_instance) | **DELETE** /v1/users/{id} | 
[**users_get_collection**](UsersApi.md#users_get_collection) | **GET** /v1/users | 
[**users_get_instance**](UsersApi.md#users_get_instance) | **GET** /v1/users/{id} | 
[**users_update_instance**](UsersApi.md#users_update_instance) | **PATCH** /v1/users/{id} | 
[**users_visible_apps_create_to_many_relationship**](UsersApi.md#users_visible_apps_create_to_many_relationship) | **POST** /v1/users/{id}/relationships/visibleApps | 
[**users_visible_apps_delete_to_many_relationship**](UsersApi.md#users_visible_apps_delete_to_many_relationship) | **DELETE** /v1/users/{id}/relationships/visibleApps | 
[**users_visible_apps_get_to_many_related**](UsersApi.md#users_visible_apps_get_to_many_related) | **GET** /v1/users/{id}/visibleApps | 
[**users_visible_apps_get_to_many_relationship**](UsersApi.md#users_visible_apps_get_to_many_relationship) | **GET** /v1/users/{id}/relationships/visibleApps | 
[**users_visible_apps_replace_to_many_relationship**](UsersApi.md#users_visible_apps_replace_to_many_relationship) | **PATCH** /v1/users/{id}/relationships/visibleApps | 



## users_delete_instance

> users_delete_instance(id)


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


## users_get_collection

> models::UsersResponse users_get_collection(filter_left_square_bracket_username_right_square_bracket, filter_left_square_bracket_roles_right_square_bracket, filter_left_square_bracket_visible_apps_right_square_bracket, sort, fields_left_square_bracket_users_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include, limit_left_square_bracket_visible_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_username_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'username' |  |
**filter_left_square_bracket_roles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'roles' |  |
**filter_left_square_bracket_visible_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'visibleApps' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_users_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type users |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_visible_apps_right_square_bracket** | Option<**i32**> | maximum number of related visibleApps returned (when they are included) |  |

### Return type

[**models::UsersResponse**](UsersResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_instance

> models::UserResponse users_get_instance(id, fields_left_square_bracket_users_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include, limit_left_square_bracket_visible_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_users_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type users |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_visible_apps_right_square_bracket** | Option<**i32**> | maximum number of related visibleApps returned (when they are included) |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_instance

> models::UserResponse users_update_instance(id, user_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**user_update_request** | [**UserUpdateRequest**](UserUpdateRequest.md) | User representation | [required] |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_visible_apps_create_to_many_relationship

> users_visible_apps_create_to_many_relationship(id, user_visible_apps_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**user_visible_apps_linkages_request** | [**UserVisibleAppsLinkagesRequest**](UserVisibleAppsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_visible_apps_delete_to_many_relationship

> users_visible_apps_delete_to_many_relationship(id, user_visible_apps_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**user_visible_apps_linkages_request** | [**UserVisibleAppsLinkagesRequest**](UserVisibleAppsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_visible_apps_get_to_many_related

> models::AppsWithoutIncludesResponse users_visible_apps_get_to_many_related(id, fields_left_square_bracket_apps_right_square_bracket, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::AppsWithoutIncludesResponse**](AppsWithoutIncludesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_visible_apps_get_to_many_relationship

> models::UserVisibleAppsLinkagesResponse users_visible_apps_get_to_many_relationship(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**limit** | Option<**i32**> | maximum resources per page |  |

### Return type

[**models::UserVisibleAppsLinkagesResponse**](UserVisibleAppsLinkagesResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_visible_apps_replace_to_many_relationship

> users_visible_apps_replace_to_many_relationship(id, user_visible_apps_linkages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**user_visible_apps_linkages_request** | [**UserVisibleAppsLinkagesRequest**](UserVisibleAppsLinkagesRequest.md) | List of related linkages | [required] |

### Return type

 (empty response body)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


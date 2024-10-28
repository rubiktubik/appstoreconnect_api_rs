# \UserInvitationsApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_invitations_create_instance**](UserInvitationsApi.md#user_invitations_create_instance) | **POST** /v1/userInvitations | 
[**user_invitations_delete_instance**](UserInvitationsApi.md#user_invitations_delete_instance) | **DELETE** /v1/userInvitations/{id} | 
[**user_invitations_get_collection**](UserInvitationsApi.md#user_invitations_get_collection) | **GET** /v1/userInvitations | 
[**user_invitations_get_instance**](UserInvitationsApi.md#user_invitations_get_instance) | **GET** /v1/userInvitations/{id} | 
[**user_invitations_visible_apps_get_to_many_related**](UserInvitationsApi.md#user_invitations_visible_apps_get_to_many_related) | **GET** /v1/userInvitations/{id}/visibleApps | 



## user_invitations_create_instance

> models::UserInvitationResponse user_invitations_create_instance(user_invitation_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_invitation_create_request** | [**UserInvitationCreateRequest**](UserInvitationCreateRequest.md) | UserInvitation representation | [required] |

### Return type

[**models::UserInvitationResponse**](UserInvitationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_invitations_delete_instance

> user_invitations_delete_instance(id)


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


## user_invitations_get_collection

> models::UserInvitationsResponse user_invitations_get_collection(filter_left_square_bracket_email_right_square_bracket, filter_left_square_bracket_roles_right_square_bracket, filter_left_square_bracket_visible_apps_right_square_bracket, sort, fields_left_square_bracket_user_invitations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, limit, include, limit_left_square_bracket_visible_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_email_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'email' |  |
**filter_left_square_bracket_roles_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by attribute 'roles' |  |
**filter_left_square_bracket_visible_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | filter by id(s) of related 'visibleApps' |  |
**sort** | Option<[**Vec<String>**](String.md)> | comma-separated list of sort expressions; resources will be sorted as specified |  |
**fields_left_square_bracket_user_invitations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type userInvitations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**limit** | Option<**i32**> | maximum resources per page |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_visible_apps_right_square_bracket** | Option<**i32**> | maximum number of related visibleApps returned (when they are included) |  |

### Return type

[**models::UserInvitationsResponse**](UserInvitationsResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_invitations_get_instance

> models::UserInvitationResponse user_invitations_get_instance(id, fields_left_square_bracket_user_invitations_right_square_bracket, fields_left_square_bracket_apps_right_square_bracket, include, limit_left_square_bracket_visible_apps_right_square_bracket)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**fields_left_square_bracket_user_invitations_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type userInvitations |  |
**fields_left_square_bracket_apps_right_square_bracket** | Option<[**Vec<String>**](String.md)> | the fields to include for returned resources of type apps |  |
**include** | Option<[**Vec<String>**](String.md)> | comma-separated list of relationships to include |  |
**limit_left_square_bracket_visible_apps_right_square_bracket** | Option<**i32**> | maximum number of related visibleApps returned (when they are included) |  |

### Return type

[**models::UserInvitationResponse**](UserInvitationResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_invitations_visible_apps_get_to_many_related

> models::AppsWithoutIncludesResponse user_invitations_visible_apps_get_to_many_related(id, fields_left_square_bracket_apps_right_square_bracket, limit)


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


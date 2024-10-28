# \AppStoreVersionPhasedReleasesApi

All URIs are relative to *https://api.appstoreconnect.apple.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_store_version_phased_releases_create_instance**](AppStoreVersionPhasedReleasesApi.md#app_store_version_phased_releases_create_instance) | **POST** /v1/appStoreVersionPhasedReleases | 
[**app_store_version_phased_releases_delete_instance**](AppStoreVersionPhasedReleasesApi.md#app_store_version_phased_releases_delete_instance) | **DELETE** /v1/appStoreVersionPhasedReleases/{id} | 
[**app_store_version_phased_releases_update_instance**](AppStoreVersionPhasedReleasesApi.md#app_store_version_phased_releases_update_instance) | **PATCH** /v1/appStoreVersionPhasedReleases/{id} | 



## app_store_version_phased_releases_create_instance

> models::AppStoreVersionPhasedReleaseResponse app_store_version_phased_releases_create_instance(app_store_version_phased_release_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_store_version_phased_release_create_request** | [**AppStoreVersionPhasedReleaseCreateRequest**](AppStoreVersionPhasedReleaseCreateRequest.md) | AppStoreVersionPhasedRelease representation | [required] |

### Return type

[**models::AppStoreVersionPhasedReleaseResponse**](AppStoreVersionPhasedReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_store_version_phased_releases_delete_instance

> app_store_version_phased_releases_delete_instance(id)


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


## app_store_version_phased_releases_update_instance

> models::AppStoreVersionPhasedReleaseResponse app_store_version_phased_releases_update_instance(id, app_store_version_phased_release_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the id of the requested resource | [required] |
**app_store_version_phased_release_update_request** | [**AppStoreVersionPhasedReleaseUpdateRequest**](AppStoreVersionPhasedReleaseUpdateRequest.md) | AppStoreVersionPhasedRelease representation | [required] |

### Return type

[**models::AppStoreVersionPhasedReleaseResponse**](AppStoreVersionPhasedReleaseResponse.md)

### Authorization

[itc-bearer-token](../README.md#itc-bearer-token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


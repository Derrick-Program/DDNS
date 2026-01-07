# \DefaultApi

All URIs are relative to *https://ddns.duacodie.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_login**](DefaultApi.md#auth_login) | **POST** /auth/login | 
[**auth_logout**](DefaultApi.md#auth_logout) | **POST** /auth/logout | 
[**auth_refresh**](DefaultApi.md#auth_refresh) | **POST** /auth/refresh | 
[**records_list**](DefaultApi.md#records_list) | **GET** /records | 
[**updates_update**](DefaultApi.md#updates_update) | **POST** /records/{recordId}:update | 



## auth_login

> models::AuthLogin200Response auth_login(login_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |

### Return type

[**models::AuthLogin200Response**](Auth_login_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_logout

> models::ErrorResponse auth_logout(logout_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logout_request** | [**LogoutRequest**](LogoutRequest.md) |  | [required] |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_refresh

> models::AuthLogin200Response auth_refresh(refresh_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_request** | [**RefreshRequest**](RefreshRequest.md) |  | [required] |

### Return type

[**models::AuthLogin200Response**](Auth_login_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_list

> models::RecordsList200Response records_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RecordsList200Response**](Records_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updates_update

> models::UpdatesUpdate200Response updates_update(record_id, update_record_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**record_id** | **String** |  | [required] |
**update_record_request** | [**UpdateRecordRequest**](UpdateRecordRequest.md) |  | [required] |

### Return type

[**models::UpdatesUpdate200Response**](Updates_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


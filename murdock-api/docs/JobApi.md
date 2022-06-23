# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_handler_job_uid_get**](JobApi.md#job_handler_job_uid_get) | **GET** /job/{uid} | Return the details of a job
[**job_remove_handler_job_uid_delete**](JobApi.md#job_remove_handler_job_uid_delete) | **DELETE** /job/{uid} | Remove a job
[**job_restart_handler_job_uid_post**](JobApi.md#job_restart_handler_job_uid_post) | **POST** /job/{uid} | Restart a finished job
[**running_job_status_handler_job_uid_status_put**](JobApi.md#running_job_status_handler_job_uid_status_put) | **PUT** /job/{uid}/status | Update the status of a running job



## job_handler_job_uid_get

> crate::models::JobModel job_handler_job_uid_get(uid)
Return the details of a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_remove_handler_job_uid_delete

> crate::models::JobModel job_remove_handler_job_uid_delete(uid)
Remove a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_restart_handler_job_uid_post

> crate::models::JobModel job_restart_handler_job_uid_post(uid)
Restart a finished job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## running_job_status_handler_job_uid_status_put

> crate::models::JobModel running_job_status_handler_job_uid_status_put(uid)
Update the status of a running job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


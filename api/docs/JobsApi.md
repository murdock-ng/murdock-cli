# \JobsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**finished_job_delete_handler_jobs_delete**](JobsApi.md#finished_job_delete_handler_jobs_delete) | **DELETE** /jobs | Removed finished jobs older than 'before' date
[**jobs_handler_jobs_get**](JobsApi.md#jobs_handler_jobs_get) | **GET** /jobs | Return the list of all jobs



## finished_job_delete_handler_jobs_delete

> Vec<crate::models::JobModel> finished_job_delete_handler_jobs_delete(before)
Removed finished jobs older than 'before' date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | **String** |  | [required] |

### Return type

[**Vec<crate::models::JobModel>**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_handler_jobs_get

> Vec<crate::models::JobModel> jobs_handler_jobs_get(limit, uid, is_pr, is_branch, is_tag, states, prnum, prstates, branch, tag, _ref, sha, tree, author, after, before)
Return the list of all jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 25]
**uid** | Option<**String**> |  |  |
**is_pr** | Option<**bool**> |  |  |
**is_branch** | Option<**bool**> |  |  |
**is_tag** | Option<**bool**> |  |  |
**states** | Option<**String**> |  |  |
**prnum** | Option<**i32**> |  |  |
**prstates** | Option<**String**> |  |  |
**branch** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**_ref** | Option<**String**> |  |  |
**sha** | Option<**String**> |  |  |
**tree** | Option<**String**> |  |  |
**author** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**before** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::JobModel>**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


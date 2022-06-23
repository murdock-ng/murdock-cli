# \CommitJobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_get_last_commit_handler_job_commit_sha_get**](CommitJobApi.md#job_get_last_commit_handler_job_commit_sha_get) | **GET** /job/commit/{sha} | Return the last job of the given commit
[**job_start_commit_handler_job_commit_post**](CommitJobApi.md#job_start_commit_handler_job_commit_post) | **POST** /job/commit | Start a manual job on a tag



## job_get_last_commit_handler_job_commit_sha_get

> crate::models::JobModel job_get_last_commit_handler_job_commit_sha_get(sha)
Return the last job of the given commit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sha** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_start_commit_handler_job_commit_post

> crate::models::JobModel job_start_commit_handler_job_commit_post(manual_job_commit_param_model)
Start a manual job on a tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_job_commit_param_model** | [**ManualJobCommitParamModel**](ManualJobCommitParamModel.md) |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


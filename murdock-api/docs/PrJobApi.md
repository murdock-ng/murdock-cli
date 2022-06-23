# \PrJobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_get_last_prnum_handler_job_pr_prnum_get**](PrJobApi.md#job_get_last_prnum_handler_job_pr_prnum_get) | **GET** /job/pr/{prnum} | Return the last job of the given PR number



## job_get_last_prnum_handler_job_pr_prnum_get

> crate::models::JobModel job_get_last_prnum_handler_job_pr_prnum_get(prnum)
Return the last job of the given PR number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prnum** | **i32** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \BranchJobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_get_last_branch_badge_handler_job_branch_branch_badge_get**](BranchJobApi.md#job_get_last_branch_badge_handler_job_branch_branch_badge_get) | **GET** /job/branch/{branch}/badge | Return the last job badge of the given branch
[**job_get_last_branch_handler_job_branch_branch_get**](BranchJobApi.md#job_get_last_branch_handler_job_branch_branch_get) | **GET** /job/branch/{branch} | Return the last job of the given branch
[**job_start_branch_handler_job_branch_post**](BranchJobApi.md#job_start_branch_handler_job_branch_post) | **POST** /job/branch | Start a manual job on a branch



## job_get_last_branch_badge_handler_job_branch_branch_badge_get

> job_get_last_branch_badge_handler_job_branch_branch_badge_get(branch)
Return the last job badge of the given branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_get_last_branch_handler_job_branch_branch_get

> crate::models::JobModel job_get_last_branch_handler_job_branch_branch_get(branch)
Return the last job of the given branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_start_branch_handler_job_branch_post

> crate::models::JobModel job_start_branch_handler_job_branch_post(manual_job_branch_param_model)
Start a manual job on a branch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_job_branch_param_model** | [**ManualJobBranchParamModel**](ManualJobBranchParamModel.md) |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


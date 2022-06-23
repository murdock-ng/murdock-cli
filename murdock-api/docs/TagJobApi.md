# \TagJobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_get_last_tag_handler_job_tag_tag_get**](TagJobApi.md#job_get_last_tag_handler_job_tag_tag_get) | **GET** /job/tag/{tag} | Return the last job of the given tag
[**job_start_tag_handler_job_tag_post**](TagJobApi.md#job_start_tag_handler_job_tag_post) | **POST** /job/tag | Start a manual job on a tag



## job_get_last_tag_handler_job_tag_tag_get

> crate::models::JobModel job_get_last_tag_handler_job_tag_tag_get(tag)
Return the last job of the given tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_start_tag_handler_job_tag_post

> crate::models::JobModel job_start_tag_handler_job_tag_post(manual_job_tag_param_model)
Start a manual job on a tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_job_tag_param_model** | [**ManualJobTagParamModel**](ManualJobTagParamModel.md) |  | [required] |

### Return type

[**crate::models::JobModel**](JobModel.md)

### Authorization

[Github OAuth Token](../README.md#Github OAuth Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


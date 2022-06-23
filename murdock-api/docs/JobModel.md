# JobModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> |  | [optional]
**commit** | Option<[**crate::models::InformationOfTheCommitToProcess**](Information_of_the_commit_to_process.md)> |  | [optional]
**_ref** | Option<**String**> |  | [optional]
**prinfo** | Option<[**crate::models::PullRequestDetailedInformationIfAny**](Pull_Request_detailed_information__if_any_.md)> |  | [optional]
**creation_time** | Option<**f32**> |  | [optional]
**start_time** | Option<**f32**> |  | [optional]
**fasttracked** | Option<**bool**> |  | [optional]
**status** | Option<[**serde_json::Value**](.md)> |  | [optional]
**state** | Option<**String**> |  | [optional]
**output** | Option<**String**> |  | [optional]
**output_text_url** | Option<**String**> |  | [optional]
**runtime** | Option<**f32**> |  | [optional]
**trigger** | Option<**String**> |  | [optional][default to api]
**triggered_by** | Option<**String**> |  | [optional]
**env** | Option<[**serde_json::Value**](.md)> |  | [optional]
**user_env** | Option<[**serde_json::Value**](.md)> |  | [optional]
**artifacts** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



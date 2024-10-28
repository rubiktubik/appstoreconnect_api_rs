# CiWorkflowUpdateRequestDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**branch_start_condition** | Option<[**models::CiBranchStartCondition**](CiBranchStartCondition.md)> |  | [optional]
**tag_start_condition** | Option<[**models::CiTagStartCondition**](CiTagStartCondition.md)> |  | [optional]
**pull_request_start_condition** | Option<[**models::CiPullRequestStartCondition**](CiPullRequestStartCondition.md)> |  | [optional]
**scheduled_start_condition** | Option<[**models::CiScheduledStartCondition**](CiScheduledStartCondition.md)> |  | [optional]
**manual_branch_start_condition** | Option<[**models::CiManualBranchStartCondition**](CiManualBranchStartCondition.md)> |  | [optional]
**manual_tag_start_condition** | Option<[**models::CiManualTagStartCondition**](CiManualTagStartCondition.md)> |  | [optional]
**manual_pull_request_start_condition** | Option<[**models::CiManualPullRequestStartCondition**](CiManualPullRequestStartCondition.md)> |  | [optional]
**actions** | Option<[**Vec<models::CiAction>**](CiAction.md)> |  | [optional]
**is_enabled** | Option<**bool**> |  | [optional]
**is_locked_for_editing** | Option<**bool**> |  | [optional]
**clean** | Option<**bool**> |  | [optional]
**container_file_path** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



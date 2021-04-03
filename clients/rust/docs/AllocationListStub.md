# AllocationListStub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ID** | Option<**String**> |  | [optional]
**eval_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**namespace** | Option<**String**> |  | [optional]
**node_id** | Option<**String**> |  | [optional]
**node_name** | Option<**String**> |  | [optional]
**job_id** | Option<**String**> |  | [optional]
**job_type** | Option<**String**> |  | [optional]
**job_version** | Option<**i32**> |  | [optional]
**task_group** | Option<**String**> |  | [optional]
**desired_status** | Option<**String**> |  | [optional]
**desired_description** | Option<**String**> |  | [optional]
**client_status** | Option<**String**> |  | [optional]
**client_description** | Option<**String**> |  | [optional]
**task_states** | Option<[**::std::collections::HashMap<String, crate::models::TaskState>**](TaskState.md)> |  | [optional]
**deployment_status** | Option<[**crate::models::AllocDeploymentStatus**](AllocDeploymentStatus.md)> |  | [optional]
**followup_eval_id** | Option<**String**> |  | [optional]
**reschedule_tracker** | Option<[**crate::models::RescheduleTracker**](RescheduleTracker.md)> |  | [optional]
**preempted_allocations** | Option<**Vec<String>**> |  | [optional]
**preempted_by_allocation** | Option<**String**> |  | [optional]
**create_index** | Option<**i32**> |  | [optional]
**modify_index** | Option<**i32**> |  | [optional]
**create_time** | Option<**i64**> |  | [optional]
**modify_time** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



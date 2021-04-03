# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stop** | Option<**bool**> |  | [optional]
**region** | Option<**String**> |  | [optional]
**namespace** | Option<**String**> |  | [optional]
**ID** | Option<**String**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**priority** | Option<**i32**> |  | [optional]
**all_at_once** | Option<**bool**> |  | [optional]
**datacenters** | Option<**Vec<String>**> |  | [optional]
**constraints** | Option<[**Vec<crate::models::Constraint>**](Constraint.md)> |  | [optional]
**affinities** | Option<[**Vec<crate::models::Affinity>**](Affinity.md)> |  | [optional]
**task_groups** | Option<[**Vec<crate::models::TaskGroup>**](TaskGroup.md)> |  | [optional]
**update** | Option<[**crate::models::UpdateStrategy**](UpdateStrategy.md)> |  | [optional]
**spreads** | Option<[**Vec<crate::models::Spread>**](Spread.md)> |  | [optional]
**periodic** | Option<[**crate::models::PeriodicConfig**](PeriodicConfig.md)> |  | [optional]
**parameterized_job** | Option<[**crate::models::ParameterizedJobConfig**](ParameterizedJobConfig.md)> |  | [optional]
**dispatched** | Option<**bool**> |  | [optional]
**payload** | Option<**String**> |  | [optional]
**reschedule** | Option<[**crate::models::ReschedulePolicy**](ReschedulePolicy.md)> |  | [optional]
**migrate** | Option<[**crate::models::MigrateStrategy**](MigrateStrategy.md)> |  | [optional]
**meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**consul_token** | Option<**String**> |  | [optional]
**vault_token** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**status_description** | Option<**String**> |  | [optional]
**stable** | Option<**bool**> |  | [optional]
**version** | Option<**i32**> |  | [optional]
**submit_time** | Option<**i64**> |  | [optional]
**create_index** | Option<**i32**> |  | [optional]
**modify_index** | Option<**i32**> |  | [optional]
**job_modify_index** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



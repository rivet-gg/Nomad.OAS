# TaskGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**count** | Option<**i32**> |  | [optional]
**constraints** | Option<[**Vec<crate::models::Constraint>**](Constraint.md)> |  | [optional]
**affinities** | Option<[**Vec<crate::models::Affinity>**](Affinity.md)> |  | [optional]
**tasks** | Option<[**Vec<crate::models::Task>**](Task.md)> |  | [optional]
**spreads** | Option<[**Vec<crate::models::Spread>**](Spread.md)> |  | [optional]
**volumes** | Option<[**::std::collections::HashMap<String, crate::models::VolumeRequest>**](VolumeRequest.md)> |  | [optional]
**restart_policy** | Option<[**crate::models::RestartPolicy**](RestartPolicy.md)> |  | [optional]
**reschedule_policy** | Option<[**crate::models::ReschedulePolicy**](ReschedulePolicy.md)> |  | [optional]
**ephemeral_disk** | Option<[**crate::models::EphemeralDisk**](EphemeralDisk.md)> |  | [optional]
**update** | Option<[**crate::models::UpdateStrategy**](UpdateStrategy.md)> |  | [optional]
**migrate** | Option<[**crate::models::MigrateStrategy**](MigrateStrategy.md)> |  | [optional]
**networks** | Option<[**Vec<crate::models::NetworkResource>**](NetworkResource.md)> |  | [optional]
**meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**services** | Option<[**Vec<crate::models::Service>**](Service.md)> |  | [optional]
**shutdown_delay** | Option<**i64**> |  | [optional]
**scaling** | Option<[**crate::models::ScalingPolicy**](ScalingPolicy.md)> |  | [optional]
**stop_after_client_disconnect** | Option<**i64**> |  | [optional]
**max_client_disconnect** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



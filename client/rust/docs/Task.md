# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**driver** | Option<**String**> |  | [optional]
**user** | Option<**String**> |  | [optional]
**lifecycle** | Option<[**crate::models::TaskLifecycle**](TaskLifecycle.md)> |  | [optional]
**config** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**constraints** | Option<[**Vec<crate::models::Constraint>**](Constraint.md)> |  | [optional]
**affinities** | Option<[**Vec<crate::models::Affinity>**](Affinity.md)> |  | [optional]
**env** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**services** | Option<[**Vec<crate::models::Service>**](Service.md)> |  | [optional]
**resources** | Option<[**crate::models::Resources**](Resources.md)> |  | [optional]
**meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**kill_timeout** | Option<**i64**> |  | [optional]
**log_config** | Option<[**crate::models::LogConfig**](LogConfig.md)> |  | [optional]
**artifacts** | Option<[**Vec<crate::models::TaskArtifact>**](TaskArtifact.md)> |  | [optional]
**vault** | Option<[**crate::models::Vault**](Vault.md)> |  | [optional]
**templates** | Option<[**Vec<crate::models::Template>**](Template.md)> |  | [optional]
**dispatch_payload** | Option<[**crate::models::DispatchPayloadConfig**](DispatchPayloadConfig.md)> |  | [optional]
**volume_mounts** | Option<[**Vec<crate::models::VolumeMount>**](VolumeMount.md)> |  | [optional]
**csi_plugin** | Option<[**crate::models::TaskCsiPluginConfig**](TaskCsiPluginConfig.md)> |  | [optional]
**leader** | Option<**bool**> |  | [optional]
**shutdown_delay** | Option<**i64**> |  | [optional]
**kill_signal** | Option<**String**> |  | [optional]
**kind** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



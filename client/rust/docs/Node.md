# Node

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ID** | Option<**String**> |  | [optional]
**datacenter** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**http_addr** | Option<**String**> |  | [optional]
**tls_enabled** | Option<**bool**> |  | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**resources** | Option<[**crate::models::Resources**](Resources.md)> |  | [optional]
**reserved** | Option<[**crate::models::Resources**](Resources.md)> |  | [optional]
**node_resources** | Option<[**crate::models::NodeResources**](NodeResources.md)> |  | [optional]
**reserved_resources** | Option<[**crate::models::NodeReservedResources**](NodeReservedResources.md)> |  | [optional]
**links** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**node_class** | Option<**String**> |  | [optional]
**drain** | Option<**bool**> |  | [optional]
**drain_strategy** | Option<[**crate::models::DrainStrategy**](DrainStrategy.md)> |  | [optional]
**scheduling_eligibility** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**status_description** | Option<**String**> |  | [optional]
**status_updated_at** | Option<**i64**> |  | [optional]
**events** | Option<[**Vec<crate::models::NodeEvent>**](NodeEvent.md)> |  | [optional]
**drivers** | Option<[**::std::collections::HashMap<String, crate::models::DriverInfo>**](DriverInfo.md)> |  | [optional]
**host_volumes** | Option<[**::std::collections::HashMap<String, crate::models::HostVolumeInfo>**](HostVolumeInfo.md)> |  | [optional]
**csi_controller_plugins** | Option<[**::std::collections::HashMap<String, crate::models::CsiInfo>**](CsiInfo.md)> |  | [optional]
**csi_node_plugins** | Option<[**::std::collections::HashMap<String, crate::models::CsiInfo>**](CsiInfo.md)> |  | [optional]
**create_index** | Option<**i32**> |  | [optional]
**modify_index** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



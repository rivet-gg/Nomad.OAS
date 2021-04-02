# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ID** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**canary_tags** | Option<**Vec<String>**> |  | [optional]
**enable_tag_override** | Option<**bool**> |  | [optional]
**port_label** | Option<**String**> |  | [optional]
**address_mode** | Option<**String**> |  | [optional]
**checks** | Option<[**Vec<crate::models::ServiceCheck>**](ServiceCheck.md)> |  | [optional]
**check_restart** | Option<[**crate::models::CheckRestart**](CheckRestart.md)> |  | [optional]
**connect** | Option<[**crate::models::ConsulConnect**](ConsulConnect.md)> |  | [optional]
**meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**canary_meta** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



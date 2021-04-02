# AllocationMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nodes_evaluated** | Option<**i32**> |  | [optional]
**nodes_filtered** | Option<**i32**> |  | [optional]
**nodes_available** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]
**class_filtered** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]
**constraint_filtered** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]
**nodes_exhausted** | Option<**i32**> |  | [optional]
**class_exhausted** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]
**dimension_exhausted** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]
**quota_exhausted** | Option<**Vec<String>**> |  | [optional]
**scores** | Option<**::std::collections::HashMap<String, f64>**> |  | [optional]
**allocation_time** | Option<**i64**> |  | [optional]
**coalesced_failures** | Option<**i32**> |  | [optional]
**score_meta_data** | Option<[**Vec<crate::models::NodeScoreMeta>**](NodeScoreMeta.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



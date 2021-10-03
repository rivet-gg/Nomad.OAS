# \VolumesApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deregister_volume**](VolumesApi.md#deregister_volume) | **delete** /volume/csi/{volume_id} | deregisters an external volume with Nomad. It is an error to deregister a volume that is in use
[**get_volume**](VolumesApi.md#get_volume) | **get** /volume/csi/{volume_id} | reads information about a specific volume
[**get_volumes**](VolumesApi.md#get_volumes) | **get** /volumes | lists all volumes
[**register_volume**](VolumesApi.md#register_volume) | **put** /volume/csi/{volume_id} | registers an external volume with Nomad. It is an error to register an existing volume



## deregister_volume

> deregister_volume(volume_id, namespace, region, index, wait)
deregisters an external volume with Nomad. It is an error to deregister a volume that is in use

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Specifies the ID of the volume. This must be the full ID. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> crate::models::CsiVolume get_volume(volume_id, namespace, region, index, wait)
reads information about a specific volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Specifies the ID of the volume. This must be the full ID. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::CsiVolume**](CsiVolume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volumes

> Vec<crate::models::CsiVolumeListStub> get_volumes(namespace, region, index, wait, _type, node_id, plugin_id)
lists all volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**_type** | Option<**String**> | Specifies the type of volume to query. Currently only supports csi. This is specified as a query string parameter. Returns an empty list if omitted |  |
**node_id** | Option<**String**> | node id |  |
**plugin_id** | Option<**String**> | Specifies a string to filter volumes based on a plugin ID prefix |  |

### Return type

[**Vec<crate::models::CsiVolumeListStub>**](CsiVolumeListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_volume

> register_volume(volume_id, namespace, region, index, wait, csi_volume_register_request)
registers an external volume with Nomad. It is an error to register an existing volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Specifies the ID of the volume. This must be the full ID. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**csi_volume_register_request** | Option<[**CsiVolumeRegisterRequest**](CsiVolumeRegisterRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


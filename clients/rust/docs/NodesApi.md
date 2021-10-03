# \NodesApi

All URIs are relative to *http://localhost:4646/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**evaluate_node**](NodesApi.md#evaluate_node) | **post** /node/{node_id}/evaluate | creates a new evaluation for the given node. This can be used to force a run of the scheduling logic
[**get_allocations_for_node**](NodesApi.md#get_allocations_for_node) | **get** /node/{node_id}/allocations | lists all of the allocations for the given node
[**get_node**](NodesApi.md#get_node) | **get** /node/{node_id} | queries the status of a client node
[**get_nodes**](NodesApi.md#get_nodes) | **get** /nodes | lists all nodes registered with Nomad
[**search**](NodesApi.md#search) | **post** /search | returns matches for a given prefix and context, where a context can be jobs, allocations, evaluations, nodes, or deployments
[**update_drain_mode_for_node**](NodesApi.md#update_drain_mode_for_node) | **post** /node/{node_id}/drain | toggles the drain mode of the node
[**update_node_eligibility**](NodesApi.md#update_node_eligibility) | **post** /node/{node_id}/eligibility | toggles the scheduling eligibility of the node



## evaluate_node

> crate::models::NodeEvalResponse evaluate_node(node_id, namespace, region, index, wait)
creates a new evaluation for the given node. This can be used to force a run of the scheduling logic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Specifies the ID of the node. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::NodeEvalResponse**](NodeEvalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allocations_for_node

> Vec<crate::models::AllocationListStub> get_allocations_for_node(node_id, namespace, region, index, wait)
lists all of the allocations for the given node

This can be used to determine what allocations have been scheduled on the node, their current status, and the values of dynamically assigned resources, like ports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Specifies the ID of the node. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**Vec<crate::models::AllocationListStub>**](AllocationListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node

> crate::models::Node get_node(node_id, namespace, region, index, wait)
queries the status of a client node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Specifies the ID of the node. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |

### Return type

[**crate::models::Node**](Node.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nodes

> Vec<crate::models::NodeListStub> get_nodes(namespace, region, index, wait, prefix)
lists all nodes registered with Nomad

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**prefix** | Option<**String**> | Specifies a string to filter jobs on based on an index prefix. This is specified as a query string parameter |  |

### Return type

[**Vec<crate::models::NodeListStub>**](NodeListStub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> crate::models::SearchResponse search(namespace, region, index, wait, search_request)
returns matches for a given prefix and context, where a context can be jobs, allocations, evaluations, nodes, or deployments

When using Nomad Enterprise, the allowed contexts include quotas and namespaces. Additionally, a prefix can be searched for within every context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**search_request** | Option<[**SearchRequest**](SearchRequest.md)> |  |  |

### Return type

[**crate::models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_drain_mode_for_node

> crate::models::NodeDrainUpdateResponse update_drain_mode_for_node(node_id, namespace, region, index, wait, node_update_drain_request)
toggles the drain mode of the node

When draining is enabled, no further allocations will be assigned to this node, and existing allocations will be migrated to new nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Specifies the ID of the node. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**node_update_drain_request** | Option<[**NodeUpdateDrainRequest**](NodeUpdateDrainRequest.md)> |  |  |

### Return type

[**crate::models::NodeDrainUpdateResponse**](NodeDrainUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_node_eligibility

> crate::models::NodeEligibilityUpdateResponse update_node_eligibility(node_id, namespace, region, index, wait, node_update_eligibility_request)
toggles the scheduling eligibility of the node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Specifies the ID of the node. This must be the full UUID, not the short 8-character one. This is specified as part of the path | [required] |
**namespace** | Option<**String**> |  |  |
**region** | Option<**String**> | Make a request across regions to the given region |  |
**index** | Option<**i64**> | index used for blocking requests |  |
**wait** | Option<**String**> | wait time used for blocking requests |  |
**node_update_eligibility_request** | Option<[**NodeUpdateEligibilityRequest**](NodeUpdateEligibilityRequest.md)> |  |  |

### Return type

[**crate::models::NodeEligibilityUpdateResponse**](NodeEligibilityUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


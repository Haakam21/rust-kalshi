# BatchCancelOrdersIndividualResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<[**models::JsonError**](JSONError.md)> |  | [optional]
**order** | Option<[**models::Order**](Order.md)> |  | [optional]
**order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional order_id to identify the orders that errored. | [optional]
**reduced_by** | **i32** | ReducedBy is how much the count of the order was reduced by because of this operation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



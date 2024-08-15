# EventData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | Deprecated: Event category. Use the series level property instead. | 
**event_ticker** | **String** | Unique identifier for events. | 
**markets** | Option<[**Vec<models::Market>**](Market.md)> | The markets that are linked to this event. Will be filled only if the query parameter \"with_nested_markets\" is equal \"true\". | [optional]
**mutually_exclusive** | **bool** | If true then the event is mutually exclusive. | 
**series_ticker** | **String** | Unique identifier for series. | 
**strike_date** | Option<**String**> | Date and time in the ISO 8601 spec. Example: 2022-11-30T15:00:00Z | [optional]
**strike_period** | Option<**String**> | The strike period for this event. This will be filled when the event strike is not a date. If it is a date then the 'strike_date' field should be filled instead. | [optional]
**sub_title** | **String** | Shortened title. | 
**title** | **String** | Event title. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



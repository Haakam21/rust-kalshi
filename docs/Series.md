# Series

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | Category specifies the category which this series belongs to. | 
**contract_url** | **String** | ContractUrl provides a direct link to contract terms which govern the series. | 
**frequency** | **String** | Description of the frequency of the series. There is no fixed value set here, but will be something human-readable like: weekly, daily, one-off. | 
**settlement_sources** | [**Vec<models::SettlementSource>**](SettlementSource.md) | SettlementSources specifies the official sources used for the determination of markets within the series. Methodology is defined in the rulebook. | 
**tags** | **Vec<String>** | Tags specifies the subjects that this series relates to, multiple series from different categories can have the same tags. | 
**ticker** | **String** | Ticker that identifies this series. | 
**title** | **String** | Title describing the series. For full context use you should use this field with the title field of the events belonging to this series. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



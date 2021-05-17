# TransactionType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scenario** | [**crate::models::TransactionScenario**](TransactionScenario.md) |  | 
**sub_scenario** | Option<**String**> | Possible sub-scenario, defined locally within the scheme (UndefinedEnum Type). | [optional]
**initiator** | [**crate::models::TransactionInitiator**](TransactionInitiator.md) |  | 
**initiator_type** | [**crate::models::TransactionInitiatorType**](TransactionInitiatorType.md) |  | 
**refund_info** | Option<[**crate::models::Refund**](Refund.md)> |  | [optional]
**balance_of_payments** | Option<**String**> | (BopCode) The API data type BopCode is a JSON String of 3 characters, consisting of digits only. Negative numbers are not allowed. A leading zero is not allowed. https://www.imf.org/external/np/sta/bopcode/ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



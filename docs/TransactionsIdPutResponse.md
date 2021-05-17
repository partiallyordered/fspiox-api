# TransactionsIdPutResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_timestamp** | Option<**String**> | The API data type DateTime is a JSON String in a lexical format that is restricted by a regular expression for interoperability reasons. The format is according to ISO 8601, expressed in a combined date, time and time zone format. A more readable version of the format is yyyy-MM-ddTHH:mm:ss.SSS[-HH:MM]. Examples -  \"2016-05-24T08:38:08.699-04:00\", \"2016-05-24T08:38:08.699Z\" (where Z indicates Zulu time zone, same as UTC). | [optional]
**transaction_state** | [**crate::models::TransactionState**](TransactionState.md) |  | 
**code** | Option<**String**> | Any code/token returned by the Payee FSP (TokenCode Type). | [optional]
**extension_list** | Option<[**crate::models::ExtensionList**](ExtensionList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



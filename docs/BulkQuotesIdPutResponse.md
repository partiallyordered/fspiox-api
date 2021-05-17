# BulkQuotesIdPutResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**individual_quote_results** | [**Vec<::models::IndividualQuoteResult>**](IndividualQuoteResult.md) | Fees for each individual transaction, if any of them are charged per transaction. | [optional] [default to null]
**expiration** | **String** | Date and time until when the quotation is valid and can be honored when used in the subsequent transaction request. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



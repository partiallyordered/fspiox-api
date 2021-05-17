# IndividualTransferResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies messages related to the same /transfers sequence. | [default to null]
**fulfilment** | [***::models::IlpFulfilment**](IlpFulfilment.md) | Fulfilment of the condition specified with the transaction. Note - Either fulfilment or errorInformation should be set, not both. | [optional] [default to null]
**error_information** | [***::models::ErrorInformation**](ErrorInformation.md) | If transfer is REJECTED, error information may be provided. Note - Either fulfilment or errorInformation should be set, not both. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



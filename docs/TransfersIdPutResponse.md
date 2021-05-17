# TransfersIdPutResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fulfilment** | [***::models::IlpFulfilment**](IlpFulfilment.md) | Fulfilment of the condition specified with the transaction. Mandatory if transfer has completed successfully. | [optional] [default to null]
**completed_timestamp** | **String** | Time and date when the transaction was completed. | [optional] [default to null]
**transfer_state** | [***::models::TransferState**](TransferState.md) | State of the transfer. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



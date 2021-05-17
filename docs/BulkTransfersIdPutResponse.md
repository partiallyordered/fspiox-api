# BulkTransfersIdPutResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_timestamp** | **String** | Time and date when the bulk transaction was completed. | [optional] [default to null]
**individual_transfer_results** | [**Vec<::models::IndividualTransferResult>**](IndividualTransferResult.md) | List of IndividualTransferResult elements. | [optional] [default to null]
**bulk_transfer_state** | [***::models::BulkTransferState**](BulkTransferState.md) | The state of the bulk transfer. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



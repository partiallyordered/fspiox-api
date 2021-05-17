# BulkTransfersPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bulk_transfer_id** | [***::models::CorrelationId**](CorrelationId.md) | Common ID between the FSPs and the optional Switch for the bulk transfer object, decided by the Payer FSP. The ID should be reused for resends of the same bulk transfer. A new ID should be generated for each new bulk transfer. | [default to null]
**bulk_quote_id** | [***::models::CorrelationId**](CorrelationId.md) | ID of the related bulk quote. | [default to null]
**payer_fsp** | [***::models::FspId**](FspId.md) | Payer FSP identifier. | [default to null]
**payee_fsp** | [***::models::FspId**](FspId.md) | Payee FSP identifier. | [default to null]
**individual_transfers** | [**Vec<::models::IndividualTransfer>**](IndividualTransfer.md) | List of IndividualTransfer elements. | [default to null]
**expiration** | **String** | Expiration time of the transfers. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



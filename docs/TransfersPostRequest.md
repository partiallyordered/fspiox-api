# TransfersPostRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | [***::models::CorrelationId**](CorrelationId.md) | The common ID between the FSPs and the optional Switch for the transfer object, decided by the Payer FSP. The ID should be reused for resends of the same transfer. A new ID should be generated for each new transfer. | [default to null]
**payee_fsp** | [***::models::FspId**](FspId.md) | Payee FSP in the proposed financial transaction. | [default to null]
**payer_fsp** | [***::models::FspId**](FspId.md) | Payer FSP in the proposed financial transaction. | [default to null]
**amount** | [***::models::Money**](Money.md) | The transfer amount to be sent. | [default to null]
**ilp_packet** | [***::models::IlpPacket**](IlpPacket.md) | The ILP Packet containing the amount delivered to the Payee and the ILP Address of the Payee and any other end-to-end data. | [default to null]
**condition** | [***::models::IlpCondition**](IlpCondition.md) | The condition that must be fulfilled to commit the transfer. | [default to null]
**expiration** | **String** | Expiration can be set to get a quick failure expiration of the transfer. The transfer should be rolled back if no fulfilment is delivered before this time. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



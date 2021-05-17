# IndividualTransfer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies messages related to the same /transfers sequence. | [default to null]
**transfer_amount** | [***::models::Money**](Money.md) | Transaction amount to be sent. | [default to null]
**ilp_packet** | [***::models::IlpPacket**](IlpPacket.md) | ILP Packet containing the amount delivered to the Payee and the ILP Address of the Payee and any other end-to-end data. | [default to null]
**condition** | [***::models::IlpCondition**](IlpCondition.md) | Condition that must be fulfilled to commit the transfer. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



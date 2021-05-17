# IndividualQuoteResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quote_id** | [***::models::CorrelationId**](CorrelationId.md) | Identifies quote message. | [default to null]
**payee** | [***::models::Party**](Party.md) | Information about the Payee in the proposed financial transaction. | [optional] [default to null]
**transfer_amount** | [***::models::Money**](Money.md) | The amount of Money that the Payer FSP should transfer to the Payee FSP. | [optional] [default to null]
**payee_receive_amount** | [***::models::Money**](Money.md) | Amount that the Payee should receive in the end-to-end transaction. Optional as the Payee FSP might not want to disclose any optional Payee fees. | [optional] [default to null]
**payee_fsp_fee** | [***::models::Money**](Money.md) | Payee FSP’s part of the transaction fee. | [optional] [default to null]
**payee_fsp_commission** | [***::models::Money**](Money.md) | Transaction commission from the Payee FSP | [optional] [default to null]
**ilp_packet** | [***::models::IlpPacket**](IlpPacket.md) | The ILP Packet that must be attached to the transfer by the Payer. | [optional] [default to null]
**condition** | [***::models::IlpCondition**](IlpCondition.md) | The condition that must be attached to the transfer by the Payer. | [optional] [default to null]
**error_information** | [***::models::ErrorInformation**](ErrorInformation.md) | Error code, category description. Note - payee, transferAmount, payeeReceiveAmount, payeeFspFee, payeeFspCommission, ilpPacket, and condition should not be set if errorInformation is set. | [optional] [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



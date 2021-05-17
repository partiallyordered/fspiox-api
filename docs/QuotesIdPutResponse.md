# QuotesIdPutResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_amount** | [***::models::Money**](Money.md) | The amount of Money that the Payer FSP should transfer to the Payee FSP. | [default to null]
**payee_receive_amount** | [***::models::Money**](Money.md) | The amount of Money that the Payee should receive in the end-to-end transaction. Optional as the Payee FSP might not want to disclose any optional Payee fees. | [optional] [default to null]
**payee_fsp_fee** | [***::models::Money**](Money.md) | Payee FSP’s part of the transaction fee. | [optional] [default to null]
**payee_fsp_commission** | [***::models::Money**](Money.md) | Transaction commission from the Payee FSP. | [optional] [default to null]
**expiration** | **String** | Date and time until when the quotation is valid and can be honored when used in the subsequent transaction. | [default to null]
**geo_code** | [***::models::GeoCode**](GeoCode.md) | Longitude and Latitude of the Payee. Can be used to detect fraud. | [optional] [default to null]
**ilp_packet** | [***::models::IlpPacket**](IlpPacket.md) | The ILP Packet that must be attached to the transfer by the Payer. | [default to null]
**condition** | [***::models::IlpCondition**](IlpCondition.md) | The condition that must be attached to the transfer by the Payer. | [default to null]
**extension_list** | [***::models::ExtensionList**](ExtensionList.md) | Optional extension, specific to deployment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



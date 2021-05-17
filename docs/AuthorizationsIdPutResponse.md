# AuthorizationsIdPutResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_info** | [***::models::AuthenticationInfo**](AuthenticationInfo.md) | OTP or QR Code if entered, otherwise empty. | [optional] [default to null]
**response_type** | [***::models::AuthorizationResponse**](AuthorizationResponse.md) | Enum containing response information; if the customer entered the authentication value, rejected the transaction, or requested a resend of the authentication value. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



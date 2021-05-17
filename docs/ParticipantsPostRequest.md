# ParticipantsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘). | 
**party_list** | [**Vec<crate::models::PartyIdInfo>**](PartyIdInfo.md) | List of PartyIdInfo elements that the client would like to update or create FSP information about. | 
**currency** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



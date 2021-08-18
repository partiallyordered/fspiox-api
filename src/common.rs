// Definitions in this file should map directly to definitions in api.yaml
// TODO: can this whole module be replaced by something that comes out of swagger generator?
// TODO: implement validation
//       https://blog.logrocket.com/json-input-validation-in-rust-web-services/
//       or try swagger generator?

use strum_macros::EnumString;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
pub use rust_decimal::Decimal;
use derive_more::{FromStr, Display, Constructor};
use serde;
use std::fmt;

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// ^([0]|([1-9][0-9]{0,17}))([.][0-9]{0,3}[1-9])?$
// TODO: validation
// TODO: rusty_money?
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display, PartialOrd, Ord)]
pub struct Amount(Decimal);

impl Amount {
    pub fn abs(&self) -> Amount {
        Amount(self.0.abs())
    }
    pub const ZERO: Amount = Amount(Decimal::ZERO);
}

#[cfg(feature = "typescript_types")]
impl TS for Amount {
    fn name() -> String {
        "Amount".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type Amount = string".to_string()
    }
}

// ^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$
// TODO: newtype
// #[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display)]
pub struct CorrelationId(pub Uuid);

#[cfg(feature = "typescript_types")]
impl TS for CorrelationId {
    fn name() -> String {
        "CorrelationId".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type CorrelationId = string".to_string()
    }
}

#[cfg(feature = "typescript_types")]
impl TS for DateTime {
    fn name() -> String {
        "DateTime".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type DateTime = string".to_string()
    }
}

impl CorrelationId {
    pub fn new() -> CorrelationId {
        return CorrelationId(Uuid::new_v4());
    }
}

// ^.{1,32}$
// TODO: validation. In fact, in addition to the spec, which specifies the regex ^.{1,32}$, FspId
//       only accepts alphanumeric characters. (At least, this is what central ledger tells us when
//       we attempt to create a participant). Additionally, central ledger will only accept 30
//       characters for FSP ID. Basically, either central ledger is wrong, or the spec is wrong.
// TODO: newtype
// TODO: how much space can a utf-8 codepoint take? If it's finite, we could use
//          mut [u8, 32 * 2]
//       as the type for FspId
pub type FspId = String;

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display, EnumString)]
pub enum Currency {
    AED,
    AFA,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    AOR,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BYR,
    BZD,
    CAD,
    CDF,
    CHE,
    CHF,
    CHW,
    CLF,
    CLP,
    CNY,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EEK,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GGP,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    IMP,
    INR,
    IQD,
    IRR,
    ISK,
    JEP,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    LVL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SPL,
    SRD,
    SSP,
    STD,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TVD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    USN,
    UYI,
    UYU,
    UZS,
    VEF,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XCD,
    XDR,
    XFO,
    XFU,
    XOF,
    XPD,
    XPF,
    XPT,
    XSU,
    XTS,
    XUA,
    XXX,
    YER,
    ZAR,
    ZMK,
    ZMW,
    ZWD,
    ZWL,
    ZWN,
    ZWR,
}

// ^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(\.\d{3}))(?:Z|[+-][01]\d:[0-5]\d)$
#[derive(Constructor, Debug, Clone, Copy)]
pub struct DateTime(pub chrono::DateTime<chrono::Utc>);

const DATETIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format(DATETIME_FORMAT))
    }
}

impl std::str::FromStr for DateTime {
    type Err = chrono::format::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chrono::{Utc, TimeZone};
        Utc.datetime_from_str(&s, DATETIME_FORMAT).map(DateTime)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}", self.0.format(DATETIME_FORMAT));
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<DateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use chrono::{Utc, TimeZone};
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, DATETIME_FORMAT).map_err(serde::de::Error::custom).map(DateTime)
    }
}

// TODO: rusty_money? re-export?
// TODO: "positive money". I.e. a type that will fail to deserialize a value < 0.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Money {
    pub currency: Currency,
    pub amount: Amount,
}

/** See section 7.6 of "API Definition v1.0.docx". Note that some of the these
 * error objects contain an httpStatusCode property that indicates the HTTP
 * response code for cases where errors are returned immediately i.e. upon
 * request, rather than on callback.  Those error objects that do not contain
 * an httpStatusCode property are expected to only be returned to callers in
 * error callbacks after the initial request was accepted with a 202/200.
 */
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MojaloopApiError {
    // Generic communication errors
    #[serde(rename = "1000")]
    CommunicationError,            // { code: "1000", message: "Communication error" },
    #[serde(rename = "1001")]
    DestinationCommunicationError, // { code: "1001", message: "Destination communication error" },

    // Generic server errors
    #[serde(rename = "2000")]
    ServerError,                   // { code: "2000", message: "Generic server error" },
    #[serde(rename = "2001")]
    InternalServerError,           // { code: "2001", message: "Internal server error" },
    #[serde(rename = "2002")]
    NotImplemented,                // { code: "2002", message: "Not implemented" , httpStatusCode: 501},
    #[serde(rename = "2003")]
    ServiceCurrentlyUnavailable,   // { code: "2003", message: "Service currently unavailable", httpStatusCode: 503 },
    #[serde(rename = "2004")]
    ServerTimedOut,                // { code: "2004", message: "Server timed out" },
    #[serde(rename = "2005")]
    ServerBusy,                    // { code: "2005", message: "Server busy" },

    // Generic client errors
    // TODO:
    // 1. check the spec to determine whether these two codes are indeed both 3000
    // 2. check the spec to determine whether they're actually intended to just be the same
    //    "generic client error" and the writer of sdk-standard-components either made a mistake or
    //    took some artistic license.
    // 3. if they are not the same, or not intended to be the same, fix this
    // 4. if they are the same, and intended to be the same code, but with different
    //    interpretations, this may require some sort of custom fix-up after deserialisation to
    //    determine the correct code based on the error message, or the HTTP response code (or we
    //    could ignore it for the sake of simplicity, and just call the API bad). Or we could
    //    ignore this- the http response code could be considered part of the error response, and
    //    therefore included in that struct. Moreover, the error description should help us
    //    delineate the correct action to take where necesary.
    #[serde(rename = "3000")]
    ClientError,                   // { code: "3000", message: "Generic client error", httpStatusCode: 400 },
    #[serde(rename = "3000")]
    MethodNotAllowed,              // { code: "3000", message: "Generic client error - Method Not Allowed", httpStatusCode: 405 },
    #[serde(rename = "3001")]
    UnacceptableVersion,           // { code: "3001", message: "Unacceptable version requested", httpStatusCode: 406 },
    #[serde(rename = "3002")]
    UnknownUri,                    // { code: "3002", message: "Unknown URI", httpStatusCode: 404 },
    #[serde(rename = "3003")]
    AddPartyInfoError,             // { code: "3003", message: "Add Party information error" },
    #[serde(rename = "3040")]
    DeletePartyInfoError,          // { code: "3040", message: "Delete Party information error" }, // Error code thrown in ALS when deleting participant info fails

    // Client validation errors
    #[serde(rename = "3100")]
    ValidationError,               // { code: "3100", message: "Generic validation error", httpStatusCode: 400 },
    #[serde(rename = "3101")]
    MalformedSyntax,               // { code: "3101", message: "Malformed syntax", httpStatusCode: 400 },
    #[serde(rename = "3102")]
    MissingElement,                // { code: "3102", message: "Missing mandatory element", httpStatusCode: 400 },
    #[serde(rename = "3103")]
    TooManyElements,               // { code: "3103", message: "Too many elements", httpStatusCode: 400 },
    #[serde(rename = "3104")]
    TooLargePayload,               // { code: "3104", message: "Too large payload", httpStatusCode: 400 },
    #[serde(rename = "3105")]
    InvalidSignature,              // { code: "3105", message: "Invalid signature", httpStatusCode: 400 },
    #[serde(rename = "3106")]
    ModifiedRequest,               // { code: "3106", message: "Modified request", httpStatusCode: 400 },
    #[serde(rename = "3107")]
    MissingMandatoryExtension,     // { code: "3107", message: "Missing mandatory extension parameter", httpStatusCode: 400 },

    // Identifier errors
    #[serde(rename = "3200")]
    IdNotFound,                    // { code: "3200", message: "Generic ID not found" },
    #[serde(rename = "3201")]
    DestinationFspError,           // { code: "3201", message: "Destination FSP Error" },
    #[serde(rename = "3202")]
    PayerFspIdNotFound,            // { code: "3202", message: "Payer FSP ID not found" },
    #[serde(rename = "3203")]
    PayeeFspIdNotFound,            // { code: "3203", message: "Payee FSP ID not found" },
    #[serde(rename = "3204")]
    PartyNotFound,                 // { code: "3204", message: "Party not found" },
    #[serde(rename = "3205")]
    QuoteIdNotFound,               // { code: "3205", message: "Quote ID not found" },
    #[serde(rename = "3206")]
    TxnRequestIdNotFound,          // { code: "3206", message: "Transaction request ID not found" },
    #[serde(rename = "3207")]
    TxnIdNotFound,                 // { code: "3207", message: "Transaction ID not found" },
    #[serde(rename = "3208")]
    TransferIdNotFound,            // { code: "3208", message: "Transfer ID not found" },
    #[serde(rename = "3209")]
    BulkQuoteIdNotFound,           // { code: "3209", message: "Bulk quote ID not found" },
    #[serde(rename = "3210")]
    BulkTransferIdNotFound,        // { code: "3210", message: "Bulk transfer ID not found" },

    // Expired errors
    #[serde(rename = "3300")]
    ExpiredError,                  // { code: "3300", message: "Generic expired error" },
    #[serde(rename = "3301")]
    TxnRequestExpired,             // { code: "3301", message: "Transaction request expired" },
    #[serde(rename = "3302")]
    QuoteExpired,                  // { code: "3302", message: "Quote expired" },
    #[serde(rename = "3303")]
    TransferExpired,               // { code: "3303", message: "Transfer expired" },

    // Payer errors
    #[serde(rename = "4000")]
    PayerError,                    // { code: "4000", message: "Generic Payer error" },
    #[serde(rename = "4001")]
    PayerFspInsufficientLiquidity, // { code: "4001", message: "Payer FSP insufficient liquidity" },
    #[serde(rename = "4100")]
    PayerRejection,                // { code: "4100", message: "Generic Payer rejection" },
    #[serde(rename = "4101")]
    PayerRejectedTxnRequest,       // { code: "4101", message: "Payer rejected transaction request" },
    #[serde(rename = "4102")]
    PayerFspUnsupportedTxnType,    // { code: "4102", message: "Payer FSP unsupported transaction type" },
    #[serde(rename = "4103")]
    PayerUnsupportedCurrency,      // { code: "4103", message: "Payer unsupported currency" },
    #[serde(rename = "4200")]
    PayerLimitError,               // { code: "4200", message: "Payer limit error" },
    #[serde(rename = "4300")]
    PayerPermissionError,          // { code: "4300", message: "Payer permission error" },
    #[serde(rename = "4400")]
    PayerBlockedError,             // { code: "4400", message: "Generic Payer blocked error" },

    // Payee errors
    #[serde(rename = "5000")]
    PayeeError,                    // { code: "5000", message: "Generic Payee error" },
    #[serde(rename = "5001")]
    PayeeFspInsufficientLiquidity, // { code: "5001", message: "Payee FSP insufficient liquidity" },
    #[serde(rename = "5100")]
    PayeeRejection,                // { code: "5100", message: "Generic Payee rejection" },
    #[serde(rename = "5101")]
    PayeeRejectedQuote,            // { code: "5101", message: "Payee rejected quote" },
    #[serde(rename = "5102")]
    PayeeFspUnsupportedTxnType,    // { code: "5102", message: "Payee FSP unsupported transaction type" },
    #[serde(rename = "5103")]
    PayeeFspRejectedQuote,         // { code: "5103", message: "Payee FSP rejected quote" },
    #[serde(rename = "5104")]
    PayeeRejectedTxn,              // { code: "5104", message: "Payee rejected transaction" },
    #[serde(rename = "5105")]
    PayeeFspRejectedTxn,           // { code: "5105", message: "Payee FSP rejected transaction" },
    #[serde(rename = "5106")]
    PayeeUnsupportedCurrency,      // { code: "5106", message: "Payee unsupported currency" },
    #[serde(rename = "5200")]
    PayeeLimitError,               // { code: "5200", message: "Payee limit error" },
    #[serde(rename = "5300")]
    PayeePermissionError,          // { code: "5300", message: "Payee permission error" },
    #[serde(rename = "5400")]
    GenericPayeeBlockedError,      // { code: "5400", message: "Generic Payee blocked error" }
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInformation {
    // TODO:
    pub error_code: MojaloopApiError,
    pub error_description: String,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_information: ErrorInformation,
}

// TODO: we're going to have Serialize implemented on all types in this API, so write a macro to
// use that information to implement Display using serde::Serialize. And possibly Debug, also?
// Probably not Debug- that would probably violate the user's expectations.
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string::<ErrorResponse>(self).unwrap())
    }
}

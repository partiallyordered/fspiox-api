// Definitions in this file should map directly to definitions in api.yaml
// TODO: can this whole module be replaced by something that comes out of swagger generator?
// TODO: implement validation
//       https://blog.logrocket.com/json-input-validation-in-rust-web-services/
//       or try swagger generator?

use serde::{Serialize, Deserialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

// ^([0]|([1-9][0-9]{0,17}))([.][0-9]{0,3}[1-9])?$
// TODO: validation
pub type Amount = String;

// ^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$
pub type CorrelationId = Uuid;

// ^.{1,32}$
// TODO: validation
pub type FspId = String;

#[derive(Deserialize, Serialize, Debug, ToSql)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
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
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
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
    UYU,
    UZS,
    VEF,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XDR,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMW,
    ZWD,
}

// ^[A-Za-z0-9-_]{43}$
// TODO: validation
pub type IlpCondition = String;

// ^[A-Za-z0-9-_]+[=]{0,2}$
// minLength: 1, maxLength: 32768
// TODO: validation
pub type IlpPacket = String;

// ^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(\.\d{3}))(?:Z|[+-][01]\d:[0-5]\d)$
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Money {
    pub currency: Currency,
    pub amount: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransfersPostRequest {
    pub transfer_id: CorrelationId,
    pub payee_fsp: FspId,
    pub payer_fsp: FspId,
    pub amount: Money,
    pub ilp_packet: IlpPacket,
    pub condition: IlpCondition,
    pub expiration: DateTime,
    // TODO: handle extensionList
}

    // pattern: ^[A-Za-z0-9-_]{43}$
    // but..
    // maxLength: 48
    // TODO: validation
    pub type IlpFulfilment = String;

    #[derive(Serialize, Deserialize, Debug, ToSql)]
    pub enum TransferState {
        RECEIVED,
        RESERVED,
        COMMITTED,
        ABORTED,
    }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferFulfilRequest {
    pub fulfilment: IlpFulfilment,
    pub completed_timestamp: DateTime,
    pub transfer_state: TransferState,
    // TODO: handle extensionList
}

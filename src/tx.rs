use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Clone, Copy)]
pub struct Tx {
    #[serde(rename = "type")]
    pub ty: TxType,

    #[serde(rename = "client")]
    pub client: Client,

    #[serde(rename = "tx")]
    pub id: TxId,

}

#[derive(Deserialize, Clone, Copy)]
pub enum TxType {
    #[serde(rename = "deposit")]
    Deposit,

    #[serde(rename = "withdrawal")]
    Withdrawal,

    #[serde(rename = "dispute")]
    Dispute,

    #[serde(rename = "resolve")]
    Resolve,

    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Deserialize,Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Client(u16);

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxId(u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
pub struct Amount(i32);
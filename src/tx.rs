use serde::{Deserialize, Deserializer, Serialize, Serializer};

const AMOUNT_SHIFT_ACCURACY: f32 = 10000.0;

#[derive(Deserialize, Clone, Copy)]
pub struct Tx {
    #[serde(rename = "type")]
    pub ty: TxType,

    #[serde(rename = "client")]
    pub client: Client,

    #[serde(rename = "tx")]
    pub id: TxId,

    #[serde(rename = "amount", deserialize_with = "Amount::deserialize", default)]
    pub amount: Amount,
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

impl From<u16> for Client {
    fn from(id: u16) -> Self {
        Client(id)
    }
}

impl Into<u16> for Client {
    fn into(self) -> u16 {
        self.0
    }
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxId(u32);

impl From<u32>  for TxId {
    fn from(id: u32) -> Self {
        TxId(id)
    }
}

impl Into<u32> for TxId {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
pub struct Amount(i32);
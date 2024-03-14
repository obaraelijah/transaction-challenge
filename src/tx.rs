use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Deserialize, Clone, Copy)]
pub struct Tx {
    pub ty: TxType,
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
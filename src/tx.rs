use serde::{Deserialize, Deserializer, Serialize, Serializer};


pub struct Tx {

}

pub enum TxType {
    Deposit,
    Withdrawal,
    Resolve,
    Chargeback,
}
use super::tx::{Amount, Client, Tx, TxId, TxType};
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
struct Account {
    client: Client,
    available: Amount,
    a_held: Amount,
    total: Amount,
    locked: bool,
}

struct Engine {
    accounts,
    tx_amounts,
    disputed,
}

impl Engine {
    
}
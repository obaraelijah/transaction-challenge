use super::tx::{Amount, Client, Tx, TxId, TxType};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use anyhow::{anyhow, Ok, Result};

#[derive(Clone, Copy, Serialize)]
pub struct Account {
    client: Client,
    #[serde(serialize_with = "Amount::serialize")]
    available: Amount,
    #[serde(serialize_with = "Amount::serialize")]
    held: Amount,
    #[serde(serialize_with = "Amount::serialize")]
    total: Amount,
    locked: bool,
}

pub struct Engine {
    accounts: BTreeMap<Client, Account>,
    tx_amounts: BTreeMap<TxId, Amount>,
    disputed: BTreeSet<TxId>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            accounts: BTreeMap::new(),
            tx_amounts: BTreeMap::new(),
            disputed: BTreeSet::new(),
        }
    }

    pub fn apply(&mut self, tx: Tx) -> Result<()> {
        if matches!(tx.ty,  TxType::Deposit | TxType::Withdrawal) {
            self.tx_amounts.insert(tx.id, tx.amount);
        }

        match tx.ty {
            TxType::Deposit => todo!(),
            TxType::Withdrawal => todo!(),
            TxType::Dispute => todo!(),
            TxType::Chargeback => todo!(),
            TxType::Resolve => todo!(),
        }
    }

    fn account(&mut self, client: Client) -> Result<&mut Account> {
        let account = self.accounts.entry(client).or_insert(Account {
            client,
            available: Amount::from(0.0),
            held: Amount::from(0.0),
            total: Amount::from(0.0),
            locked: false,
        });

        if account.locked {
            return Err(anyhow!("account is locked"));
        }

        Ok(account)
    }

    fn deposit(&mut self, client: Client, amount: Amount, _tx_id: TxId) -> Result<()> {
        let account = self.account(client)?;

        account.available = account.available + amount;
        account.total = account.total + amount;
        Ok(())
    }

    fn withdraw(&mut self, client: Client, amount: Amount, tx_id: TxId) -> Result<()> {
        let account = self.account(client)?;

        if account.available < amount {
            return Err(anyhow!(
                "insufficient funds for withdrawal, tx: {:?}",
                tx_id
            ));
        }

        account.available = account.available - amount;
        account.total = account.total - amount;
        Ok(())
    }
}
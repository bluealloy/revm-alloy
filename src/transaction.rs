use alloy_consensus::{transaction::Recovered, Transaction as AlloyTransaction};
use derive_more::{AsMut, AsRef};
use revm_context_interface::{
    result::InvalidTransaction, transaction::AuthorizationItem, Transaction as RevmTransaction,
};
use revm_primitives::{Address, Bytes, TxKind, B256, U256};

#[derive(Clone, Debug, PartialEq, Eq, Hash, AsMut, AsRef)]
pub struct RevmAlloyTransaction<TX: AlloyTransaction>(Recovered<TX>);

impl<TX: AlloyTransaction> From<Recovered<TX>> for RevmAlloyTransaction<TX> {
    fn from(tx: Recovered<TX>) -> Self {
        RevmAlloyTransaction(tx)
    }
}

impl<TX> RevmTransaction for RevmAlloyTransaction<TX>
where
    TX: AlloyTransaction,
{
    type TransactionError = InvalidTransaction;

    fn tx_type(&self) -> u8 {
        self.0.tx().ty()
    }

    fn caller(&self) -> Address {
        self.0.signer()
    }

    fn gas_limit(&self) -> u64 {
        self.0.tx().gas_limit()
    }

    fn value(&self) -> U256 {
        self.0.tx().value()
    }

    fn input(&self) -> &Bytes {
        self.0.tx().input()
    }

    fn nonce(&self) -> u64 {
        self.0.tx().nonce()
    }

    fn kind(&self) -> TxKind {
        self.0.tx().kind()
    }

    fn chain_id(&self) -> Option<u64> {
        self.0.tx().chain_id()
    }

    fn gas_price(&self) -> u128 {
        self.0.tx().gas_price().unwrap_or(self.0.max_fee_per_gas())
    }

    fn access_list(&self) -> Option<impl Iterator<Item = (&Address, &[B256])>> {
        self.0.tx().access_list().map(|list| {
            list.iter()
                .map(|item| (&item.address, item.storage_keys.as_slice()))
        })
    }

    fn blob_versioned_hashes(&self) -> &[B256] {
        self.0.tx().blob_versioned_hashes().unwrap_or_default()
    }

    fn max_fee_per_blob_gas(&self) -> u128 {
        self.0.tx().max_fee_per_gas()
    }

    fn authorization_list_len(&self) -> usize {
        self.0
            .tx()
            .authorization_list()
            .map(|l| l.len())
            .unwrap_or_default()
    }

    fn authorization_list(&self) -> impl Iterator<Item = AuthorizationItem> {
        self.0
            .tx()
            .authorization_list()
            .unwrap_or(&[])
            .iter()
            .map(|a| {
                (
                    a.recover_authority().ok(),
                    U256::from(a.chain_id),
                    a.nonce,
                    a.address,
                )
            })
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.0.tx().max_priority_fee_per_gas()
    }
}

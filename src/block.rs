use alloy_consensus::BlockHeader as AlloyBlock;
use derive_more::{AsMut, AsRef};
use revm_context_interface::{block::BlobExcessGasAndPrice, Block as RevmBlock};
use revm_primitives::{Address, B256, U256};

#[derive(Clone, Debug, PartialEq, Eq, Hash, AsMut, AsRef)]
pub struct RevmAlloyBlock<B: AlloyBlock>(B);

impl<B: AlloyBlock> From<B> for RevmAlloyBlock<B> {
    fn from(block: B) -> Self {
        RevmAlloyBlock(block)
    }
}

impl<B> RevmBlock for RevmAlloyBlock<B>
where
    B: AlloyBlock,
{
    fn number(&self) -> u64 {
        self.0.number()
    }

    fn beneficiary(&self) -> Address {
        self.0.beneficiary()
    }

    fn timestamp(&self) -> u64 {
        self.0.timestamp()
    }

    #[doc = " The gas limit of the block."]
    fn gas_limit(&self) -> u64 {
        self.0.gas_limit()
    }

    fn basefee(&self) -> u64 {
        self.0.base_fee_per_gas().unwrap_or(u64::MAX)
    }

    fn difficulty(&self) -> U256 {
        self.0.difficulty()
    }

    fn prevrandao(&self) -> Option<B256> {
        self.0.parent_beacon_block_root()
    }

    fn blob_excess_gas_and_price(&self) -> Option<BlobExcessGasAndPrice> {
        self.0.excess_blob_gas().map(BlobExcessGasAndPrice::new)
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

mod eth_signer;

#[openbrush::contract(env = PinkEnvironment)]
mod evm_chain {
    use crate::eth_signer::EthSigner;
    use ink_lang as ink;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use pink_extension::PinkEnvironment;
    use registry_traits::{
        AssetInfo, AssetsRegisry, BalanceFetcher, ChainType, Error as RegistryError, Inspector,
        SignedTransaction,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct EvmChain {
        admin: AccountId,
        /// The chain name
        chain: Vec<u8>,
        /// Type of chain
        chain_type: ChainType,
        /// The registered assets list
        assets: Vec<AssetInfo>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct NativeSet {
        #[ink(topic)]
        chain: Vec<u8>,
        #[ink(topic)]
        asset: Option<AssetInfo>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct StableSet {
        #[ink(topic)]
        chain: Vec<u8>,
        #[ink(topic)]
        asset: Option<AssetInfo>,
    }

    pub type Result<T> = core::result::Result<T, RegistryError>;

    impl EvmChain {
        #[ink(constructor)]
        /// Create an Ethereum entity
        pub fn new(chain: Vec<u8>) -> Self {
            ink::utils::initialize_contract(|this: &mut Self| {
                this.admin = Self::env().caller();
                this.chain = chain;
                this.chain_type = ChainType::EVM;
            })
        }

        /// Set native asset
        /// Authorized method, only the contract owner can do
        #[ink(message)]
        pub fn set_native(&mut self, asset: AssetInfo) -> Result<()> {
            Self::env().emit_event(NativeSet {
                chain: self.chain,
                asset: Some(asset),
            });
            Ok(())
        }

        /// Set native asset
        /// Authorized method, only the contract owner can do
        #[ink(message)]
        pub fn set_stable(&mut self, asset: AssetInfo) -> Result<()> {
            Self::env().emit_event(StableSet {
                chain: self.chain,
                asset: Some(asset),
            });
            Ok(())
        }
    }

    /// Same as Signer trait
    #[ink::trait_definition]
    pub trait EthTx {
        /// Sign a transaction
        #[ink(message)]
        fn sign_transaction(&self, signer: EthSigner, unsigned_tx: Vec<u8>) -> SignedTransaction;
    }

    impl EthTx for EvmChain {
        #[ink(message)]
        fn sign_transaction(&self, signer: EthSigner, unsigned_tx: Vec<u8>) -> SignedTransaction {
            // TODO: sign with signer
            SignedTransaction::EthSignedTransaction
        }
    }

    // impl Inspector for EvmChain {

    // }

    // impl BalanceFetcher for EvmChain {

    // }

    // impl AssetsRegisry<(), Error> for EvmChain {

    // }
}

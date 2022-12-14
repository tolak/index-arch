#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use ink_lang as ink;
use ink_storage::traits::{
    PackedAllocate, PackedLayout, SpreadAllocate, SpreadLayout, StorageLayout,
};
use scale::{Decode, Encode};
use xcm::latest::{AssetId, MultiLocation};

/// Errors that can occur upon registry module.
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    BadOrigin,
    AssetAlreadyRegistered,
    AssetNotFound,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SignedTransaction {
    /// Ethereum signed transaction
    EthSignedTransaction,
    /// Substrate-based chain signed transaction
    SubSignedTransaction,
}

pub trait Signer {
    /// Sign a transaction
    fn sign_transaction(&self, unsigned_tx: Vec<u8>) -> SignedTransaction;
}

/// Query the account balance of an asset under a multichain scenario is a mess,
/// not only because different chains have different account systems but also have
/// different asset registry mechanism(e.g. Acala use Currency, Phala use pallet-assets
/// manage registered foreign assets). Besides, query the native asset and foreign assets
/// on a chain also different
///
/// Use `AssetId` and `MultiLocation` to represent indentification of the `asset` and `account` respectively
/// is a good choice because developers can customize the way how they represent the `asset`
/// `account`. For example, for `USDC` on Ethereum, bridge1 can represent it with
/// `MultiLocation::new(1, X2(GeneralKey('Ethereum'), GeneralKey(usdc_addr))`, bridge2 can represent
/// it with `MultiLocation::new(1, X3(Parachain(2004), GeneralIndex(0), GeneralKey(usdc_addr))`.
///
/// Both `AssetId` and `MultiLocation` are primitives introduced by XCM format.
#[ink::trait_definition]
pub trait BalanceFetcher {
    /// Return on-chain `asset` amount of `account`
    #[ink(message)]
    fn balance_of(&self, asset: AssetId, account: MultiLocation) -> u128;
}

/// Beyond general properties like `name`, `symbol` and `decimals`,
/// a `location` is needed to identify the asset between multi-chains
#[derive(
    Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, SpreadAllocate,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout,))]
pub struct AssetInfo {
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
    /// Encoded asset MultiLocation
    location: Vec<u8>,
}

#[ink::trait_definition]
pub trait AssetsRegisry {
    /// Register the asset
    /// Authorized method, only the contract owner can do
    #[ink(message)]
    fn register(&mut self, asset: AssetInfo) -> core::result::Result<(), Error>;

    /// Unregister the asset
    /// Authorized method, only the contract owner can do
    #[ink(message)]
    fn unregister(&mut self, asset: AssetInfo) -> core::result::Result<(), Error>;

    /// Return all registerd assets
    #[ink(message)]
    fn registered_assets(&self, name: Vec<u8>) -> Vec<AssetInfo>;

    #[ink(message)]
    fn lookup_by_name(&self, name: Vec<u8>) -> Option<AssetInfo>;

    #[ink(message)]
    fn lookup_by_symbol(&self, name: Vec<u8>) -> Option<AssetInfo>;

    #[ink(message)]
    fn lookup_by_location(&self, name: Vec<u8>) -> Option<AssetInfo>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout,))]
pub enum ChainType {
    Evm,
    Sub,
}

#[ink::trait_definition]
pub trait Inspector {
    /// Return set native asset of the chain
    #[ink(message)]
    fn native_asset(&self) -> Option<AssetInfo>;

    /// Return set stable asset of the chain
    #[ink(message)]
    fn stable_asset(&self) -> Option<AssetInfo>;
}

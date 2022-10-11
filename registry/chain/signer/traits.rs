pub enum SignedTransaction {
	/// Ethereum signed transaction

	/// Substrate-based chain signed transaction
}

#[ink::trait_definition]
pub trait Signer {
	/// Sign a transaction
	#[ink(message)]
	fn signTransaction(unsigned_tx: Vec<u8>) -> SignedTransaction
}


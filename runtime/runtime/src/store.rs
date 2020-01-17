use near_crypto::PublicKey;
use near_primitives::account::Account;
use near_primitives::hash::CryptoHash;
use near_primitives::serialize::option_base64_format;
use near_primitives::types::AccountId;
use near_primitives::views::{AccessKeyView, ReceiptView};

/// Record in the state storage.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StateRecord {
    /// Account information.
    Account { account_id: AccountId, account: Account },
    /// Data records inside the contract, encoded in base64.
    Data { key: String, value: String },
    /// Contract code encoded in base64.
    Contract { account_id: AccountId, code: String },
    /// Access key associated with some account.
    AccessKey { account_id: AccountId, public_key: PublicKey, access_key: AccessKeyView },
    /// Postponed Action Receipt.
    PostponedReceipt(Box<ReceiptView>),
    /// Received data from DataReceipt encoded in base64 for the given account_id and data_id.
    ReceivedData {
        account_id: AccountId,
        data_id: CryptoHash,
        #[serde(with = "option_base64_format")]
        data: Option<Vec<u8>>,
    },
}

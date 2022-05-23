pub mod types;

use std::str::FromStr;

use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use anchor_client::solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use anchor_syn::hash;
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::account::ReadableAccount;
use solana_sdk::bs58;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signature::Signer,
};

use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster, EventContext};

use crate::types::UserMetadata;

fn main() {
    println!("Starting test...");

    let conn = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        // "https://solana-api.projectserum.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    println!("Connected to mainnet, slot={}", conn.get_slot().unwrap());

    let hubble_program_key =
        Pubkey::from_str("HubbLeXBb7qyLHt3x7gvYaRrxQmmgExb7fCJgDqFuB6T").unwrap();

    let user_metadata_discriminator = get_discriminator_bytes("UserMetadata");

    let account_type_filter = RpcFilterType::Memcmp(Memcmp {
        offset: 0,
        bytes: MemcmpEncodedBytes::Base58(bs58::encode(user_metadata_discriminator).into_string()),
        encoding: None,
    });

    let suspects = [
        ""
// pubkey addresses"
    ];

    for suspect in suspects.iter() {
        let owner_type_filter = RpcFilterType::Memcmp(Memcmp {
            offset: 50,
            bytes: MemcmpEncodedBytes::Base58(suspect.to_string()),
            encoding: None,
        });

        let config = RpcProgramAccountsConfig {
            filters: Some(vec![account_type_filter.clone(), owner_type_filter]),
            account_config: RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                data_slice: None,
                commitment: None,
            },
            with_context: None,
        };

        let accounts = conn
            .get_program_accounts_with_config(&hubble_program_key, config)
            .unwrap();

        println!("accounts, {:?} ", accounts);
    }
}

pub fn get_discriminator_bytes(account_name: &str) -> [u8; 8] {
    let discriminator_preimage = format!("account:{}", account_name);
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8]);
    discriminator
}

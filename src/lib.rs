//! JSONRPC interface using bitcoincore-rpc's RpcApi trait to access bitcoin RPC methods.

/// how to use:
/// ```
/// use bitcoin_rpc_api::new_client;
/// let mut io = jsonrpc_core::IoHandler::default();
/// io.extend_with(
/// 	BitcoinApi::to_delegate(new_client(
/// 		std::env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set"),
/// 		std::env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set"),
/// 		std::env::var("BITCOIN_RPC_PASS").expect("BITCOIN_RPC_PASS must be set")
/// 	))
/// );
/// ```
use bitcoincore_rpc::{bitcoin::BlockHash, Auth};
use jsonrpc_core::{Error as RpcError, Error, ErrorCode as RpcErrorCode};
use jsonrpc_derive::rpc;

use crate::bitcoin::hashes::_export::_core::str::FromStr;
use crate::bitcoin::{Block, BlockHeader};

pub use bitcoincore_rpc::{Client as BitcoinClient, RpcApi as BitcoinApi};
pub use bitcoincore_rpc_json::*;

pub struct BitcoinConfig<C>(C);

impl<C> BitcoinConfig<C> {
    fn into_inner(self) -> C {
        self.0
    }

    fn new(inner: C) -> BitcoinConfig<C> {
        BitcoinConfig(inner)
    }
}

fn to_rpc_error(e: bitcoincore_rpc::Error) -> RpcError {
    RpcError {
        code: RpcErrorCode::InternalError, //TODO: should it?
        message: e.to_string(),
        data: None,
    }
}

fn to_block_hash(block_hash: String) -> Result<BlockHash, RpcError> {
    BlockHash::from_str(&block_hash).map_err(|e| RpcError {
        code: RpcErrorCode::ParseError,
        message: format!(
            "cannot convert hash {} to BlockHash structure; {:?}",
            block_hash, e
        ),
        data: None,
    })
}

/// generate bitcoincore_rpc client
pub fn new_client(btc_url: String, user: String, pass: String) -> BitcoinConfig<BitcoinClient> {
    let auth = Auth::UserPass(user, pass);
    let client = BitcoinClient::new(btc_url, auth).unwrap();
    BitcoinConfig(client)
}

/// this api is jsonrpc-core and jsonrpc-derive compatible.
#[rpc]
pub trait BitcoinJsonRPCApi {
    /// category wallet
    #[rpc(name = "getwalletinfo")]
    fn get_wallet_info(&self) -> Result<GetWalletInfoResult, RpcError>;

    #[rpc(name = "listwallets")]
    fn list_wallets(&self) -> Result<Vec<String>, RpcError>;

    /// category control
    #[rpc(name = "uptime")]
    fn uptime(&self) -> Result<u64, RpcError>;

    /// category network
    #[rpc(name = "getnetworkinfo")]
    fn get_network_info(&self) -> Result<GetNetworkInfoResult, RpcError>;

    #[rpc(name = "getconnectioncount")]
    fn get_connection_count(&self) -> Result<usize, RpcError>;

    #[rpc(name = "getpeerinfo")]
    fn get_peer_info(&self) -> Result<Vec<GetPeerInfoResult>, RpcError>;

    #[rpc(name = "ping")]
    fn ping(&self) -> Result<(), RpcError>;

    #[rpc(name = "getnettotals")]
    fn get_net_totals(&self) -> Result<GetNetTotalsResult, RpcError>;

    /// category blockchain
    #[rpc(name = "getblock")]
    fn get_block(&self, block_hash: String) -> Result<Block, RpcError>;

    #[rpc(name = "getblockhash")]
    fn get_block_hash(&self, heigh: u64) -> Result<BlockHash, RpcError>;

    #[rpc(name = "getblockheader")]
    fn get_block_header(&self, block_hash: String) -> Result<BlockHeader, RpcError>;

    #[rpc(name = "getblockheaderinfo")]
    fn get_block_header_info(&self, block_hash: String) -> Result<GetBlockHeaderResult, RpcError>;

    #[rpc(name = "getdifficulty")]
    fn get_difficulty(&self) -> Result<f64, RpcError>;

    #[rpc(name = "getblockchaininfo")]
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, RpcError>;

    #[rpc(name = "getblockcount")]
    fn get_block_count(&self) -> Result<u64, RpcError>;

    #[rpc(name = "getbestblockhash")]
    fn get_best_block_hash(&self) -> Result<BlockHash, RpcError>;

    #[rpc(name = "gettxoutsetinfo")]
    fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfoResult, RpcError>;

    /// category mining
    #[rpc(name = "getmininginfo")]
    fn get_mining_info(&self) -> Result<GetMiningInfoResult, RpcError>;

    //TODO: add more...
}

//TODO: there must be a better way than this.
impl<C: BitcoinApi> BitcoinJsonRPCApi for BitcoinConfig<C>
where
    C: Send + Sync + 'static,
{
    fn get_wallet_info(&self) -> Result<GetWalletInfoResult, RpcError> {
        self.0.get_wallet_info().map_err(to_rpc_error)
    }

    fn list_wallets(&self) -> Result<Vec<String>, RpcError> {
        self.0.list_wallets().map_err(to_rpc_error)
    }

    fn uptime(&self) -> Result<u64, RpcError> {
        self.0.uptime().map_err(to_rpc_error)
    }

    fn get_network_info(&self) -> Result<GetNetworkInfoResult, RpcError> {
        self.0.get_network_info().map_err(to_rpc_error)
    }

    fn get_connection_count(&self) -> Result<usize, RpcError> {
        self.0.get_connection_count().map_err(to_rpc_error)
    }

    fn get_peer_info(&self) -> Result<Vec<GetPeerInfoResult>, Error> {
        self.0.get_peer_info().map_err(to_rpc_error)
    }

    fn ping(&self) -> Result<(), RpcError> {
        self.0.ping().map_err(to_rpc_error)
    }

    fn get_net_totals(&self) -> Result<GetNetTotalsResult, RpcError> {
        self.0.get_net_totals().map_err(to_rpc_error)
    }

    fn get_block(&self, block_hash: String) -> Result<Block, RpcError> {
        self.0
            .get_block(&to_block_hash(block_hash)?)
            .map_err(to_rpc_error)
    }

    fn get_block_hash(&self, height: u64) -> Result<BlockHash, RpcError> {
        self.0.get_block_hash(height).map_err(to_rpc_error)
    }

    fn get_block_header(&self, block_hash: String) -> Result<BlockHeader, RpcError> {
        self.0
            .get_block_header(&to_block_hash(block_hash)?)
            .map_err(to_rpc_error)
    }

    fn get_block_header_info(&self, block_hash: String) -> Result<GetBlockHeaderResult, RpcError> {
        self.0
            .get_block_header_info(&to_block_hash(block_hash)?)
            .map_err(to_rpc_error)
    }

    fn get_difficulty(&self) -> Result<f64, RpcError> {
        self.0.get_difficulty().map_err(to_rpc_error)
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, RpcError> {
        self.0.get_blockchain_info().map_err(to_rpc_error)
    }

    fn get_block_count(&self) -> Result<u64, RpcError> {
        self.0.get_block_count().map_err(to_rpc_error)
    }

    fn get_best_block_hash(&self) -> Result<BlockHash, RpcError> {
        self.0.get_best_block_hash().map_err(to_rpc_error)
    }

    fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfoResult, RpcError> {
        self.0.get_tx_out_set_info().map_err(to_rpc_error)
    }

    fn get_mining_info(&self) -> Result<GetMiningInfoResult, RpcError> {
        self.0.get_mining_info().map_err(to_rpc_error)
    }
}

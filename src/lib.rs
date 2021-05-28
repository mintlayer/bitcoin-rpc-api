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

use bitcoincore_rpc::{bitcoin::BlockHash, Auth, Client, RpcApi};
use jsonrpc_core::{Error as RpcError, ErrorCode as RpcErrorCode};
use jsonrpc_derive::rpc;

pub use bitcoincore_rpc_json::*;

pub struct BitcoinConfig<C>(C);

fn to_rpc_error(e: bitcoincore_rpc::Error) -> RpcError {
    RpcError {
        code: RpcErrorCode::InternalError, //TODO: should it?
        message: e.to_string(),
        data: None,
    }
}

/// generate bitcoincore_rpc client
pub fn new_client(btc_url: String, user: String, pass: String) -> BitcoinConfig<Client> {
    let auth = Auth::UserPass(user, pass);
    let client = Client::new(btc_url, auth).unwrap();
    BitcoinConfig(client)
}

#[rpc]
pub trait BitcoinApi {
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

    #[rpc(name = "ping")]
    fn ping(&self) -> Result<(), RpcError>;

    #[rpc(name = "getnettotals")]
    fn get_net_totals(&self) -> Result<GetNetTotalsResult, RpcError>;

    /// category blockchain
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
impl<C: RpcApi> BitcoinApi for BitcoinConfig<C>
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

    fn ping(&self) -> Result<(), RpcError> {
        self.0.ping().map_err(to_rpc_error)
    }

    fn get_net_totals(&self) -> Result<GetNetTotalsResult, RpcError> {
        self.0.get_net_totals().map_err(to_rpc_error)
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

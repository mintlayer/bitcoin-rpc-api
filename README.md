# bitcoin-rpc-api
JSONRPC supported interface using [bitcoincore-rpc's](https://github.com/rust-bitcoin/rust-bitcoincore-rpc) RpcApi trait to access bitcoin RPC methods.

### example bitcoin.conf
```bash
server=1
rest=1
daemon=1

rpcuser=<username>
rpcpassword=<password>
rpcport=8332

rpcallowip=<your_ip_address>

## only for testing, but still, avoid this
#rpcallowip=0.0.0.0/0

rpcallowip=127.0.0.1

rpcbind=127.0.0.1
```

### how to add in substrate's node's rpc.rs:
inside [rpc.rs](https://github.com/mintlayer/mintlayer-node/blob/master/node/src/rpc.rs#L61)
```rust
io.extend_with(
  BitcoinJsonRPCApi::to_delegate(new_client(
    "http://btc.mintlayer.org:8332".to_string(),
    "admin".to_string(),
    "admin".to_string()
  ))
);
```  

### curl example when used with substrate:
following an example in [substrate recipes](https://substrate.dev/recipes/custom-rpc.html),
```bash
 $ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"getnetworkinfo",
      "params": []
    }'
```
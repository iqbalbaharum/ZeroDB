**Generate key**

![Hash process](https://github.com/iqbalbaharum/ZeroDB/blob/deploy-workspace/generate_key_hash.png)

Example:

    {
        token_address: '0xc509dee1e5b4dbdbc08ace7985ffd831538132b6',
        token_id: '0',
        chain_id: '56',
        nonce: 0
    }


**Deployment Process**

For sk, can generate new secret key if desired, can run 

    aqua key create

Deploy service command, in services directory, run this command:

    aqua remote deploy_service --addr /dns4/kras-04.fluence.dev/tcp/19001/wss/p2p/12D3KooWFEwNWcHqi9rtsmDhsYcDbRUCDXH84RC4FW6UfsFWaoHi --sk Wm78Ix1xmtUqPhW0We0ck8bnRHVr5phA+DBGyMgcKe0= --config-path deploy.json --service zero_db_ipfs





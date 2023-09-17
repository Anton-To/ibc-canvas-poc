# ibc-canvas-poc

Attempt at creating a multi-chain version of r/place

Currently deployed on osmosis and archway:
  
osmo1m80jpsnpsqjy7dkm6jsajg30ngq8rhquw4yavtfe40sru888jresdfvn7n  
archway14ckly7w7swtr7wshpmm6gylvfasgp6f50eage7mdhz2nal9hxgwqjyz3df  

# Hermes relayer configuration

Installation
```bash
cd $HOME
wget https://github.com/informalsystems/hermes/releases/download/v1.6.0/hermes-v1.6.0-x86_64-unknown-linux-gnu.tar.gz
mkdir -p $HOME/.hermes/bin
tar -C $HOME/.hermes/bin/ -vxzf hermes-v1.6.0-x86_64-unknown-linux-gnu.tar.gz

cd $HOME
echo "export PATH="$HOME/.hermes/bin:$PATH"" >> ~/.bashrc
source ~/.bashrc
```

Config

```bash
tee $HOME/.hermes/config.toml > /dev/null <<EOF
[global]
log_level = 'debug'

[mode]

[mode.clients]
enabled = true
refresh = true
misbehaviour = true

[mode.connections]
enabled = true

[mode.channels]
enabled = true

[mode.packets]
enabled = true
clear_interval = 100
clear_on_start = true
tx_confirmation = true

[telemetry]
enabled = true
host = '127.0.0.1'
port = 3001

[[chains]]
id = 'constantine-3'
rpc_addr = 'https://rpc.constantine.archway.tech:443'
grpc_addr = 'https://grpc.constantine.archway.tech:443'
event_source = { mode = 'pull', interval = '4s' }
rpc_timeout = '15s'
account_prefix = 'archway'
key_name = 'archkey'
store_prefix = 'ibc'
gas_price = { price = 900000000000, denom = 'aconst' }
max_gas = 2000000
clock_drift = '5s'
trusting_period = '21599s'
trust_threshold = { numerator = '1', denominator = '3' }

[chains.packet_filter]
policy = 'allow'
list = [
    ['wasm.archway14ckly7w7swtr7wshpmm6gylvfasgp6f50eage7mdhz2nal9hxgwqjyz3df', 'channel-53'],
]

[[chains]]
id = 'osmo-test-5'
rpc_addr = 'https://rpc.osmotest5.osmosis.zone:443'
grpc_addr = 'https://grpc.osmotest5.osmosis.zone:443'
event_source = { mode = 'pull', interval = '4s' }
rpc_timeout = '15s'
account_prefix = 'osmo'
key_name = 'osmokey'
store_prefix = 'ibc'
gas_price = { price = 0.01, denom = 'uosmo' }
max_gas = 10000000
clock_drift = '5s'
trusting_period = '4days'
trust_threshold = { numerator = '1', denominator = '3' }

[chains.packet_filter]
policy = 'allow'
list = [
    ['wasm.osmo1m80jpsnpsqjy7dkm6jsajg30ngq8rhquw4yavtfe40sru888jresdfvn7n', 'channel-1654'],
]
EOF
```
Channels clients and connections are created by following instructions at https://hermes.informal.systems/documentation/commands/path-setup/index.html

```bash
hermes create channel --a-chain constantine-3 --b-chain osmo-test-5 --a-port wasm.archway14ckly7w7swtr7wshpmm6gylvfasgp6f50eage7mdhz2nal9hxgwqjyz3df --b-port wasm.osmo1m80jpsnpsqjy7dkm6jsajg30ngq8rhquw4yavtfe40sru888jresdfvn7n --channel-version multi-place-1 --new-client-connection
```

Add keys
```bash
echo [MNEMONIC] > mnemonic_file.json
hermes keys add --key-name archkey --hd-path "m/44'/118'/0'/0/0" --chain constantine-3 --mnemonic-file mnemonic_file.json
hermes keys add --key-name osmokey --hd-path "m/44'/118'/0'/0/0" --chain osmo-test-5 --mnemonic-file mnemonic_file.json
rm mnemonic_file.json
```

Run
```bash
hermes start
```





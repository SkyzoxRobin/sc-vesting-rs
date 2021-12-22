OWNER="../wallet-owner.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
EGLD="1000000000000000000" # 18 decimals

BYTECODE="output/template.wasm"


deploy() {
    arg1="value"

    erdpy --verbose contract deploy --bytecode=${BYTECODE} --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=590000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $arg1 \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

template() {
    arg1="0x$(echo -n 'value' | xxd -p -u | tr -d '\n')"
    arg2="0x$(printf '%x' value)"
    arg3="0x$(erdpy wallet bech32 --decode 'value')"

     erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="template" \
        --value=100 \
        --arguments $arg1 $args2 $arg3 \
        --send || return
}

upgrade() {
    arg1="value"

    erdpy --verbose contract upgrade ${ADDRESS} --bytecode=${BYTECODE} --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=590000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $arg1 \
        --outfile="deploy-devnet.interaction.json" --send || return

    echo ""
    echo "Smart contract upgraded address: ${ADDRESS}"
}

query() {
    erdpy --verbose contract query ${ADDRESS} --function="viewFunction" --proxy=${PROXY}
}
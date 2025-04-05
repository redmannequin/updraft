import json
import borsh
import base64
import requests 

from borsh import types
from typing import Any

def get_tx(tx: str) -> Any:
    url = "https://api.mainnet-beta.solana.com"
    headers = { 
        "Content-Type": "application/json" 
    }
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            tx,
            "json"
        ]
    }
    res = requests.post(url, headers=headers, json=payload)
    return res.json()

def swap_event(data: str):
    data_bytes = base64.b64decode(data)

    public_key = types.fixed_array(types.u8, 32)
    
    swap_event_schema = borsh.schema({
        'pool_state': public_key,
        'sender': public_key,
        'token_account_0': public_key,
        'token_account_1': public_key,
        'amount_0': types.u64,
        'transfer_fee_0': types.u64,
        'amount_1': types.u64,
        'transfer_fee_1': types.u64,
        'zero_for_one': types.u8,
        'sqrt_price_x64': types.u128,
        'liquidity': types.u128,
        'tick': types.i32,
    })

    swap = borsh.deserialize(swap_event_schema, data_bytes[8:])
    print(list(data_bytes[:8]))
    print(swap)


data = "QMbN6CYIceLlm6egw6j8/oGnkb/15tghCp4hh9fOuOzfc1JuIHDGn7Bf9sNWNXjGEqXillKJZkIK9hZRGfxA1Vp+jw3T4DqCzuBvhC8XiVmZCPSwwCni+vbhNVFjoPBAzt9K8VSm2oSY7Ex0MZ7+HToxtg/p5zLeFmruDq4KXMt0yG0kTNY+e3ZzdwgAAAAAAAAAAAAAAAD12PxQAgAAAAAAAAAAAAAAAUkooQiKchdhCAAAAAAAAADpYrTNGCgAAAAAAAAAAAAAFaYAAA=="
swap_event(data)
tx = get_tx("4RTxhFarxUFZApeTMcuR3z3oX1xhxh1Bpkt8npDzMH8kBFSFU3iwKSqhAzqS2Dc3Y4Z7nCq2c1ZKL38jnVfprKH8")
print(json.dumps(tx, indent=2))
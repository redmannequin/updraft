import borsh
import base64
from borsh import types

data = "QMbN6CYIceLlm6egw6j8/oGnkb/15tghCp4hh9fOuOzfc1JuIHDGn7Bf9sNWNXjGEqXillKJZkIK9hZRGfxA1Vp+jw3T4DqCzuBvhC8XiVmZCPSwwCni+vbhNVFjoPBAzt9K8VSm2oSY7Ex0MZ7+HToxtg/p5zLeFmruDq4KXMt0yG0kTNY+e3ZzdwgAAAAAAAAAAAAAAAD12PxQAgAAAAAAAAAAAAAAAUkooQiKchdhCAAAAAAAAADpYrTNGCgAAAAAAAAAAAAAFaYAAA=="
data_bytes = base64.b64decode(data)

swap_event_schema = borsh.schema({
    'pool_state': types.fixed_array(types.u8, 32),
    'sender': types.fixed_array(types.u8, 32),
    'token_account_0': types.fixed_array(types.u8, 32),
    'token_account_1': types.fixed_array(types.u8, 32),
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
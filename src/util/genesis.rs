extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use data::state;

// Returns Genesis State
// Output   state       Genesis State
pub fn get_genesis() -> state::state{
    state::state {
        nonce           :   0,
        hash            :   "Genesis State".to_string(),
        prev_state      :   "".to_string(),
        acc_hash        :   "Genesis Log".to_string(),
        l_hash          :   "Genesis Proof".to_string(),
        fuel_exp        :   100,
    }
}

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn test(bi: BigInt) -> String {
    bi.get_u64().1.to_string()
}
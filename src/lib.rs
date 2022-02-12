use napi_derive::napi;

use std::{thread, time::Duration};

#[napi]
struct TestStruct {
    number: u32
}

#[napi]
impl TestStruct {
    #[napi]
    pub fn to_string(&self) -> String {
        self.number.to_string()
    }

    #[napi(factory)]
    pub fn from_number(n: u32) -> Self {
        thread::sleep(Duration::from_secs(2));
        TestStruct{ number: n }
    }
}
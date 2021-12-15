#![recursion_limit = "2048"]

#[macro_use]
extern crate napi_derive;

use bengbenge;

#[napi(js_name = "BengBenge")]
pub struct BengBengeFactory {
    bbe: bengbenge::BengBenge,
}

#[napi]
impl BengBengeFactory {
    #[napi(constructor)]
    pub fn new() -> Self {
        BengBengeFactory {
            bbe: bengbenge::BengBenge::new(),
        }
    }

    #[napi]
    pub fn append(&mut self, value: String) {
        self.bbe.append(value);
    }

    #[napi]
    pub fn next(&mut self) -> Option<String> {
        self.bbe.next()
    }
}

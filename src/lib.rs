#![allow(unused_variables)]
#![allow(dead_code)]

pub struct BengBenge {
    dns: String
}

impl BengBenge {
    pub fn new(dns: String) -> BengBenge {
        BengBenge {dns}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let bb = BengBenge::new("bengbenge".to_string());
        assert_eq!(bb.dns, "bengbenge");
    }
}
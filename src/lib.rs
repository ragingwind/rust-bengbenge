#![allow(dead_code)]

#[cfg(not(has_std))]
pub struct BengBenge {
    container: Vec<String>,
    cursor: usize,
}

impl BengBenge {
    fn new() -> BengBenge {
        BengBenge {
            container: Vec::new(),
            cursor: 0,
        }
    }

    fn append(&mut self, value: String) {
        self.container.push(value);
    }
}

impl Iterator for BengBenge {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.container.get(self.cursor) {
            Some(value) => {
                self.cursor = if self.cursor + 1 < self.container.len() { self.cursor + 1 } else { 0 };
                Some(value.to_owned())
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_address_infinity() {
        let mut bb = BengBenge::new();

        bb.append("192.168.0.1".to_string());
        bb.append("192.168.0.2".to_string());
        bb.append("192.168.0.3".to_string());
        bb.append("192.168.0.4".to_string());

        assert_eq!("192.168.0.1", bb.next().unwrap());
        assert_eq!("192.168.0.2", bb.next().unwrap());
        assert_eq!("192.168.0.3", bb.next().unwrap());
        assert_eq!("192.168.0.4", bb.next().unwrap());
        assert_eq!("192.168.0.1", bb.next().unwrap());
        assert_eq!("192.168.0.2", bb.next().unwrap());
        assert_eq!("192.168.0.3", bb.next().unwrap());
        assert_eq!("192.168.0.4", bb.next().unwrap());
    }

    #[test]
    fn it_loops_infinity() {
        let mut bb = BengBenge::new();

        bb.append("192.168.0.1".to_string());
        bb.append("192.168.0.2".to_string());
        bb.append("192.168.0.3".to_string());
        bb.append("192.168.0.4".to_string());

        let mut addreses = vec!["192.168.0.1", "192.168.0.2", "192.168.0.3", "192.168.0.4"];
        addreses.reverse();

        while let Some(address) = bb.next() {
            assert_eq!(addreses.pop().unwrap(), address);

            if addreses.len() == 0 {
                break;
            }
        }
    }
}

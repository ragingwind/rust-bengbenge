#![allow(dead_code)]

use std::slice::Iter;

struct BengBenge {
    container: Vec<String>,
    cursor: usize,
}

impl BengBenge {
    fn new() -> BengBenge {
        BengBenge{
            container: Vec::new(),
            cursor: 0,
        }
    }

    fn append(&mut self, value: String) {
        self.container.push(value);
    }

    fn iter(&self) -> Iter<String> {
        self.container.iter()
    }
}

impl Iterator for BengBenge {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.container.get(self.cursor) {
            Some(value) => {
                self.cursor += 1;
                Some(value.to_owned())
            },
            None => {
                self.cursor = 0;
                Some(self.container.get(self.cursor).unwrap().to_owned())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut bb = BengBenge::new();

        bb.append("192.168.0.1".to_string());
        bb.append("192.168.0.2".to_string());
        bb.append("192.168.0.3".to_string());
        bb.append("192.168.0.4".to_string());

        let mut iterator = bb.iter();

        assert_eq!("192.168.0.1", iterator.next().unwrap());
        assert_eq!("192.168.0.2", iterator.next().unwrap());
        assert_eq!("192.168.0.3", iterator.next().unwrap());
        assert_eq!("192.168.0.4", iterator.next().unwrap());
    }
}
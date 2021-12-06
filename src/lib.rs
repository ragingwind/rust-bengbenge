#[allow(dead_code)]
struct BengBenge {

}

#[allow(dead_code)]
struct BengBenges {
    benge: [BengBenge; 0]
}

#[allow(dead_code)]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn it_works() {
        // assert_eq!(sum(2, 2), 4);
    }
}


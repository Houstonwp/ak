pub mod ast;
pub mod contract;
pub mod observable;
pub mod parser;
pub mod simulation;
pub mod visitor;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

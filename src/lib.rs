mod error;
pub use self::error::*;

pub mod adapter;
pub mod uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

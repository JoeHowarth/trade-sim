pub mod types;
pub mod market;
// pub mod types;
pub mod prelude;
pub mod agent;

pub fn from_lib() -> String {
   "hi".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

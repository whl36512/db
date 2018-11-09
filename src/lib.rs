#[macro_use] extern crate log;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_postgres;

pub mod db;
pub mod util;
pub mod constants;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

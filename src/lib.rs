//! Secrets is a simple library for retrieving environment specific
//! variables. It's basically `dotenv` so you probably wanna use that instead.
//! This library is designed to only work with utf8 characters.

use std::collections::HashMap;


pub mod error;

pub struct Secret {
    data: HashMap<String, String>,
}

impl Secret {
    pub fn new(path: &str) -> Secret {
        let data = parse_file(path);
        match data {
            Ok(data) => {
                Secret {
                    data,
                }
            },
            Err(e) => panic!("Could not create instance of Secret due to: {:?}", e)
        }


    }
    pub fn tell(key: &str) -> Result<String, &'static str> {
        todo!();
    }
}

fn parse_file(path: &str) -> Result<HashMap<String, String>, error::Error> {
    todo!();
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn tell_test() {
        assert!(false);
    }
}

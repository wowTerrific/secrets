//! Secrets is a simple library for retrieving environment specific
//! variables. It's basically `dotenv` so you probably wanna use that instead.
//! This library is designed to only work with utf8 characters, and will trim
//! any extra white space off of lines. The format of the `.secret` file is to
//! be the variable name and variable value separated by `=` with no space between. 
//! Each variable / value combination needs to be separated with a new line.
//! For example:
//! ```text
//! VARIABLE_NAME=variable_value
//! 
//! PASSWORD=password12345
//! ```

use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Secret {
    data: HashMap<String, String>,
}

impl Secret {
    pub fn new(path: &str) -> Secret {
        let data = parse_file(path);

        Secret {
            data,
        }
    }

    /// [tell()] returns the value of the corresponding key as a `String`. If the the
    /// key does not match anything within the `.secret` file, the program will
    /// `panic!` in order to make sure you fix your file before your program will
    /// compile.
    pub fn tell(&self, key: &str) -> String {
        let query = &self.data.get(key);

        if let Some(val) = query {
            String::from(*val)
        } else {
            panic!("value does not exist for {:?}", key);
        }
    }
}

fn parse_file(path: &str) -> HashMap<String, String> {
    let mut value_map = HashMap::new();
    let secret_file = read_to_string(path)
        .expect("Could not find `.secret` file in path specified.");

    for line in secret_file.lines() {

        if line.contains('=') {
            let (key, value) = separate_values_by_eq(line);
            value_map.insert(key, value);
        }
    }

    value_map
}

fn separate_values_by_eq(line: &str) -> (String, String) {
    let mut key_value = line.split('=');
    let fail_message = "\nCould not parse `.secret` file.\nPlease check format and try again...";

    let key = key_value.next().expect(fail_message).trim();
    let value = key_value.next().expect(fail_message).trim();

    (key.to_owned(), value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tell_test() {
        let secrets = Secret::new("./tests/data/.secret");
        assert_eq!(String::from("abcd"), secrets.tell("DATABASE"));
        assert_eq!(String::from("B3x03#?..[]"), secrets.tell("PASSWORD"));
        assert_eq!(String::from("t"), secrets.tell("SPACECHARACTERS"));
        assert_eq!(String::from("123a##!!~"), secrets.tell("GREG"));
    }

    #[test]
    fn separate_values() {
        let line = "DATABASE=abcd";
        let expected = (String::from("DATABASE"), String::from("abcd"));
        let result = separate_values_by_eq(line);
        assert_eq!(expected, result);

        let line = "PASSWORD=B3x03#?..[]";
        let expected = (String::from("PASSWORD"), String::from("B3x03#?..[]"));
        let result = separate_values_by_eq(line);
        assert_eq!(expected, result);

        let line = "SPACECHARACTERS=    t   ";
        let expected = (String::from("SPACECHARACTERS"), String::from("t"));
        let result = separate_values_by_eq(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_test_file() {
        let test_secrets = parse_file("./tests/data/.secret");

        assert!(!test_secrets.is_empty());
        assert_eq!(test_secrets.get("DATABASE").unwrap(), &String::from("abcd"));
        assert_eq!(test_secrets.get("PASSWORD").unwrap(), &String::from("B3x03#?..[]"));
        assert_eq!(test_secrets.get("SPACECHARACTERS").unwrap(), &String::from("t"));
        assert_eq!(test_secrets.get("GREG").unwrap(), &String::from("123a##!!~"));
        assert_eq!(test_secrets.get(""), None);
    }

}
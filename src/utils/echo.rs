use std::io;
use crate::utils::utilabc::UtilAbc;

pub struct EchoUtil {
    
}

impl UtilAbc for EchoUtil {
    fn get_name(&self) -> String {
        "LOOPBACK".to_string()
    }

    fn run(&self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        println!("{}", input);
    }
}
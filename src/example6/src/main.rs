use std::str::FromStr;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    println!("{:?}", u16::parse("23333 hello world!"))
}

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
        where Self: Sized;
}

impl<T> Parse for T where T: FromStr + Default {
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> where Self: Sized {
        let reg: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = reg.captures(s) {
            captures.get(0).map_or(Err("Invalid number".to_string()), |m| {
                m.as_str()
                    .parse::<T>()
                    .map_err(|_| "Invalid number".to_string())
            })
        } else {
            //println!("{:?}", s);
            Err("Invalid number".to_string())
        }
    }
}


#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert_eq!(u32::parse("123.45abcd"), Err("failed to parse captured string".into()));
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

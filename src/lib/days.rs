use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DaysSec {
    pub day_sec: u64,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for DaysSec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = |_| anyhow!(format!("Failed to parse size from {}", s));
        let days = s.parse::<u64>().map_err(err)?;
        let day_sec = 24 * 60 * 60;
        Ok(Self {
            day_sec: days * day_sec,
        })
    }
}

#[test]
fn file_size_parse_test() {
    let day_sec = 24 * 60 * 60;
    let k = "3".parse::<DaysSec>().unwrap();
    assert_eq!(
        k,
        DaysSec {
            day_sec: 3 * day_sec
        }
    );
}

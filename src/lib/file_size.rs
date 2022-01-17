use anyhow::{anyhow, Result};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FileSize {
    pub size: u64,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for FileSize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!(format!("Failed to parse size from {}", s));
        let err_parse = |_| anyhow!(format!("Failed to parse size from {}", s));
        let re = Regex::new(r"([0-9]+)[gG]([0-9]+)[mM]([0-9]+)[kK]")?;
        let caps = re.captures(s).ok_or_else(err)?;

        let g = caps
            .get(1)
            .map_or("0", |m| m.as_str())
            .parse::<u64>()
            .map_err(err_parse)?;
        let m = caps
            .get(2)
            .map_or("0", |m| m.as_str())
            .parse::<u64>()
            .map_err(err_parse)?;
        let k = caps
            .get(3)
            .map_or("0", |m| m.as_str())
            .parse::<u64>()
            .map_err(err_parse)?;

        let kilo = 1024;
        Ok(Self {
            size: k * kilo + m * kilo * kilo + g * kilo * kilo * kilo,
        })
    }
}

#[test]
fn file_size_parse_test() {
    let kilo = 1024;
    let k = "0g0m01k".parse::<FileSize>().unwrap();
    assert_eq!(k, FileSize { size: 1 * kilo });

    let m = "0g3m2k".parse::<FileSize>().unwrap();
    assert_eq!(
        m,
        FileSize {
            size: 3 * kilo * kilo + 2 * kilo
        }
    );

    let g = "5g3m2k".parse::<FileSize>().unwrap();
    assert_eq!(
        g,
        FileSize {
            size: 5 * kilo * kilo * kilo + 3 * kilo * kilo + 2 * kilo
        }
    );
}

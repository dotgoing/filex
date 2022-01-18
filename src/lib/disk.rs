use anyhow::{anyhow, Result};
use fs2;
use std::str::FromStr;

#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub available: u64,
}

pub fn get_disk_info() -> Result<DiskInfo, anyhow::Error> {
    let total_err = |_| anyhow!(format!("Fail to get total size"));
    let available_err = |_| anyhow!(format!("Fail to get total size"));

    let total = fs2::total_space(".").map_err(total_err)?;
    let available = fs2::available_space(".").map_err(available_err)?;

    Ok(DiskInfo { total, available })
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DiskFreePercent {
    pub percent: u64,
}

impl FromStr for DiskFreePercent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = |_| {
            anyhow!(format!(
                "Failed to parse disk free percent from {}. it should be usize",
                s
            ))
        };
        let percent = s.parse::<u64>().map_err(err)?;

        if percent > 100 {
            return Err(anyhow!(format!(
                "disk free percent should be between 0 and 100"
            )));
        }

        Ok(Self { percent })
    }
}

#[test]
fn file_size_parse_test() {
    let dfp = "34".parse::<DiskFreePercent>().unwrap();
    assert_eq!(dfp, DiskFreePercent { percent: 34 });

    let dfp = "100".parse::<DiskFreePercent>().unwrap();
    assert_eq!(dfp, DiskFreePercent { percent: 100 });
}

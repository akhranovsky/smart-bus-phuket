#![allow(dead_code)]

use anyhow::{bail, ensure, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Bus {
    pub no: u8,
    pub licence_plate_no: String,
    pub id: String,
    // _icon: String,
    pub service_status: ServiceStatus,
    pub direction: Direction,
    pub operate_position: String,
    // _a: String,
    // _b: String,
    // _c: String,
    // _d: String,
    // _e: String,
    // _f: String,
    // _concat: String,
    // _run: String,
    // pub date_time: NaiveDateTime,
    // pub date: NaiveDate,
    // pub time: NaiveTime,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Suspend,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Direction {
    #[serde(rename = "0")]
    A,
    #[serde(rename = "1")]
    B,
}

impl TryFrom<&Value> for Bus {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Some(array) = value.as_array() else {
            bail!("expected array");
        };

        ensure!(array.len() == 17, "expected 17 items, got {}", array.len());

        let get_str = |index: usize| {
            let s = array[index]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("expected str at position {index}"))
                .map(ToString::to_string);
            s
        };

        Ok(Self {
            no: get_str(0)?.parse()?,
            licence_plate_no: get_str(1)?,
            id: get_str(2)?,
            // _icon: get_str(3)?,
            service_status: serde_json::from_value(array[4].clone())?,
            direction: serde_json::from_value(array[5].clone())?,
            operate_position: get_str(6)?,
            // _a: get_str(7)?,
            // _b: get_str(8)?,
            // _c: get_str(9)?,
            // _d: get_str(10)?,
            // _e: get_str(11)?,
            // _f: get_str(12)?,
            // _concat: get_str(13)?,
            // _run: get_str(14)?,
            // date_time: NaiveDateTime::new(
            //     NaiveDate::parse_from_str(&get_str(15)?, "%d/%m/%Y")?,
            //     NaiveTime::parse_from_str(&get_str(16)?, "%r")?,
            // ),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::TEST_BUSES, test_parse};

    use super::*;

    test_parse!(Bus, TEST_BUSES, 14);
}

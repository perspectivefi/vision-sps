use std::str::FromStr;
use substreams::prelude::BigDecimal;
use crate::constants::{ROUNDED_ONE_DAY_IN_SECONDS, SECONDS_PER_YEAR};

pub fn get_day_timestamp(timestamp: u64) -> u64 {
    (timestamp / ROUNDED_ONE_DAY_IN_SECONDS) * ROUNDED_ONE_DAY_IN_SECONDS
}

pub fn format_key(address: String, timestamp: u64) -> String {
    format!("{}-{}", address, timestamp)
}

pub fn calculate_apr(previous_rate: String, current_rate: String, time_delta: u64) -> String {
    let time_factor: u64 = SECONDS_PER_YEAR / time_delta;

    let previous_rate_big_decimal = BigDecimal::from_str(&previous_rate).unwrap_or_default();
    let current_rate_big_decimal = BigDecimal::from_str(&current_rate).unwrap_or_default();

    if previous_rate_big_decimal == BigDecimal::from(0) || time_delta == 0 {
        return "0".to_string();
    }

    let apr = (((current_rate_big_decimal / previous_rate_big_decimal) - BigDecimal::from(1))
        * BigDecimal::from(time_factor)
        * BigDecimal::from(100))
        .to_string();

    apr.to_string()
}
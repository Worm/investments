use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Deserializer};
use serde::de::Error;

use core::GenericResult;
use types::{Date, Decimal};
use util;

fn parse_date(date: &str) -> GenericResult<Date> {
    util::parse_date(date, "%Y-%m-%dT00:00:00")
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where D: Deserializer<'de>
{
    let value: String = Deserialize::deserialize(deserializer)?;
    parse_date(&value).map_err(D::Error::custom)
}

pub fn parse_security_description(mut issuer: &str) -> &str {
    if let Some(index) = issuer.find("п/у") {
        issuer = &issuer[..index];
    }

    if let Some(index) = issuer.find('(') {
        issuer = &issuer[..index];
    }

    issuer.trim()
}

pub fn parse_quantity(decimal_quantity: Decimal) -> GenericResult<u32> {
    Ok(decimal_quantity.to_u32().and_then(|quantity| {
        if Decimal::from_u32(quantity).unwrap() == decimal_quantity {
            Some(quantity)
        } else {
            None
        }
    }).ok_or_else(|| format!("Invalid quantity: {}", decimal_quantity))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_parsing() {
        assert_eq!(parse_date("2017-12-31T00:00:00").unwrap(), date!(31, 12, 2017));
    }

    #[test]
    fn security_description_parsing() {
        assert_eq!(parse_security_description(
            "FinEx MSCI China UCITS ETF (USD Share Class) п/у FinEx Investment Management LLP"),
            "FinEx MSCI China UCITS ETF");
    }
}
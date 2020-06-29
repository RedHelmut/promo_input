use chrono::prelude::*;
use serde::de::{self, Visitor, Error};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct PromoDateNative(pub NaiveDate);

pub fn convert_to_date(input: i64) -> PromoDateNative {
    let mut old_date = chrono::NaiveDate::from_ymd(1899, 12, 30);
    let days = chrono::Duration::days(input as i64);
    old_date = old_date + days;
    let prom_dta = PromoDateNative(old_date);

    prom_dta
}
impl<'de> Deserialize<'de> for PromoDateNative {
    fn deserialize<D>(deserializer: D) -> Result<PromoDateNative, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PromoDateNativeVisitor)
    }
}
impl Default for PromoDateNative {
    fn default() -> Self {
        PromoDateNative {
            0: NaiveDate::from_ymd(2000, 1, 1),
        }
    }
}
impl fmt::Display for PromoDateNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.0.month(), self.0.day(), self.0.year())
    }
}
impl PromoDateNative {
    pub fn get_month_year(&self) -> String {
        match self.0.month0() {
            0 => format!("Jan {}", self.0.year()),
            1 => format!("Feb {}", self.0.year()),
            2 => format!("Mar {}", self.0.year()),
            3 => format!("Apr {}", self.0.year()),
            4 => format!("May {}", self.0.year()),
            5 => format!("Jun {}", self.0.year()),
            6 => format!("Jul {}", self.0.year()),
            7 => format!("Aug {}", self.0.year()),
            8 => format!("Sept {}", self.0.year()),
            9 => format!("Oct {}", self.0.year()),
            10 => format!("Nov {}", self.0.year()),
            11 => format!("Dec {}", self.0.year()),
            _ => format!("Jan {}", self.0.year()),
        }
    }
}
///works best for csv parse
impl serde::Serialize for PromoDateNative {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let stb = self.0.format("%m/%d/%Y").to_string();
        serializer.serialize_str(&stb)
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Default)]
pub struct PromoDateNativeVisitor;
impl<'de> Visitor<'de> for PromoDateNativeVisitor {
    type Value = PromoDateNative;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a date in format m/d/y")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match chrono::NaiveDate::parse_from_str(value, "%m/%d/%Y") {
            Ok(date) => Ok(PromoDateNative { 0: date }),
            Err(_) => match chrono::NaiveDate::parse_from_str(value, "%m-%d-%Y") {
                Ok(date) => Ok(PromoDateNative { 0: date }),
                Err(_) => match value.parse::<f64>() {
                    Ok(valid_f64) => {
                        let prom_dta = convert_to_date(valid_f64 as i64);
                        Ok(prom_dta)
                    }
                    Err(_) => Err(serde::de::Error::custom(
                        "Bad date format please use m/d/y or m-d-y",
                    )),
                },
            },
        }
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match chrono::NaiveDate::parse_from_str(&value, "%m/%d/%Y") {
            Ok(date) => Ok(PromoDateNative { 0: date }),
            Err(_) => match chrono::NaiveDate::parse_from_str(&value, "%m-%d-%Y") {
                Ok(date) => Ok(PromoDateNative { 0: date }),
                Err(_) => match value.parse::<f64>() {
                    Ok(valid_f64) => {
                        let prom_dta = convert_to_date(valid_f64 as i64);
                        Ok(prom_dta)
                    }
                    Err(_) => Err(serde::de::Error::custom(
                        "Bad date format please use m/d/y or m-d-y",
                    )),
                },
            },
        }
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut old_date = chrono::NaiveDate::from_ymd(1899, 12, 31);
        let days = chrono::Duration::days(value as i64);
        old_date = old_date + days;
        let prom_dta = PromoDateNative(old_date);

        match chrono::NaiveDate::parse_from_str(&prom_dta.to_string(), "%m/%d/%Y") {
            Ok(date) => Ok(PromoDateNative { 0: date }),
            Err(_) => match chrono::NaiveDate::parse_from_str(&prom_dta.to_string(), "%m-%d-%Y") {
                Ok(date) => Ok(PromoDateNative { 0: date }),
                Err(_) => Err(serde::de::Error::custom(
                    "Bad date format please use m/d/y or m-d-y",
                )),
            },
        }
    }
}


#[derive(Clone, PartialEq)]
pub enum AndOrType {
    And,
    Or,
    Any(i64),
    None,
}
impl From<&AndOrType> for String {
    fn from( typ: &AndOrType) -> Self {
        let mut ret: String = String::new();

        let txt = match typ {
            AndOrType::And => "And",
            AndOrType::Or => "Or",
            AndOrType::Any(many) => {
                ret = format!("Any({})", many);
                ret.as_str()
            }
            AndOrType::None => "None",
        };

        txt.to_owned()
    }
}

impl<'de> Deserialize<'de> for AndOrType {
    fn deserialize<D>(deserializer: D) -> Result<AndOrType, D::Error>
        where
            D: Deserializer<'de>,
    {
        let deser_result: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
        let input_str: serde_json::Value = deser_result;
        match input_str.as_str() {
            Some(ed_str) => {
                let split_at_pound: Vec<_> = ed_str.trim().split("#").collect();

                if split_at_pound.len() > 0 {
                    let mut val: i64 = 1;
                    if split_at_pound.len() > 1 {
                        if let Ok(good_val) = split_at_pound[1].parse::<i64>() {
                            val = good_val;
                        }
                    }
                    match split_at_pound[0].to_uppercase() {
                        ref s if s == "OR" => Ok(AndOrType::Or),
                        ref s if s == "AND" => Ok(AndOrType::And),
                        ref s if s == "ANY" => Ok(AndOrType::Any(val)),
                        ref s if s == "NONE" => Ok(AndOrType::None),
                        _ => Err(serde::de::Error::custom("Unexpected value")),
                    }
                } else {
                    Err(serde::de::Error::custom("Unexpected value"))
                }
            }
            None => Err(serde::de::Error::custom("Unexpected value")),
        }
    }
}

impl Serialize for AndOrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut ret: String = String::new();
        serializer.serialize_str(match *self {
            AndOrType::And => "And",
            AndOrType::Or => "Or",
            AndOrType::Any(many) => {
                ret = format!("Any({})", many);
                ret.as_str()
            }
            AndOrType::None => "None",
        })
    }
}
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

impl fmt::Display for AndOrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut ret: String = String::new();

        let txt = match *self {
            AndOrType::And => "And",
            AndOrType::Or => "Or",
            AndOrType::Any(many) => {
                ret = format!("Any({})", many);
                ret.as_str()
            }
            AndOrType::None => "None",
        };
        write!(f, "{}", txt)
    }
}

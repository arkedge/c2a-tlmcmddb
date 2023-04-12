use std::io::Read;

use anyhow::{anyhow, ensure, Result};
use csv::StringRecord;
use serde::{de::Visitor, Deserialize, Deserializer};
use tlmcmddb::cmd as model;

use crate::{escape::unescape, macros::check_header, util};

/*
+------------+-------+---------+-------+---------------------------------------------------------------------------------------------------------------------------+---------+-------------+--------------+-------+
| Component  |       |         |       |                                                        Params                                                             |         |             |              |       |
+------------+       |         |       +---------+------------------+---------+--------+------------------+------------------+------------------+------------------+         |             |              |       |
| MOBC       | Name  | Target  | Code  | Num     |      Param1      |      Param2      |      Param3      |      Param4      |      Param5      |      Param6      | Danger  | Is          | Description  | Note  |
+------------+       |         |       | Params  +---------+--------+---------+--------+---------+--------+---------+--------+---------+--------+---------+--------+ Flag    | Restricted  |              |       |
| Comment    |       |         |       |         | Type    | Descr  | Type    | Descr  | Type    | Descr  | Type    | Descr  | Type    | Descr  | Type    | Descr  |         |             |              |       |
+------------+-------+---------+-------+---------+---------+--------+---------+--------+---------+--------+---------+--------+---------+--------+---------+--------+---------+-------------+--------------+-------+
*/

mod header {
    pub const COMPONENT: &str = "Component";
    pub const NAME: &str = "Name";
    pub const COMMENT: &str = "Comment";
    pub const TARGET: &str = "Target";
    pub const CODE: &str = "Code";
    pub const PARAMS: &str = "Params";
    pub const DANGER_FLAG: &str = "Danger Flag";
    pub const IS_RESTRICTED: &str = "Is Restricted";
    pub const DESCRIPTION: &str = "Description";
    pub const NOTE: &str = "Note";
    pub const NUM_PARAMS: &str = "Num Params";
    pub const PARAM1: &str = "Param1";
    pub const PARAM2: &str = "Param2";
    pub const PARAM3: &str = "Param3";
    pub const PARAM4: &str = "Param4";
    pub const PARAM5: &str = "Param5";
    pub const PARAM6: &str = "Param6";
    pub const PARAM_TYPE: &str = "Type";
    pub const PARAM_DESCRIPTION: &str = "Description";
}

fn check_first_header(record: StringRecord) -> Result<()> {
    ensure!(record.len() >= 21, "the number of columns is mismatch");
    check_header!(&record[0], header::COMPONENT);
    check_header!(&record[1], header::NAME);
    check_header!(&record[2], header::TARGET);
    check_header!(&record[3], header::CODE);
    check_header!(&record[4], header::PARAMS);
    check_header!(&record[17], header::DANGER_FLAG);
    check_header!(&record[18], header::IS_RESTRICTED);
    check_header!(&record[19], header::DESCRIPTION);
    check_header!(&record[20], header::NOTE);
    Ok(())
}

fn parse_second_header(record: StringRecord) -> Result<String> {
    ensure!(record.len() >= 16, "the number of columns is mismatch");
    check_header!(&record[4], header::NUM_PARAMS);
    check_header!(&record[5], header::PARAM1);
    check_header!(&record[7], header::PARAM2);
    check_header!(&record[9], header::PARAM3);
    check_header!(&record[11], header::PARAM4);
    check_header!(&record[13], header::PARAM5);
    check_header!(&record[15], header::PARAM6);
    let component = unescape(&record[0]);
    Ok(component)
}

fn check_third_header(record: StringRecord) -> Result<()> {
    ensure!(record.len() >= 17, "the number of columns is mismatch");
    check_header!(&record[0], header::COMMENT);
    check_header!(&record[5], header::PARAM_TYPE);
    check_header!(&record[6], header::PARAM_DESCRIPTION);
    check_header!(&record[7], header::PARAM_TYPE);
    check_header!(&record[8], header::PARAM_DESCRIPTION);
    check_header!(&record[9], header::PARAM_TYPE);
    check_header!(&record[10], header::PARAM_DESCRIPTION);
    check_header!(&record[11], header::PARAM_TYPE);
    check_header!(&record[12], header::PARAM_DESCRIPTION);
    check_header!(&record[13], header::PARAM_TYPE);
    check_header!(&record[14], header::PARAM_DESCRIPTION);
    check_header!(&record[15], header::PARAM_TYPE);
    check_header!(&record[16], header::PARAM_DESCRIPTION);
    Ok(())
}

fn build_comment(record: StringRecord) -> model::Comment {
    let mut text = String::new();
    for col in record.iter() {
        text.push_str(&unescape(col));
        text.push(',');
    }
    text.truncate(text.len() - 1); // trim last comma
    model::Comment { text }
}

fn parse_body<I, E>(mut iter: I) -> Result<Vec<model::Entry>>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut entries = vec![];
    while let Some(record) = util::try_next_record(&mut iter)? {
        ensure!(record.len() >= 21, "the number of columns is mismatch");
        if record[0].is_empty() {
            let line: Line = record.deserialize(None)?;
            let command = line.try_into()?;
            entries.push(model::Entry::Command(command));
        } else {
            entries.push(model::Entry::Comment(build_comment(record)));
        }
    }
    Ok(entries)
}

pub fn parse<I, E>(mut iter: I) -> Result<(String, model::Database)>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    check_first_header(util::next_record(&mut iter)?)?;
    let component = parse_second_header(util::next_record(&mut iter)?)?;
    check_third_header(util::next_record(&mut iter)?)?;
    let entries = parse_body(&mut iter)?;
    Ok((component, model::Database { entries }))
}

pub fn parse_csv<R: Read>(rdr: R) -> Result<(String, model::Database)> {
    let mut csv = crate::csv_reader_builder().from_reader(rdr);
    let mut iter = csv.records();
    parse(&mut iter)
}

#[derive(Debug, Deserialize)]
struct Line {
    _comment_mark: String,
    command_name: String,
    target: String,
    #[serde(deserialize_with = "deserialize_hex_with_0x")]
    code: u16,
    num_params: usize,
    param1_type: Option<model::DataType>,
    param1_description: String,
    param2_type: Option<model::DataType>,
    param2_description: String,
    param3_type: Option<model::DataType>,
    param3_description: String,
    param4_type: Option<model::DataType>,
    param4_description: String,
    param5_type: Option<model::DataType>,
    param5_description: String,
    param6_type: Option<model::DataType>,
    param6_description: String,
    danger_flag: Option<DangerFlag>,
    is_restricted: Option<IsRestricted>,
    description: String,
    note: String,
}

impl TryFrom<Line> for model::Command {
    type Error = anyhow::Error;

    fn try_from(line: Line) -> Result<Self, Self::Error> {
        let mut parameters = vec![];
        ensure!(
            line.num_params <= 6,
            "Num Params must be less than or equal to 6"
        );
        if line.num_params >= 1 {
            let Some(data_type) = line.param1_type else {
                return Err(anyhow!("Param1 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param1_description),
            });
        }
        if line.num_params >= 2 {
            let Some(data_type) = line.param2_type else {
                return Err(anyhow!("Param2 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param2_description),
            });
        }
        if line.num_params >= 3 {
            let Some(data_type) = line.param3_type else {
                return Err(anyhow!("Param3 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param3_description),
            });
        }
        if line.num_params >= 4 {
            let Some(data_type) = line.param4_type else {
                return Err(anyhow!("Param4 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param4_description),
            });
        }
        if line.num_params >= 5 {
            let Some(data_type) = line.param5_type else {
                return Err(anyhow!("Param5 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param5_description),
            });
        }
        if line.num_params >= 6 {
            let Some(data_type) = line.param6_type else {
                return Err(anyhow!("Param6 Type is missing"));
            };
            parameters.push(model::Parameter {
                data_type,
                description: unescape(&line.param6_description),
            });
        }
        Ok(model::Command {
            name: unescape(&line.command_name),
            target: unescape(&line.target),
            code: line.code,
            parameters,
            is_danger: line.danger_flag.is_some(),
            is_restricted: line.is_restricted.is_some(),
            description: unescape(&line.description),
            note: unescape(&line.note),
        })
    }
}

#[derive(Debug, Deserialize)]
enum DangerFlag {
    #[serde(rename = "danger")]
    Danger,
}

#[derive(Debug, Deserialize)]
enum IsRestricted {
    #[serde(rename = "restricted")]
    Restricted,
}

fn deserialize_hex_with_0x<'de, D>(de: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_str(HexVisitor)
}

struct HexVisitor;
impl<'de> Visitor<'de> for HexVisitor {
    type Value = u16;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a hexadecimal number string prefixed with 0x")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Some(hex) = v.strip_prefix("0x") else {
            return Err(E::custom(anyhow!("must be prefixed with 0x")));
        };
        let value = u16::from_str_radix(hex, 16).map_err(E::custom)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let csv = include_bytes!("../fixtures/CMD_DB/valid.csv");
        let json = include_bytes!("../fixtures/CMD_DB/valid.json");
        let expected: model::Database = serde_json::from_slice(json).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv.as_slice());
        let mut iter = rdr.records();
        let (_component, actual) = parse(&mut iter).unwrap();
        assert_eq!(expected, actual);
    }
}

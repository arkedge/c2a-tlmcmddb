use std::collections::{btree_map, BTreeMap};

use anyhow::{anyhow, ensure, Context, Result};
use csv::StringRecord;
use serde::Deserialize;
use tlmcmddb::tlm::{self as model};

use super::version::Version;
use crate::{escape::unescape, macros::check_header, util};

/*
+-------+--------+---------------+---------------------------------------------+------------------------------------------------------+--------------------------+--------+-------+
|       | Entry  |   OSW Info.   |              Extraction Info.               |                   Conversion Info.                   | Display Info. (since v3) |        |       |
|       |--------+-------+-------+-------+-------------------------------------+--------+-----------------------------------+---------+--------+-------+---------+        |       |
| Comm  | Name   | Var.  | Expr  | Ext.  |           Pos. Designator           | Conv.  |                 Poly              | Status  | Label  | Unit  | Format  | Descr  | Note  |
|       |        | Type  |       | Type  |-------------+-----------+-----------+ Type   +-----+-----+-----+-----+-----+-----+         |        |       |         |        |       |
|       |        |       |       |       | Octet Pos.  | bit Pos.  | bit Len.  |        | a0  | a1  | a2  | a3  | a4  | a5  |         |        |       |         |        |       |
+-------+--------+-------+-------+-------+-------------+-----------+-----------+--------+-----+-----+-----+-----+-----+-----+---------+--------+-------+---------+--------+-------+
*/

mod header {
    pub const COMMENT: &str = "Comment";
    pub const TLM_ENTRY: &str = "TLM Entry";
    pub const TLM_FIELD: &str = "TLM Field";
    pub const ONBOARD_SOFTWARE_INFO: &str = "Onboard Software Info.";
    pub const EXTRACTION_INFO: &str = "Extraction Info.";
    pub const CONVERSION_INFO: &str = "Conversion Info.";
    pub const DISPLAY_INFO: &str = "Display Info.";
    pub const DESCRIPTION: &str = "Description";
    pub const NOTE: &str = "Note";
    pub const NAME: &str = "Name";
    pub const VAR_TYPE: &str = "Var.%%##Type";
    pub const VARIABLE_OR_FUNCTION_NAME: &str = "Variable or Function Name";
    pub const EXT_TYPE: &str = "Ext.%%##Type";
    pub const POS_DESIGNATOR_OLD_TYPO: &str = "Pos. Desiginator";
    pub const POS_DESIGNATOR: &str = "Pos. Designator";
    pub const CONV_TYPE: &str = "Conv.%%##Type";
    pub const POLY: &str = "Poly (Σa_i * x^i)";
    pub const STATUS: &str = "Status";
    pub const LABEL: &str = "Label";
    pub const UNIT: &str = "Unit";
    pub const FORMAT: &str = "Format";
    pub const OCTET_POS: &str = "Octet%%##Pos.";
    pub const BIT_POS: &str = "bit%%##Pos.";
    pub const BIT_LEN: &str = "bit%%##Len.";
    pub const A0: &str = "a0";
    pub const A1: &str = "a1";
    pub const A2: &str = "a2";
    pub const A3: &str = "a3";
    pub const A4: &str = "a4";
    pub const A5: &str = "a5";
}

struct HeaderScanner {
    record: StringRecord,
    position: usize,
}

impl HeaderScanner {
    fn new(record: StringRecord) -> Self {
        HeaderScanner {
            record,
            position: 0,
        }
    }

    fn skip(&mut self, n: usize) {
        self.position += n;
    }

    fn check(&mut self, expected: &str) -> Result<()> {
        let actual = self
            .record
            .get(self.position)
            .ok_or_else(|| anyhow!("the number of columns is mismatch"))?;
        check_header!(actual, expected);
        self.position += 1;
        Ok(())
    }
}

fn check_first_header(record: StringRecord, version: Version) -> Result<()> {
    let mut scanner = HeaderScanner::new(record);
    scanner.check(header::COMMENT)?;
    if version.major <= 2 {
        scanner.check(header::TLM_ENTRY)?;
    } else {
        scanner.check(header::TLM_FIELD)?;
    }
    scanner.check(header::ONBOARD_SOFTWARE_INFO)?;
    scanner.skip(1);
    scanner.check(header::EXTRACTION_INFO)?;
    scanner.skip(3);
    scanner.check(header::CONVERSION_INFO)?;
    scanner.skip(7);
    if version.major >= 3 {
        scanner.check(header::DISPLAY_INFO)?;
        scanner.skip(2);
    }
    scanner.check(header::DESCRIPTION)?;
    scanner.check(header::NOTE)?;
    Ok(())
}

fn check_second_header(record: StringRecord, version: Version) -> Result<()> {
    ensure!(record.len() >= 16, "the number of columns is mismatch");

    let mut scanner = HeaderScanner::new(record);
    scanner.skip(1);
    scanner.check(header::NAME)?;
    scanner.skip(1); // header::VAR_TYPE
    scanner.check(header::VARIABLE_OR_FUNCTION_NAME)?;
    scanner.skip(1); // header::EXT_TYPE
    if version.major <= 2 {
        scanner.check(header::POS_DESIGNATOR_OLD_TYPO)?;
    } else {
        scanner.check(header::POS_DESIGNATOR)?;
    }
    scanner.skip(2);
    scanner.skip(1); // header::CONV_TYPE
    scanner.check(header::POLY)?;
    scanner.skip(5);
    scanner.check(header::STATUS)?;
    if version.major >= 3 {
        scanner.check(header::LABEL)?;
        scanner.check(header::UNIT)?;
        scanner.check(header::FORMAT)?;
    }
    Ok(())
}

fn check_third_header(record: StringRecord) -> Result<()> {
    ensure!(record.len() >= 15, "the number of columns is mismatch");
    //check_header!(&record[5], header::OCTET_POS);
    //check_header!(&record[6], header::BIT_POS);
    //check_header!(&record[7], header::BIT_LEN);
    check_header!(&record[9], header::A0);
    check_header!(&record[10], header::A1);
    check_header!(&record[11], header::A2);
    check_header!(&record[12], header::A3);
    check_header!(&record[13], header::A4);
    check_header!(&record[14], header::A5);
    Ok(())
}

fn check_headers<I, E>(mut iter: I, version: Version) -> Result<()>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    check_first_header(util::next_record(&mut iter)?, version)?;
    check_second_header(util::next_record(&mut iter)?, version)?;
    check_third_header(util::next_record(&mut iter)?)?;
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

fn parse_entries<I, E, Line>(mut iter: I) -> Result<Vec<model::Entry>>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
    Line: serde::de::DeserializeOwned + TryInto<LineModel, Error = anyhow::Error>,
{
    let mut entries = vec![];
    let mut current_bit_field_group = None;
    while let Some(record) = util::try_next_record(&mut iter)? {
        if record[0].is_empty() {
            let line = record.deserialize::<Line>(None)?;
            match line.try_into()? {
                LineModel::BitFieldGroup(bit_field_group) => {
                    if let Some(bit_field_group) = current_bit_field_group.take() {
                        entries.push(model::Entry::FieldGroup(bit_field_group));
                    }
                    current_bit_field_group = Some(bit_field_group);
                }
                LineModel::BitField(bit_field) => {
                    if let Some(bit_field_group) = current_bit_field_group.as_mut() {
                        bit_field_group
                            .sub_entries
                            .push(model::SubEntry::Field(bit_field));
                    } else {
                        return Err(anyhow!("unexpected bit field"));
                    }
                }
            }
        } else {
            let comment = build_comment(record);
            if let Some(bit_field_group) = current_bit_field_group.as_mut() {
                bit_field_group
                    .sub_entries
                    .push(model::SubEntry::Comment(comment));
            } else {
                entries.push(model::Entry::Comment(comment));
            }
        }
    }
    if let Some(bit_field_group) = current_bit_field_group.take() {
        entries.push(model::Entry::FieldGroup(bit_field_group));
    }
    Ok(entries)
}

pub fn parse<I, E>(mut iter: I, version: Version) -> Result<Vec<model::Entry>>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    check_headers(&mut iter, version)?;
    if version.major <= 2 {
        parse_entries::<_, _, LineV2>(&mut iter)
    } else {
        parse_entries::<_, _, LineV3>(&mut iter)
    }
}

struct LineConversionInfo {
    conversion_type: ConversionType,
    a0: Option<f64>,
    a1: Option<f64>,
    a2: Option<f64>,
    a3: Option<f64>,
    a4: Option<f64>,
    a5: Option<f64>,
    status: Option<String>,
}

struct LineDisplayInfo {
    label: Option<String>,
    unit: Option<String>,
    format: Option<String>,
}

impl TryFrom<LineConversionInfo> for model::ConversionInfo {
    type Error = anyhow::Error;

    fn try_from(info: LineConversionInfo) -> Result<Self, Self::Error> {
        match info.conversion_type {
            ConversionType::None => {
                ensure!(
                    info.a0.is_none(),
                    "a0 must be empty when Conv. Type is NONE"
                );
                ensure!(
                    info.a1.is_none(),
                    "a1 must be empty when Conv. Type is NONE"
                );
                ensure!(
                    info.a2.is_none(),
                    "a2 must be empty when Conv. Type is NONE"
                );
                ensure!(
                    info.a3.is_none(),
                    "a3 must be empty when Conv. Type is NONE"
                );
                ensure!(
                    info.a4.is_none(),
                    "a4 must be empty when Conv. Type is NONE"
                );
                ensure!(
                    info.a5.is_none(),
                    "a5 must be empty when Conv. Type is NONE"
                );
                /*
                ensure!(
                    info.status.is_none(),
                    "Status must be empty when Conv. Type is NONE"
                );
                */
                Ok(model::ConversionInfo::None)
            }
            ConversionType::Hex => {
                ensure!(info.a0.is_none(), "a0 must be empty when Conv. Type is HEX");
                ensure!(info.a1.is_none(), "a1 must be empty when Conv. Type is HEX");
                ensure!(info.a2.is_none(), "a2 must be empty when Conv. Type is HEX");
                ensure!(info.a3.is_none(), "a3 must be empty when Conv. Type is HEX");
                ensure!(info.a4.is_none(), "a4 must be empty when Conv. Type is HEX");
                ensure!(info.a5.is_none(), "a5 must be empty when Conv. Type is HEX");
                ensure!(
                    info.status.is_none(),
                    "Status must be empty when Conv. Type is HEX"
                );
                Ok(model::ConversionInfo::Hex)
            }
            ConversionType::Status => {
                ensure!(
                    info.a0.is_none(),
                    "a0 must be empty when Conv. Type is STATUS"
                );
                ensure!(
                    info.a1.is_none(),
                    "a1 must be empty when Conv. Type is STATUS"
                );
                ensure!(
                    info.a2.is_none(),
                    "a2 must be empty when Conv. Type is STATUS"
                );
                ensure!(
                    info.a3.is_none(),
                    "a3 must be empty when Conv. Type is STATUS"
                );
                ensure!(
                    info.a4.is_none(),
                    "a4 must be empty when Conv. Type is STATUS"
                );
                ensure!(
                    info.a5.is_none(),
                    "a5 must be empty when Conv. Type is STATUS"
                );
                let Some(status) = info.status else {
                    return Err(anyhow!("Conv. Type is STATUS but Status is missing"));
                };
                let status = parse_status_map(&unescape(&status))?;
                Ok(model::ConversionInfo::Status(status))
            }
            ConversionType::Poly => {
                ensure!(
                    info.status.is_none(),
                    "Status must be empty when Conv. Type is POLY"
                );
                let polynomial = model::conversion::Polynomial {
                    a0: info.a0.unwrap_or_default(),
                    a1: info.a1.unwrap_or_default(),
                    a2: info.a2.unwrap_or_default(),
                    a3: info.a3.unwrap_or_default(),
                    a4: info.a4.unwrap_or_default(),
                    a5: info.a5.unwrap_or_default(),
                };
                Ok(model::ConversionInfo::Polynomial(polynomial))
            }
        }
    }
}

impl From<LineDisplayInfo> for model::DisplayInfo {
    fn from(info: LineDisplayInfo) -> Self {
        Self {
            label: info.label,
            unit: info.unit,
            format: info.format,
        }
    }
}

fn parse_status_map(s: &str) -> Result<model::conversion::Status> {
    let mut default_value = None;
    let mut map = BTreeMap::new();
    let pairs = s.split(',');
    for pair in pairs {
        let (key_str, value) = pair
            .split_once('=')
            .ok_or_else(|| anyhow!("malformed status mapping rule"))?;
        let key_str = key_str.trim();
        let value = value.trim();
        if key_str == "*" {
            if default_value.is_some() {
                return Err(anyhow!(
                    "invalid status mapping rule: default value is defined twice"
                ));
            }
            default_value = Some(value.to_string());
        } else {
            let key = if let Some(hex) = key_str.strip_prefix("0x") {
                i64::from_str_radix(hex, 16).context("invalid status mapping rule: key must be a signed decimal integer or 0x-prefixed HEX")?
            } else {
                key_str
                .parse::<i64>()
                .context("invalid status mapping rule: key must be a signed decimal integer or 0x-prefixed HEX")?
            };
            match map.entry(key) {
                btree_map::Entry::Occupied(_) => {
                    return Err(anyhow!(
                        "invalid status mapping rule: rule for key {} is defined twice",
                        key
                    ))
                }
                btree_map::Entry::Vacant(vacant) => {
                    vacant.insert(value.to_string());
                }
            }
        }
    }
    let variants = map
        .into_iter()
        .map(|(key, value)| model::conversion::Variant { key, value })
        .collect();
    Ok(model::conversion::Status {
        variants,
        default_value,
    })
}

enum LineModel {
    BitFieldGroup(model::FieldGroup),
    BitField(model::Field),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
enum ConversionType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "HEX")]
    Hex,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "POLY")]
    Poly,
}

#[derive(Debug, Deserialize)]
pub struct LineV2 {
    _comment_mark: String,
    field_name: String,
    variable_type: Option<model::VariableType>,
    expression: Option<String>,
    extraction_type: String,
    octet_position: usize,
    bit_position: usize,
    bit_length: usize,
    conversion_type: ConversionType,
    a0: Option<f64>,
    a1: Option<f64>,
    a2: Option<f64>,
    a3: Option<f64>,
    a4: Option<f64>,
    a5: Option<f64>,
    status: Option<String>,
    description: String,
    note: String,
}

impl From<LineV2> for LineV3 {
    fn from(line: LineV2) -> Self {
        Self {
            _comment_mark: line._comment_mark,
            field_name: line.field_name,
            variable_type: line.variable_type,
            expression: line.expression,
            extraction_type: line.extraction_type,
            octet_position: line.octet_position,
            bit_position: line.bit_position,
            bit_length: line.bit_length,
            conversion_type: line.conversion_type,
            a0: line.a0,
            a1: line.a1,
            a2: line.a2,
            a3: line.a3,
            a4: line.a4,
            a5: line.a5,
            label: None,
            unit: None,
            format: None,
            status: line.status,
            description: line.description,
            note: line.note,
        }
    }
}

impl TryFrom<LineV2> for LineModel {
    type Error = anyhow::Error;

    fn try_from(line: LineV2) -> Result<Self, Self::Error> {
        let line: LineV3 = line.into();
        line.try_into()
    }
}

#[derive(Debug, Deserialize)]
struct LineV3 {
    _comment_mark: String,
    field_name: String,
    variable_type: Option<model::VariableType>,
    expression: Option<String>,
    extraction_type: String,
    octet_position: usize,
    bit_position: usize,
    bit_length: usize,
    conversion_type: ConversionType,
    a0: Option<f64>,
    a1: Option<f64>,
    a2: Option<f64>,
    a3: Option<f64>,
    a4: Option<f64>,
    a5: Option<f64>,
    status: Option<String>,
    label: Option<String>,
    unit: Option<String>,
    format: Option<String>,
    description: String,
    note: String,
}

impl LineV3 {
    fn take_conversion_info(&mut self) -> LineConversionInfo {
        LineConversionInfo {
            conversion_type: self.conversion_type,
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            a5: self.a5,
            status: self.status.take(),
        }
    }

    fn take_display_info(&mut self) -> LineDisplayInfo {
        LineDisplayInfo {
            label: self.label.take(),
            unit: self.unit.take(),
            format: self.unit.take(),
        }
    }
}

impl TryFrom<LineV3> for LineModel {
    type Error = anyhow::Error;

    fn try_from(line: LineV3) -> Result<Self, Self::Error> {
        if line.variable_type.is_some() {
            Ok(Self::BitFieldGroup(line.try_into()?))
        } else {
            Ok(Self::BitField(line.try_into()?))
        }
    }
}

impl TryFrom<LineV3> for model::FieldGroup {
    type Error = anyhow::Error;

    fn try_from(mut line: LineV3) -> Result<Self, Self::Error> {
        let Some(variable_type) = line.variable_type.take() else {
            return Err(anyhow!("Var. Type is missing"));
        };
        let expression = unescape(&line.expression.take().unwrap_or_default());
        let onboard_software_info = model::OnboardSoftwareInfo {
            variable_type,
            expression,
        };
        let bit_field = line.try_into()?;
        Ok(Self {
            onboard_software_info,
            sub_entries: vec![model::SubEntry::Field(bit_field)],
        })
    }
}

impl TryFrom<LineV3> for model::Field {
    type Error = anyhow::Error;

    fn try_from(mut line: LineV3) -> Result<Self, Self::Error> {
        if line.variable_type.is_some() {
            return Err(anyhow!("Var. Type is present"));
        };
        if line.expression.is_some() {
            return Err(anyhow!("Variable or Function Name is present"));
        };
        let extraction_info = model::FieldExtractionInfo {
            extraction_type: unescape(&line.extraction_type),
            octet_position: line.octet_position,
            bit_position: line.bit_position,
            bit_length: line.bit_length,
        };
        let conversion_info = line.take_conversion_info();
        let display_info = line.take_display_info();
        Ok(Self {
            name: unescape(&line.field_name),
            extraction_info,
            conversion_info: conversion_info.try_into()?,
            display_info: display_info.into(),
            description: unescape(&line.description),
            display_info: Default::default(),
            note: unescape(&line.note),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let csv = include_bytes!("../../fixtures/TLM_DB/valid_body.csv");
        let json = include_bytes!("../../fixtures/TLM_DB/valid_body.json");
        let expected: Vec<model::Entry> = serde_json::from_slice(json).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv.as_slice());
        let mut iter = rdr.records();
        let actual = parse(&mut iter, Version { major: 2 }).unwrap();
        assert_eq!(expected, actual)

        // make snapshot:
        // serde_json::to_writer_pretty(std::fs::OpenOptions::new().write(true).truncate(true).open("fixtures/TLM_DB/valid_body.json").unwrap(), &actual).unwrap();
    }
}

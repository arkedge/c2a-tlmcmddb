use anyhow::{anyhow, ensure, Context, Result};
use csv::StringRecord;
use tlmcmddb::tlm as model;

use crate::{escape::unescape, macros::check_header, util, PosStringRecord};

/*
+---+-----------------+---------+-------------------+
|   | Target          | OBC     | Local Var         |
+---+-----------------+---------+-------------------+
|   | PacketID        | 0xf0    | int offset = ...  |
+---+-----------------+---------+-------------------+
|   | Enable/Disable  | ENABLE  |                   |
+---+-----------------+---------+-------------------+
|   | IsRestricted    | FALSE   |                   |
+---+-----------------+---------+-------------------+
*/

mod header {
    pub const TARGET: &str = "Target";
    pub const PACKET_ID: &str = "PacketID";
    pub const ENABLE_DISABLE: &str = "Enable/Disable";
    pub const IS_RESTRICTED: &str = "IsRestricted";
    pub const LOCAL_VAR: &str = "Local Var";
}

fn parse_first_line(record: StringRecord) -> Result<String> {
    ensure!(record.len() >= 4, "the number of columns is mismatch");
    check_header!(&record[1], header::TARGET);
    check_header!(&record[3], header::LOCAL_VAR);
    let target = &record[2];
    Ok(unescape(target))
}

fn parse_packet_id(hex_with_0x: &str) -> Result<u8> {
    let Some(hex) = hex_with_0x.strip_prefix("0x") else {
        return Err(anyhow!("the value of PacketID must start with 0x"));
    };
    u8::from_str_radix(hex, 16).context("parsing PacketID")
}

fn parse_second_line(record: StringRecord) -> Result<(u8, String)> {
    ensure!(record.len() >= 4, "the number of columns is mismatch");
    check_header!(&record[1], header::PACKET_ID);
    let packet_id_hex_with_0x = &record[2];
    let packet_id = parse_packet_id(packet_id_hex_with_0x)?;
    let local_var = &record[3];
    Ok((packet_id, unescape(local_var)))
}

fn parse_third_line(record: StringRecord) -> Result<bool> {
    ensure!(record.len() >= 3, "the number of columns is mismatch");
    check_header!(&record[1], header::ENABLE_DISABLE);
    let is_enabled_str = &record[2];
    let is_enabled = match is_enabled_str {
        "ENABLE" => true,
        "DISABLE" => false,
        _ => {
            return Err(anyhow!(
                "the value of Enable/Disable must be either ENABLE or DISABLE"
            ))
        }
    };
    Ok(is_enabled)
}

fn parse_fourth_line(record: StringRecord) -> Result<bool> {
    ensure!(record.len() >= 3, "the number of columns is mismatch");
    check_header!(&record[1], header::IS_RESTRICTED);
    let is_restricted_str = &record[2];
    let is_restricted = match is_restricted_str {
        "TRUE" => true,
        "FALSE" => false,
        _ => {
            return Err(anyhow!(
                "the value of IsRestricted must be either TRUE or FALSE"
            ))
        }
    };
    Ok(is_restricted)
}

pub fn parse<I, E>(mut iter: I) -> Result<model::Metadata>
where
    I: Iterator<Item = Result<PosStringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let target = parse_first_line(util::next_record(&mut iter)?.record)?;
    let (packet_id, local_variables) = parse_second_line(util::next_record(&mut iter)?.record)?;
    let is_enabled = parse_third_line(util::next_record(&mut iter)?.record)?;
    let is_restricted = parse_fourth_line(util::next_record(&mut iter)?.record)?;
    let _padding_line = util::next_record(&mut iter)?;
    Ok(model::Metadata {
        target,
        packet_id,
        is_enabled,
        is_restricted,
        local_variables,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let csv = include_bytes!("../../fixtures/TLM_DB/valid_metadata.csv");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv.as_slice());
        let mut iter = rdr.records();
        let metadata = parse(&mut iter).unwrap();
        assert_eq!("OBC", metadata.target);
        assert_eq!(0xf0, metadata.packet_id);
        assert!(metadata.is_enabled);
        assert!(!metadata.is_restricted);
        assert!(iter.next().is_none());
    }
}

pub mod body;
mod filename;
pub mod metadata;
pub mod telemetry;

pub use filename::Filename;

use crate::PosStringRecordIterator;
use anyhow::Result;
use std::io::Read;

pub fn parse_csv<R: Read>(telemetry_name: String, rdr: R) -> Result<tlmcmddb::tlm::Telemetry> {
    let csv = crate::csv_reader_builder().from_reader(rdr);
    let mut iter = PosStringRecordIterator::from_reader(csv);
    telemetry::parse(telemetry_name, &mut iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    fn parse_testdata() -> Result<tlmcmddb::tlm::Telemetry> {
        let csv = {
            let header = include_bytes!("../fixtures/TLM_DB/valid_metadata.csv");
            let body = include_bytes!("../fixtures/TLM_DB/valid_body.csv");

            let mut csv = header.to_vec();
            csv.extend_from_slice(body);
            Cursor::new(csv)
        };

        parse_csv("".to_string(), csv)
    }

    #[test]
    fn test_read_csv() {
        let expected: tlmcmddb::tlm::Telemetry = {
            let json = include_bytes!("../fixtures/TLM_DB/valid.json");
            serde_json::from_slice(json).unwrap()
        };

        let actual = parse_testdata().unwrap();

        assert_eq!(expected, actual)

        // make snapshot
        // serde_json::to_writer_pretty(
        //     std::fs::OpenOptions::new()
        //         .create(true)
        //         .write(true)
        //         .truncate(true)
        //         .open("fixtures/TLM_DB/valid.json")
        //         .unwrap(),
        //     &actual,
        // );
    }

    #[test]
    fn test_ser_json() {
        let expected = include_str!("../fixtures/TLM_DB/valid.json");

        let tlm = parse_testdata().unwrap();
        let actual = serde_json::to_string_pretty(&tlm).unwrap();

        assert_eq!(expected, actual)
    }
}

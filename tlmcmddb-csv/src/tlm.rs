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

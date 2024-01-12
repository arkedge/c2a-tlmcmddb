pub mod body;
mod filename;
pub mod metadata;
pub mod telemetry;
mod version;

pub use filename::Filename;

use anyhow::Result;
use std::io::Read;

pub fn parse_csv<R: Read>(telemetry_name: String, rdr: R) -> Result<tlmcmddb::tlm::Telemetry> {
    let mut csv = crate::csv_reader_builder().from_reader(rdr);
    let (version, mut iter) = version::guess(csv.records())?;

    telemetry::parse(telemetry_name, &mut iter, version)
}

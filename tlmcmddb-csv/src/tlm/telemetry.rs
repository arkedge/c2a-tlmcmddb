use anyhow::{ensure, Result};
use csv::StringRecord;
use tlmcmddb::tlm as model;

use super::{body, metadata, version::Version};

pub fn parse<I, E>(
    telemetry_name: String,
    mut iter: I,
    version: Version,
) -> Result<model::Telemetry>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    ensure!(2 <= version.major && version.major <= 3, "unknown version");
    let metadata = metadata::parse(&mut iter)?;
    let entries = body::parse(&mut iter, version)?;
    Ok(model::Telemetry {
        name: telemetry_name,
        metadata,
        entries,
    })
}

use anyhow::Result;
use tlmcmddb::tlm as model;

use super::{body, metadata};
use crate::PosStringRecord;

pub fn parse<I, E>(telemetry_name: String, mut iter: I) -> Result<model::Telemetry>
where
    I: Iterator<Item = Result<PosStringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let metadata = metadata::parse(&mut iter)?;
    let entries = body::parse(&mut iter)?;
    Ok(model::Telemetry {
        name: telemetry_name,
        metadata,
        content: model::Content::Struct(entries),
    })
}

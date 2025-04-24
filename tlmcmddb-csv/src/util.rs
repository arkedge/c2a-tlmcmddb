use crate::PosStringRecord;
use anyhow::{anyhow, Result};

pub fn next_record<I, E>(iter: &mut I) -> Result<PosStringRecord>
where
    I: Iterator<Item = Result<PosStringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(iter
        .next()
        .ok_or_else(|| anyhow!("unexpected end of data"))??)
}

pub fn try_next_record<I, E>(iter: &mut I) -> Result<Option<PosStringRecord>>
where
    I: Iterator<Item = Result<PosStringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let Some(record) = iter.next().transpose()? else {
        return Ok(None);
    };
    let is_empty = record.record.is_empty() || record.record.iter().all(|col| col.is_empty());
    if is_empty {
        return Ok(None);
    }
    Ok(Some(record))
}

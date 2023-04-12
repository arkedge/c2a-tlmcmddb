use anyhow::{anyhow, Result};
use csv::StringRecord;

pub fn next_record<I, E>(iter: &mut I) -> Result<StringRecord>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(iter
        .next()
        .ok_or_else(|| anyhow!("unexpected end of data"))??)
}

pub fn try_next_record<I, E>(iter: &mut I) -> Result<Option<StringRecord>>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let Some(record) = iter.next().transpose()? else {
        return Ok(None);
    };
    let is_empty = record.is_empty() || record.iter().all(|col| col.is_empty());
    if is_empty {
        return Ok(None);
    }
    Ok(Some(record))
}

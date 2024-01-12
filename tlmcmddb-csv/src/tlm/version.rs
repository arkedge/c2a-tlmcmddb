use anyhow::{anyhow, Result};
use csv::StringRecord;

#[derive(Clone, Copy)]
pub struct Version {
    pub major: u64,
}

pub fn guess<I, E>(mut iter: I) -> Result<(Version, impl Iterator<Item = Result<StringRecord, E>>)>
where
    I: Iterator<Item = Result<StringRecord, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    // guess by the 6th line

    let metadata_lines = {
        let mut metadata_lines = vec![];
        for _ in 0..5 {
            let line = iter
                .next()
                .ok_or_else(|| anyhow!("unexpected end of data"))?;
            metadata_lines.push(line)
        }
        metadata_lines.into_iter()
    };

    let body_header_line = iter
        .next()
        .ok_or_else(|| anyhow!("unexpected end of data"))??;

    let version = if body_header_line.iter().any(|s| s == "Display Info.") {
        Version { major: 3 }
    } else {
        Version { major: 2 }
    };

    Ok((
        version,
        metadata_lines
            .chain(std::iter::once(Ok(body_header_line)))
            .chain(iter),
    ))
}

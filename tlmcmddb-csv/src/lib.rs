mod macros;
mod util;

pub mod cmd;
pub mod escape;
pub mod tlm;

pub struct PosStringRecord {
    pub position: csv::Position,
    pub record: csv::StringRecord,
}

struct PosStringRecordIterator<R>(pub csv::StringRecordsIntoIter<R>);

impl<R> PosStringRecordIterator<R>
where
    R: std::io::Read,
{
    fn from_reader(reader: csv::Reader<R>) -> PosStringRecordIterator<R> {
        PosStringRecordIterator(reader.into_records())
    }
}

impl<'r, R: 'r> Iterator for PosStringRecordIterator<R>
where
    R: std::io::Read,
{
    type Item = Result<PosStringRecord, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.0.reader().position().clone();
        let record = self.0.next()?;
        Some(record.map(|record| PosStringRecord { position, record }))
    }
}

pub fn csv_reader_builder() -> csv::ReaderBuilder {
    let mut builder = csv::ReaderBuilder::new();
    builder.has_headers(false);
    builder
}

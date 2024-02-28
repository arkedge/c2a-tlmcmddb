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

#[derive(Debug)]
struct PositionError<E> {
    position: csv::Position,
    error: E,
}

impl<E> std::fmt::Display for PositionError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error at line {}:  {}",
            self.position.line() + 1,
            self.error
        )
    }
}

impl<E> std::error::Error for PositionError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

trait ErrWithPosition {
    type T;
    type E;
    fn err_with_position(self, position: &csv::Position) -> Result<Self::T, PositionError<Self::E>>
    where
        Self: Sized;
}

impl<T, E> ErrWithPosition for Result<T, E> {
    type T = T;
    type E = E;
    fn err_with_position(self, position: &csv::Position) -> Result<T, PositionError<E>> {
        self.map_err(|error| PositionError {
            position: position.clone(),
            error,
        })
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

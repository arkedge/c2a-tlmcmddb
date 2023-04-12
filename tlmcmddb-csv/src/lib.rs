mod macros;
mod util;

pub mod cmd;
pub mod escape;
pub mod tlm;

pub fn csv_reader_builder() -> csv::ReaderBuilder {
    let mut builder = csv::ReaderBuilder::new();
    builder.has_headers(false);
    builder
}

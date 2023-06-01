use arrow::compute::concat_batches;
use arrow::ipc::reader::FileReader;
use color_eyre::eyre;
use std::fs::{read_dir, File};
use std::path::PathBuf;

#[test]
fn concat_test() -> eyre::Result<()> {
    let mut records = Vec::new();
    let mut maybe_schema = None;

    let mut record_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    record_path.push("tests/records");

    for entry in read_dir(record_path)? {
        let file = File::open(entry?.path())?;
        let reader = FileReader::try_new(file, None)?;
        for result in reader {
            let record = result?;

            if maybe_schema.is_none() {
                maybe_schema = Some(record.schema());
            }

            records.push(record);
        }
    }

    if let Some(schema) = maybe_schema {
        concat_batches(&schema, &records)?;
    }

    Ok(())
}

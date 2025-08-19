use nalgebra::DMatrix;
use std::error::Error;
use std::result::Result;

pub fn read_csv(file_path: &str) -> Result<DMatrix<f64>, Box<dyn Error>> {
    println!("Reading CSV file: {}", file_path);

    let mut rdr = csv::Reader::from_path(file_path)?;

    let records = rdr
        .records()
        .map(|result| -> Result<Vec<f64>, Box<dyn Error>> {
            let record = result?;
            let row = record
                .iter()
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<f64>, _>>()?;
            Ok(row)
        })
        .collect::<Result<Vec<Vec<f64>>, _>>()?;

    if records.is_empty() {
        return Ok(DMatrix::from_row_slice(0, 0, &[]));
    }

    let rows = records.len();
    let cols = records[0].len();
    let matrix = DMatrix::from_vec(rows, cols, records.concat());
    Ok(matrix)
}

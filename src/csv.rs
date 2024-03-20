use anyhow::Result;
use csv_core::{ReadFieldResult, Reader};

pub struct Csv;

impl Csv {
    pub fn parse(
        mut bytes: &[u8],
        starting_col: Option<usize>,
        starting_row: Option<usize>,
        cols: Option<usize>,
        rows: Option<usize>,
    ) -> Result<Vec<Vec<String>>> {
        let mut rdr = Reader::new();
        let mut row_idx = 0;
        let output = &mut [0; 1024];

        let mut table = Vec::new();
        let mut row: Vec<String> = Vec::new();
        let mut cell = String::new();

        let starting_col = starting_col.unwrap_or_default();
        let starting_row = starting_row.unwrap_or_default();

        loop {
            let (result, nin, nout) = rdr.read_field(bytes, output);
            bytes = &bytes[nin..];
            let read = &output[..nout];

            match result {
                ReadFieldResult::InputEmpty => {
                    break;
                }
                ReadFieldResult::OutputFull => {
                    cell.push_str(std::str::from_utf8(read)?);
                }
                ReadFieldResult::Field { record_end } => {
                    cell.push_str(std::str::from_utf8(read)?);
                    row.push(cell);
                    cell = String::new();

                    if record_end {
                        if let Some(rows) = rows {
                            if row_idx >= starting_row + rows {
                                break;
                            }
                        }

                        if row_idx >= starting_row {
                            if let Some(cols) = cols {
                                table.push(row[starting_col..starting_col + cols].to_vec());
                            } else {
                                table.push(row[starting_col..].to_vec());
                            }
                        }

                        row_idx += 1;
                        row = Vec::new();
                    }
                }
                ReadFieldResult::End => {
                    break;
                }
            }
        }

        Ok(table)
    }
}

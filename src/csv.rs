use anyhow::{Context, Result};
use csv_core::{ReadFieldResult, Reader};

/// Maximum size for the output buffer when reading CSV fields
const OUTPUT_BUFFER_SIZE: usize = 1024;

/// A CSV parser that provides functionality to read and parse CSV data with optional
/// row and column filtering.
pub struct Csv;

impl Csv {
    /// Parses CSV data with optional row and column filtering.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The input CSV data as a byte slice
    /// * `starting_col` - Optional starting column index (0-based)
    /// * `starting_row` - Optional starting row index (0-based)
    /// * `cols` - Optional number of columns to include
    /// * `rows` - Optional number of rows to include
    ///
    /// # Returns
    ///
    /// Returns a Result containing a Vec of Vec of Strings representing the parsed CSV data
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * UTF-8 conversion fails
    /// * Input data is malformed
    pub fn parse(
        mut bytes: &[u8],
        starting_col: Option<usize>,
        starting_row: Option<usize>,
        cols: Option<usize>,
        rows: Option<usize>,
    ) -> Result<Vec<Vec<String>>> {
        let mut rdr = Reader::new();
        let mut row_idx = 0;
        let output = &mut [0; OUTPUT_BUFFER_SIZE];

        let mut table = Vec::new();
        let mut row = Vec::new();
        let mut cell = String::new();

        let starting_col = starting_col.unwrap_or(0);
        let starting_row = starting_row.unwrap_or(0);

        loop {
            let (result, bytes_read, bytes_written) = rdr.read_field(bytes, output);
            bytes = &bytes[bytes_read..];
            let field_data = &output[..bytes_written];

            match result {
                ReadFieldResult::InputEmpty | ReadFieldResult::End => break,

                ReadFieldResult::OutputFull => {
                    cell.push_str(
                        std::str::from_utf8(field_data)
                            .context("Failed to convert field data to UTF-8")?,
                    );
                }

                ReadFieldResult::Field { record_end } => {
                    cell.push_str(
                        std::str::from_utf8(field_data)
                            .context("Failed to convert field data to UTF-8")?,
                    );
                    row.push(cell);
                    cell = String::new();

                    if record_end {
                        if Self::should_process_row(row_idx, starting_row, rows) {
                            let filtered_row = Self::filter_row(&row, starting_col, cols);
                            table.push(filtered_row);
                        }

                        if Self::should_stop_processing(row_idx, starting_row, rows) {
                            break;
                        }

                        row_idx += 1;
                        row = Vec::new();
                    }
                }
            }
        }

        Ok(table)
    }

    /// Determines if the current row should be processed based on filtering criteria
    #[inline]
    fn should_process_row(row_idx: usize, starting_row: usize, rows: Option<usize>) -> bool {
        row_idx >= starting_row && rows.map_or(true, |r| row_idx < starting_row + r)
    }

    /// Determines if processing should stop based on row constraints
    #[inline]
    fn should_stop_processing(row_idx: usize, starting_row: usize, rows: Option<usize>) -> bool {
        rows.is_some_and(|r| row_idx >= starting_row + r)
    }

    /// Filters a row based on column constraints
    #[inline]
    fn filter_row(row: &[String], starting_col: usize, cols: Option<usize>) -> Vec<String> {
        match cols {
            Some(n) => row[starting_col..starting_col + n].to_vec(),
            None => row[starting_col..].to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_csv() {
        let input = b"a,b,c\n1,2,3\n";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
            ]
        );
    }

    #[test]
    fn test_parse_empty_csv() {
        let input = b"";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_single_cell() {
        let input = b"value";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(result, vec![vec!["value".to_string()]]);
    }

    #[test]
    fn test_parse_single_row() {
        let input = b"a,b,c";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]]
        );
    }

    #[test]
    fn test_parse_multiple_rows() {
        let input = b"a,b,c\n1,2,3\nx,y,z";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
                vec!["x".to_string(), "y".to_string(), "z".to_string()],
            ]
        );
    }

    #[test]
    fn test_column_filtering() {
        let input = b"a,b,c,d\n1,2,3,4";

        // Test starting column
        let result = Csv::parse(input, Some(1), None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["b".to_string(), "c".to_string(), "d".to_string()],
                vec!["2".to_string(), "3".to_string(), "4".to_string()],
            ]
        );

        // Test column count
        let result = Csv::parse(input, None, None, Some(2), None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["1".to_string(), "2".to_string()],
            ]
        );

        // Test both starting column and count
        let result = Csv::parse(input, Some(1), None, Some(2), None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["b".to_string(), "c".to_string()],
                vec!["2".to_string(), "3".to_string()],
            ]
        );
    }

    #[test]
    fn test_row_filtering() {
        let input = b"a,b\n1,2\n3,4\n5,6";

        // Test starting row
        let result = Csv::parse(input, None, Some(1), None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["1".to_string(), "2".to_string()],
                vec!["3".to_string(), "4".to_string()],
                vec!["5".to_string(), "6".to_string()],
            ]
        );

        // Test row count
        let result = Csv::parse(input, None, None, None, Some(2)).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["1".to_string(), "2".to_string()],
            ]
        );

        // Test both starting row and count
        let result = Csv::parse(input, None, Some(1), None, Some(2)).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["1".to_string(), "2".to_string()],
                vec!["3".to_string(), "4".to_string()],
            ]
        );
    }

    #[test]
    fn test_empty_fields() {
        let input = b"a,,c\n,b,\n,,";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "".to_string(), "c".to_string()],
                vec!["".to_string(), "b".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string()],
            ]
        );
    }

    #[test]
    fn test_quoted_fields() {
        let input = b"\"a,b\",c\n\"\"\"quoted\"\"\",\"line\nbreak\"";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a,b".to_string(), "c".to_string()],
                vec!["\"quoted\"".to_string(), "line\nbreak".to_string()],
            ]
        );
    }

    #[test]
    fn test_unicode_content() {
        let input = "α,β,γ\n🦀,🎉,🌟".as_bytes();
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["α".to_string(), "β".to_string(), "γ".to_string()],
                vec!["🦀".to_string(), "🎉".to_string(), "🌟".to_string()],
            ]
        );
    }

    #[test]
    fn test_combined_filtering() {
        let input = b"a,b,c,d\n1,2,3,4\n5,6,7,8\n9,10,11,12";
        let result = Csv::parse(input, Some(1), Some(1), Some(2), Some(2)).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["2".to_string(), "3".to_string()],
                vec!["6".to_string(), "7".to_string()],
            ]
        );
    }

    #[test]
    fn test_boundary_conditions() {
        // Empty rows
        let input = b"\n\n\n";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["".to_string()],
                vec!["".to_string()],
                vec!["".to_string()],
            ]
        );

        // Single character fields
        let input = b"a\nb\nc";
        let result = Csv::parse(input, None, None, None, None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string()],
                vec!["b".to_string()],
                vec!["c".to_string()],
            ]
        );
    }

    #[test]
    fn test_out_of_bounds_filtering() {
        let input = b"a,b\n1,2";

        // Starting column beyond available columns
        let result = Csv::parse(input, Some(5), None, None, None).unwrap();
        assert!(result.iter().all(|row| row.is_empty()));

        // Starting row beyond available rows
        let result = Csv::parse(input, None, Some(5), None, None).unwrap();
        assert!(result.is_empty());

        // Requesting more columns than available
        let result = Csv::parse(input, None, None, Some(5), None).unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["1".to_string(), "2".to_string()],
            ]
        );
    }
}

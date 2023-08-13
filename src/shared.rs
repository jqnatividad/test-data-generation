use csv::Reader;
use std::mem;

/// This function converts a String to a &'static str</br>
///
/// # Arguments
///
/// * `s: String` - The textual string to be converted.</br>
///
/// # Example
///
/// ```rust
/// extern crate test_data_generation;
///
/// use test_data_generation::shared;
///
/// fn main() {
///     let my_string = String::from("Hello World");
///        let static_str =  shared::string_to_static_str(my_string);
/// }
/// ```
pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

pub trait CsvManipulator {
    /// This function parses all the rows and splits the columns into separate Vectors
    ///
    /// # Arguments
    /// * `rdr: Reader<&[u8]>` - The csv::Reader that has read the csv file and is ready to process the data.</br>
    ///  
    /// ```rust
    /// extern crate test_data_generation;
    /// extern crate csv;
    ///
    /// use test_data_generation::shared::CsvManipulator;
    /// use csv::Reader;
    ///
    /// fn main() {
    ///     struct CsvMngr {}
    ///     impl CsvManipulator for CsvMngr {}
    ///
    ///     let mut data = String::from("");
    ///     data.push_str("\"firstname\",\"lastname\"\n");
    ///     data.push_str("\"Aaron\",\"Aaberg\"\n");
    ///     data.push_str("\"Aaron\",\"Aaby\"\n");
    ///     data.push_str("\"Abbey\",\"Aadland\"\n");
    ///     data.push_str("\"Abbie\",\"Aagaard\"\n");
    ///     data.push_str("\"Abby\",\"Aakre\"");
    ///     
    ///     let rdr: Reader<&[u8]> = csv::ReaderBuilder::new()
    ///     .has_headers(true)
    ///     .quote(b'"')
    ///     .double_quote(true)
    ///     .delimiter(b',')
    ///     .from_reader(data.as_bytes());///       
    ///     let columns = CsvMngr::read_as_columns(rdr);
    ///     let column0 = vec!("Aaron", "Aaron", "Abbey", "Abbie", "Abby");
    ///     let column1 = vec!("Aaberg", "Aaby", "Aadland", "Aagaard", "Aakre");
    ///     
    ///     println!("firstname: {:?}", column0);
    ///     println!("lastname: {:?}", column1);
    /// }
    /// ```
    fn read_as_columns(mut rdr: Reader<&[u8]>) -> Vec<Vec<String>> {
        let headers = rdr.headers().unwrap().clone();
        let num_columns = headers.len();
        let mut columns = Vec::with_capacity(num_columns);
        let mut record = csv::StringRecord::new();
        let mut columns_len;
        let mut record_len;
        let mut num_new_columns;
        let mut new_columns;
        let mut field;

        while rdr.read_record(&mut record).unwrap() {
            columns_len = columns.len();
            record_len = record.len();
            if columns_len < record_len {
                num_new_columns = record_len - columns_len;
                new_columns = vec![Vec::new(); num_new_columns];
                columns.extend(new_columns);
            }

            for c in 0..record.len() {
                field = record.get(c).unwrap();
                columns[c].push(field.to_owned());
            }
        }

        columns
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct XTest {}
    impl CsvManipulator for XTest {}

    #[test]
    fn test_read_as_columns() {
        let mut data = String::from("");
        data.push_str("\"firstname\",\"lastname\"\n");
        data.push_str("\"Aaron\",\"Aaberg\"\n");
        data.push_str("\"Aaron\",\"Aaby\"\n");
        data.push_str("\"Abbey\",\"Aadland\"\n");
        data.push_str("\"Abbie\",\"Aagaard\"\n");
        data.push_str("\"Abby\",\"Aakre\"");

        let rdr: Reader<&[u8]> = csv::ReaderBuilder::new()
            .has_headers(true)
            .quote(b'"')
            .double_quote(true)
            .delimiter(b',')
            .from_reader(data.as_bytes());

        let columns = XTest::read_as_columns(rdr);
        let column0 = vec!["Aaron", "Aaron", "Abbey", "Abbie", "Abby"];
        let column1 = vec!["Aaberg", "Aaby", "Aadland", "Aagaard", "Aakre"];

        assert_eq!(columns[0], column0);
        assert_eq!(columns[1], column1);
    }

    #[test]
    // ensure the conversion of String to &'static str
    fn test_to_static_str() {
        let static_str: &'static str = "Hello World";
        let my_string = String::from("Hello World");
        let my_static_str = string_to_static_str(my_string);

        assert_eq!(static_str, my_static_str);
    }
}

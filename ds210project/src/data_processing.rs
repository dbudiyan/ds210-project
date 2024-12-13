use std::collections::HashMap;
use csv::ReaderBuilder;

// Processes a CSV file and returns a vector of HashMaps where each map represents a row.
pub fn process_dataset(file_path: &str) -> Vec<HashMap<String, String>> {
    let mut dataset = Vec::new();

    // Create a CSV reader with headers enabled.
    let reader = ReaderBuilder::new().has_headers(true).from_path(file_path);

    if let Ok(mut reader) = reader {
        let headers = reader.headers().unwrap().clone(); // Get headers for mapping

        // Process each record (row) in the CSV file.
        for result in reader.records() {
            if let Ok(record) = result {
                let mut row = HashMap::new();

                // Map each field in the record to its corresponding header.
                for (key, value) in record.iter().enumerate() {
                    row.insert(headers[key].to_string(), value.to_string());
                }

                dataset.push(row); // Add the row to the dataset.
            }
        }
    } else {
        eprintln!("Failed to read the file: {}", file_path); // Handle file read error.
    }

    dataset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_processing() {
        let processed_data = process_dataset("used_car_dataset.csv");
        assert!(!processed_data.is_empty(), "Processed dataset should not be empty.");
    }
}

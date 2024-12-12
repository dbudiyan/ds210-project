use std::collections::HashMap;
use csv::ReaderBuilder;

// Processes the dataset by reading a CSV file and returning a vector of hash maps where each map represents a row.
pub fn process_dataset(file_path: &str) -> Vec<HashMap<String, String>> {
    let mut dataset = Vec::new();
    // Initializes a CSV reader with headers enabled.
    let reader = ReaderBuilder::new().has_headers(true).from_path(file_path);

    // Checks if the file is successfully read, and processes the records if it is.
    if let Ok(mut reader) = reader {
        let headers = reader.headers().unwrap().clone();

        for result in reader.records() {
            if let Ok(record) = result {
                let mut row = HashMap::new();
                for (key, value) in record.iter().enumerate() {
                    row.insert(headers[key].to_string(), value.to_string()); // Use the `headers` variable here
                }
                dataset.push(row);
            }
        }
    } else {
        eprintln!("Failed to read the file: {}", file_path);
    }

    dataset
}

// Filters the dataset based on a specific attribute and value.
pub fn filter_by_attribute(
    dataset: &[HashMap<String, String>],
    attribute: &str,
    value: &str,
) -> Vec<HashMap<String, String>> {
    dataset
        .iter()
        .filter(|entry| entry.get(attribute).map_or(false, |v| v == value))
        .cloned()
        .collect()
}

// Extracts unique values from a specific attribute in the dataset.
pub fn extract_unique_values(dataset: &[HashMap<String, String>], attribute: &str) -> Vec<String> {
    let mut unique_values = dataset
        .iter()
        .filter_map(|entry| entry.get(attribute).cloned())
        .collect::<Vec<_>>();
    unique_values.sort();
    unique_values.dedup();
    unique_values
}

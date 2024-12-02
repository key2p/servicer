use crate::utils::service_names::{get_full_service_name, get_service_file_path};

/// Print contents of a .service file
///
/// # Arguments
///
/// * `name` - The service name
///
pub fn handle_print_service_file(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(name);
    let service_file_path = get_service_file_path(&full_service_name);

    if service_file_path.exists() {
        // Open the file using Tokio's File API
        let buffer = std::fs::read(&service_file_path)?;

        // Convert the buffer to a UTF-8 string and print it
        let contents = String::from_utf8(buffer)?;
        println!(
            "Reading {}:\n{}",
            service_file_path.to_str().unwrap(),
            contents
        );
    } else {
        eprintln!("{}: No such file", service_file_path.to_str().unwrap());
    }

    Ok(())
}

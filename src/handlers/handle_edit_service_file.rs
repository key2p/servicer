use std::path::PathBuf;

use crate::utils::service_names::{get_full_service_name, get_service_file_path};
use tempfile::Builder;

const SERVICE_TEMPLATE: &str = r#"
# Generated with servicer
[Unit]
Description=My Sample Service
After=network.target

[Service]
Type=simple
ExecStart=/path/to/your/command
Restart=always

# Add a reload script to enable the `reload` command
# ExecReload=

[Install]
WantedBy=multi-user.target
"#;

/// Opens an text editor to create or update a service file
///
/// # Arguments
///
/// * `name`- Name of the service to edit
/// * `editor` - Name of editor. The editor must be visible in path
///
pub fn handle_edit_service_file(
    name: &str,
    editor: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(name);
    let service_file_path = get_service_file_path(&full_service_name);

    if service_file_path.exists() {
        let edit_success = edit_file(editor, &service_file_path)?;

        if edit_success {
            println!(
                "Service file {} edited successfully.",
                service_file_path.to_str().unwrap()
            );
        } else {
            eprintln!("Edit operation canceled. No changes were saved.");
        }
    } else {
        // Write the template content to a temporary file
        let temp_file = Builder::new().prefix(&full_service_name).tempfile()?;
        let temp_file_path = temp_file.path().to_owned();

        std::fs::write(&temp_file_path, SERVICE_TEMPLATE.as_bytes())?;

        // Prompt user to edit
        let edit_success = edit_file(editor, &temp_file_path)?;

        if edit_success {
            // Copy the content of the temporary file to the target location
            std::fs::copy(&temp_file_path, &service_file_path)?;

            println!(
                "Service file {} created.",
                service_file_path.to_str().unwrap()
            );
        } else {
            eprintln!("Create operation canceled. No changes were saved.");
        }

        // Remove the temporary file
        std::fs::remove_file(&temp_file_path)?;
    }

    Ok(())
}

/// Prompt the user to edit the file. Returns true if the file editor command exits successfully
/// and the file's `modified` time updates.
///
/// # Args
///
/// * `editor`
/// * `path`
///
fn edit_file(editor: &str, path: &PathBuf) -> Result<bool, std::io::Error> {
    let orig_mod_time = std::fs::metadata(path)?.modified()?;
    let edit_status = std::process::Command::new(editor).arg(path).status()?;

    let edited_mod_time = std::fs::metadata(path)?.modified()?;

    Ok(edit_status.success() && orig_mod_time != edited_mod_time)
}

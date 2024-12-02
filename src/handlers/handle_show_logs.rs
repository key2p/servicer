use crate::utils::service_names::get_full_service_name;
use std::{io::BufRead, process::Stdio};

/// Show logs for a service
///
/// Proxies to `journalctl`. Consider decoding the journal directly in future.
///
/// # Arguments
///
/// * `name`- Name of the service in short form (hello-world)
/// * `follow` - Print logs
///
pub fn handle_show_logs(
    name: &str,
    lines: u32,
    follow: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_name = get_full_service_name(name);

    let mut command = std::process::Command::new("journalctl");

    // Set the journal unit name with -u option
    command.arg("-u").arg(full_name);

    // Set the number of lines to show with -n option
    command.arg("-n").arg(lines.to_string());

    if follow {
        // Enable continuous following with --follow option
        command.arg("--follow");
    }

    // Set stdout to be captured (piped) so we can read the output
    command.stdout(Stdio::piped());

    // Run the command asynchronously
    let mut child = command.spawn()?;

    // Get a handle to the child process's stdout
    let stdout = child.stdout.take().unwrap();

    // Create a stream to read lines from the stdout
    let reader = std::io::BufReader::new(stdout).lines();

    // Process the lines and proxy the output to the user
    for line in reader {
        let line = line?;
        println!("{}", line); // You can send the line to the user in your actual code
    }

    // Wait for the child process to complete and get its exit status
    child.wait()?;

    Ok(())
}

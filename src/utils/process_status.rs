use std::io::BufRead;

/// Gets the kernel page size of the system in KB
pub fn get_page_size() -> Result<usize, Box<dyn std::error::Error>> {
    let path = "/proc/self/smaps";
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut kernel_page_size: Option<usize> = None;

    let lines = reader.lines();
    for line in lines {
        let line = line?;
        if line.starts_with("KernelPageSize:") {
            if let Some(size_str) = line.split_whitespace().nth(1) {
                if let Ok(size) = size_str.parse::<usize>() {
                    kernel_page_size = Some(size);
                    break;
                }
            }
        }
    }

    kernel_page_size.ok_or_else(|| format!("can't find KernelPageSize from {}", path).into())
}

/// Gets the memory used by a process in KB
///
/// Formula from QPS: (rss pages - shared pages) * page size
///
/// # Arguments
///
/// * `pid` - Process ID
/// * `page_size` - The page size in KB
///
pub fn get_memory_usage(pid: u32, page_size_kb: u64) -> Result<u64, std::io::Error> {
    let path = format!("/proc/{}/statm", pid);
    let contents = std::fs::read_to_string(&path)?;

    let values: Vec<&str> = contents.split_whitespace().collect();
    if values.len() < 2 {
        panic!("Invalid format of /proc/PID/statm file");
    }

    let rss_pages: u64 = values[1].parse().unwrap_or(0);
    let shared_pages: u64 = values[2].parse().unwrap_or(0);

    Ok((rss_pages - shared_pages) * page_size_kb)
}

/// Gets the CPU time of a process
///
/// # Arguments
///
/// * `pid`
///
pub fn get_cpu_time(pid: u32) -> Result<u64, Box<dyn std::error::Error>> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = std::fs::read_to_string(stat_path)?;
    let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();

    // The 14th field in /proc/<pid>/stat represents utime (user mode CPU time) in clock ticks
    // The 15th field represents stime (kernel mode CPU time) in clock ticks
    let utime: u64 = stat_fields[13].parse()?;
    let stime: u64 = stat_fields[14].parse()?;

    Ok(utime + stime)
}

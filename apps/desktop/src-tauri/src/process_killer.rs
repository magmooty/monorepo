use std::process::Command;

use log::debug;

static LOG_TARGET: &str = "Process Killer";

#[cfg(target_os = "macos")]
fn shutdown_process(process_id: u32, _: bool) -> Result<(), std::io::Error> {
    use log::debug;

    let output = Command::new("kill")
        .arg("-TERM")
        .arg(process_id.to_string())
        .output()?;

    if output.status.success() {
        debug!(target: LOG_TARGET, "Process {} killed", process_id);
    } else {
        debug!(target: LOG_TARGET, "Failed to kill process {}", process_id);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn find_process_on_port(port: u16) -> Option<u32> {
    use log::debug;

    debug!(target: LOG_TARGET, "Finding processes listening on port {}", port);

    let output = Command::new("lsof")
        .arg("-t")
        .arg("-i")
        .arg(format!(":{}", port))
        .output()
        .expect("Failed to execute lsof command");

    if output.stdout.is_empty() {
        debug!(target: LOG_TARGET, "No process is listening on port {}", port);
        None
    } else {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u32>()
            .ok()
    }
}

#[cfg(target_os = "windows")]
fn shutdown_process(process_id: u32, force: bool) -> Result<(), std::io::Error> {
    use log::debug;

    let output;

    if force {
        output = Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(process_id.to_string())
            .output()
            .expect("Failed to execute lsof command");
    } else {
        output = Command::new("taskkill")
            .arg("/PID")
            .arg(process_id.to_string())
            .output()
            .expect("Failed to execute lsof command");
    }

    if output.status.success() {
        debug!(target: LOG_TARGET, "Process {} killed", process_id);
    } else {
        debug!(target: LOG_TARGET, "Failed to kill process {}", process_id);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn find_process_on_port(port: u16) -> Option<u32> {
    let output = Command::new("netstat")
        .arg("-ano")
        .arg("-p")
        .arg("TCP")
        .output()
        .expect("Failed to execute netstat command");

    if output.stdout.is_empty() {
        debug!(target: LOG_TARGET, "No process is listening on port {}", port);
        None
    } else {
        let output_str = String::from_utf8_lossy(&output.stdout);

        let lines: Vec<&str> = output_str.split("\n").collect();
        for line in lines {
            if line.contains(&format!(":{}", port)) {
                debug!(target: LOG_TARGET, "Process found on port {}: {}", port, line);
                let parts: Vec<&str> = line.split_whitespace().collect();

                // The last part should be the PID
                if let Some(pid) = parts.last() {
                    if let Ok(pid_number) = pid.parse::<u32>() {
                        return Some(pid_number);
                    }
                }
            }
        }

        None
    }
}

pub fn kill_hanging_sidecars() {
    let whatsapp = find_process_on_port(5003);

    if let Some(whatsapp) = whatsapp {
        debug!(target: LOG_TARGET, "Killing hanging WhatsApp sidecar on port 5003 with PID {}", &whatsapp);
        shutdown_process(whatsapp, true).unwrap_or_default();
    }

    let surreal = find_process_on_port(5004);

    if let Some(surreal) = surreal {
        debug!(target: LOG_TARGET, "Killing hanging Surreal sidecar on port 5004 with PID {}", &surreal);
        shutdown_process(surreal, false).unwrap_or_default();
    }
}

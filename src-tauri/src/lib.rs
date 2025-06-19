use tauri::api::process::{Command, CommandEvent};
use tauri::Window;
use std::thread::sleep;
use std::time::Duration;
use std::process;
//use tokio::time::sleep;

/// Starts the sidecar server and restarts it if it crashes
pub async fn start_sidecar_loop(window: Window) {
    loop {
        println!("[Sidecar] Spawning...");

        match Command::new_sidecar("server")
            .expect("failed to create sidecar command")
            .spawn()
        {
            Ok((mut rx, _child)) => {
                println!("[Sidecar] Running");

                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            println!("[Sidecar stdout] {}", line);
                            let _ = window.emit("server-log", line);
                        }
                        CommandEvent::Stderr(line) => {
                            println!("[Sidecar stderr] {}", line);
                            let _ = window.emit("server-error", line);
                        }
                        CommandEvent::Error(error) => {
                            println!("[Sidecar error] {}", error);
                            break;
                        }
                        CommandEvent::Terminated(_) => {
                            println!("[Sidecar] Terminated");
                            break;
                        }
                        _ => {}
                    }
                }

                // Wait a bit before restarting
                println!("[Sidecar] Restarting in 3s...");
                sleep(Duration::from_secs(3));
                //sleep(Duration::from_secs(3)).await;

            }
            Err(e) => {
                println!("[Sidecar] Failed to spawn: {}", e);
                sleep(Duration::from_secs(3));
                // sleep(Duration::from_secs(3)).await;

            }
        }
    }
}



pub async fn monitor_critical_sidecar(window: Window) {
    println!("[Sidecar] Spawning...");

    let result = Command::new_sidecar("server")
        .expect("Failed to create sidecar command")
        .spawn();

    match result {
        Ok((mut rx, _child)) => {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("[Sidecar stdout] {}", line);
                        let _ = window.emit("server-log", line);
                    }
                    CommandEvent::Stderr(line) => {
                        eprintln!("[Sidecar stderr] {}", line);
                        let _ = window.emit("server-error", line);
                    }
                    CommandEvent::Error(err) => {
                        eprintln!("[Sidecar error] {}", err);
                        process::exit(1); // 💥 Exit on error
                    }
                    CommandEvent::Terminated(_) => {
                        eprintln!("[Sidecar] Terminated unexpectedly.");
                        process::exit(1); // 💥 Exit on termination
                    }
                    _ => {}
                }
            }
        }
        Err(err) => {
            eprintln!("[Sidecar] Failed to spawn: {}", err);
            process::exit(1); // 💥 Exit immediately if sidecar can't start
        }
    }
}




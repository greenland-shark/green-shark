extern crate daemonize;

use daemonize::Daemonize;
use notify_rust::Notification;
use signal_hook::iterator::SignalsInfo;
use signal_hook::{consts::SIGTERM, iterator::Signals};
use std::fs;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs::File, thread};
// TODO: Figure out how to delete tmp file once program exists #2

fn main() {
    println!("starting daemon for green shark");

    let stdout = File::create("/tmp/green-shark.out").unwrap();
    let stderr = File::create("/tmp/green-shark.err").unwrap();

    let pid_file = Arc::new("/tmp/shark.pid");
    let pid_file_ref = pid_file.clone();

    let sleep_time = Duration::new(8, 0);

    let daemonize = Daemonize::new()
        .pid_file(*pid_file)
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("jack")
        .group("jack") // Group name
        .group(2) // or group id.
        .umask(0o1777) // Set umask, `0o027` by default.
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(r) => {
            println!("Success, daemonized {}", r);
            io::stdout().flush().unwrap();
            return;
        }

        Err(e) => eprintln!("This Error, {}", e),
    }

    let signal_handler_thread = thread::spawn(move || {
        let mut signal: SignalsInfo = Signals::new(&[SIGTERM]).expect("Bloody thing broked.");
        for sig in signal.forever() {
            match sig {
                SIGTERM => {
                    if let Err(e) = fs::remove_file(*pid_file_ref) {
                        eprintln!("Unable to remove file. {:?}", e)
                    }
                }
                _ => {}
            }
        }
    });

    loop {
        thread::sleep(sleep_time);
        Notification::new()
            .summary("hello green shark")
            .body("Green Shark running")
            .show()
            .unwrap();
    }

    signal_handler_thread.join().expect("Thread panicked!");
}

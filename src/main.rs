extern crate daemonize;

use daemonize::Daemonize;
use notify_rust::Notification;
use std::io::{self, Write};
use std::time::Duration;
use std::{fs::File, thread};

fn main() {
    println!("starting daemon for green shark");

    let stdout = File::create("/tmp/green-shark.out").unwrap();
    let stderr = File::create("/tmp/green-shark.err").unwrap();

    let sleep_time = Duration::new(8, 0);

    let daemonize = Daemonize::new()
        .pid_file("/tmp/shark.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("milton")
        .group("milton") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(r) => {
            println!("Success, daemonized {}", r);
            io::stdout().flush().unwrap();
        }
        Err(e) => eprintln!("This Error, {}", e),
    }
    loop {
        thread::sleep(sleep_time);
        Notification::new()
            .summary("hello green shark")
            .body("Green Shark running")
            .show()
            .unwrap();
    }
}

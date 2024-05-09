use chrono::{DateTime, Local, NaiveDate};
use green_shark::state::State;
use std::{env, error::Error};
use zbus::ConnectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let last_update: Option<NaiveDate> = None;
    let next_update: Option<NaiveDate> = None;
    let config = check_if_for_env_config_exists_if_not_create_it();

    let state = match State::from_file(&config) {
        Ok(s) => s,
        Err(_) => State::new(),
    };


    let _connection = ConnectionBuilder::session()?
        .name("org.green_sharkd.GreenSharkd")?
        .serve_at("/org/green_sharkd/State", state)?
        .build()
        .await?;

    loop {
        let now = Local::now();
        let now = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        println!("now: {}", now);
        // futures::future::pending::<()>().await;
    }
}

fn check_if_for_env_config_exists_if_not_create_it() -> String {
    if let Ok(path) = env::var("GREEN_SHARK_CONFIG") {
        path
    } else {
        let mut path = env::var("HOME").unwrap();
        path.push_str("/.config/green_shark");
        path
    }
}

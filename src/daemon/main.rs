use zbus::{Connection, Result, dbus_interface, ConnectionBuilder, SignalContext, fdo};
use futures::stream::TryStreamExt;
use event_listener::Event;
use green_shark::state::State;

#[tokio::main]
async fn main() -> Result<()> {
    let state = State::new();

    let _connection = ConnectionBuilder::session()?
        .name("org.green_sharkd.GreenSharkd")?
        .serve_at("/org/green_sharkd/State", state)?
        .build()
        .await?;


    loop {
        futures::future::pending::<()>().await;
    }

}

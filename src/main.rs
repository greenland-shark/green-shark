use zbus::{Connection, Result, dbus_interface, ConnectionBuilder, SignalContext, fdo};
use futures::stream::TryStreamExt;
use event_listener::Event;

struct Greeter {
    name: String,
    done: Event,
}

#[dbus_interface(name = "org.green_sharkd.Greeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    async fn go_away(&self,
    #[zbus(signal_context)]
    ctxt: SignalContext<'_>,
    ) -> fdo::Result<()> {
        Self::greeted_everyone(&ctxt).await?;
        self.done.notify(1);

        Ok(())
    }

    #[dbus_interface(property)]
    async fn greeter_name(&self) -> &str {
        &self.name
    }

    #[dbus_interface(property)]
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };

    let done_listener = greeter.done.listen();

    let connection = ConnectionBuilder::session()?
        .name("org.green_sharkd.Greeter")?
        .serve_at("/org/green_sharkd/Greeter", greeter)?
        .build()
        .await?;


    done_listener.await;


    loop {
        futures::future::pending::<()>().await;
    }

}

use zbus::{Connection, Result, dbus_interface, ConnectionBuilder, SignalContext, fdo};
use futures::stream::TryStreamExt;
use event_listener::Event;
use chrono::Utc;

#[derive(Debug)]
enum Currency {
    NZD,
    GBP,
    BRL,
}

#[derive(Debug)]
struct Transaction {
    amount: (Currency, f32),
    name: String,
    label: String,
    id: i64,
    frequency: Frequency,
    start_date: i64,
    end_date: Option<i64>,
}

#[derive(Debug)]
enum Frequency {
    OneOff(i64),
    Monthly(u8),
    Weekly(u8),
    Yearly((u8, u8)),
}

#[derive(Debug)]
struct State {
    transactions: Vec<Transaction>
}

#[dbus_interface(name = "org.green_sharkd.Commands")]
impl State {
    async fn add_transaction(&mut self, amount: f32, name: &str) -> String {
        println!("adding transaction");
        let now = Utc::now();
        let uid = now.timestamp();
        let now = now.timestamp();
        self.transactions.push( Transaction {
            amount: (Currency::GBP, amount),
            name: name.to_string(),
            label: "Coffee".to_string(),
            id: uid,
            frequency: Frequency::OneOff(now),
            start_date: now,
            end_date: None,
        });

        format!("Transactions: {:?}", self)
    }

    // async fn go_away(&self,
    // #[zbus(signal_context)]
    // ctxt: SignalContext<'_>,
    // ) -> fdo::Result<()> {
    //     Self::greeted_everyone(&ctxt).await?;
    //     self.done.notify(1);

    //     Ok(())
    // }

    #[dbus_interface(property)]
    async fn transactions(&self) -> String {
        format!("{:?}", self.transactions)
    }

    // #[dbus_interface(property)]
    // async fn set_greeter_name(&mut self, name: String) {
    //     self.name = name;
    // }

    // #[dbus_interface(signal)]
    // async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let state: State = State {
        transactions: Vec::new()
    };


    let _connection = ConnectionBuilder::session()?
        .name("org.green_sharkd.GreenSharkd")?
        .serve_at("/org/green_sharkd/State", state)?
        .build()
        .await?;


    loop {
        futures::future::pending::<()>().await;
    }

}


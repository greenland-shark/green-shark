use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads};
use std::error::Error;

struct Hello {
    called_count: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Handle error
    let conn = Connection::new_session()?;

    conn.request_name("com.green_shark.green_sharkd", false, true, false)?;

    let mut cr = Crossroads::new();

    let iface_token = cr.register("com.green_shark.green_sharkd", |b| {
        let hello_happened = b
            .signal::<(String,), _>("HelloHappened", ("sender",))
            .msg_fn();

        b.method(
            "Hello",
            ("name",),
            ("reply",),
            move |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
                println!("Incoming hello call from {}!", name);
                hello.called_count += 1;
                let reply = format!(
                    "Hello {}! This API has been used {} times.",
                    name, hello.called_count
                );

                let signal_msg = hello_happened(ctx.path(), &(name,));
                ctx.push_msg(signal_msg);

                Ok((reply,))
            },
        );
    });

    cr.insert("/hello", &[iface_token], Hello { called_count: 0 });

    cr.serve(&conn)?;

    unreachable!()
}

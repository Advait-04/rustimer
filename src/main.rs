// 72

use chrono::{prelude::DateTime, Local};
use futures_util::stream::StreamExt;
use std::{
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    usize,
};
use zbus::Connection;
use zbus_macros::proxy;

#[proxy(
    default_service = "org.gnome.SessionManager",
    default_path = "/org/gnome/SessionManager/Presence",
    interface = "org.gnome.SessionManager.Presence"
)]
trait SessionManager {
    #[zbus(signal)]
    fn status_changed(&self, status: u32) -> zbus::Result<()>;
}

#[derive(Debug)]
struct Timer {
    timestamp: u128,
}

impl Timer {
    fn update(&mut self, new_value: u128) {
        self.timestamp = new_value;
    }

    fn calculate_diff(&self, current_time: u128) -> u128 {
        current_time - self.timestamp
    }
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    // est connections
    let session_connection = Connection::session().await?;

    // setup proxy with the connections
    let session_manager_proxy = SessionManagerProxy::new(&session_connection).await?;

    let mut session_manager_stream = session_manager_proxy.receive_status_changed().await?;

    let some_time = Local::now();
    println!("is this working {:?}", some_time);

    while let Some(signal) = session_manager_stream.next().await {
        let args: StatusChangedArgs = signal.args().expect("Error parsing the args");
        match args.status {
            0 => {
                println!("log back in time: {:?}", Instant::now())
            }
            3 => {
                println!("logout time: {:?}", Instant::now())
            }
            _ => println!("ignore me man"),
        }
    }

    Ok(())
}

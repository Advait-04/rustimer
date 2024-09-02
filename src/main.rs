//

use chrono::{DateTime, Local, TimeDelta, Utc};
use futures_util::{stream::StreamExt, try_join, TryFutureExt};
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

#[proxy(
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/session/_32",
    interface = "org.freedesktop.login1.Session"
)]
trait Login1Session {
    #[zbus(signal)]
    fn unlock(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn lock(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn active(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<String>;
}

#[derive(Debug)]
struct Timer {
    timestamp: DateTime<Utc>,
}

impl Timer {
    fn update(&mut self, new_value: DateTime<Utc>) {
        self.timestamp = new_value;
    }

    fn calculate_diff(&self, current_time: DateTime<Utc>) -> TimeDelta {
        current_time.time() - self.timestamp.time()
    }
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    // est connections
    let session_connection = Connection::session().await?;
    let system_connection = Connection::system().await?;

    // setup proxy with the connections
    let session_manager_proxy = SessionManagerProxy::new(&session_connection).await?;
    let login_session_proxy = Login1SessionProxy::new(&system_connection).await?;

    let mut session_manager_stream = session_manager_proxy.receive_status_changed().await?;
    let mut login_state_stream = login_session_proxy.receive_unlock().await?;

    let unlock_stream_handle = tokio::spawn(async move {
        while let Some(signal) = login_state_stream.next().await {
            println!("Unlocked at: {}", Local::now());
        }
    });

    let lock_stream_handle = tokio::spawn(async move {
        while let Some(signal) = session_manager_stream.next().await {
            let args: StatusChangedArgs = signal.args().expect("Error parsing the args");
            match args.status {
                3 => println!("this is the lock part: {}", Local::now()),
                _ => {}
            }
        }
    });

    try_join!(lock_stream_handle, unlock_stream_handle);

    Ok(())
}

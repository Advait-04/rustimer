use futures_util::stream::StreamExt;
use zbus::Connection;
use zbus_macros::proxy;

#[proxy(
    default_service = "org.gnome.ScreenSaver",
    default_path = "/org/gnome/ScreenSaver",
    interface = "org.gnome.ScreenSaver"
)]
trait ScreenSaver {
    #[zbus(signal)]
    fn active_changed(&self, active_changed: bool) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let screen_saver_proxy = ScreenSaverProxy::new(&connection).await?;
    let mut screan_saver_stream = screen_saver_proxy.receive_active_changed().await?;

    while let Some(msg) = screan_saver_stream.next().await {
        let args: ActiveChangedArgs = msg.args().expect("Error while parsing message");
        println!("Got new message boys: status:{}", args.active_changed)
    }

    Ok(())
}

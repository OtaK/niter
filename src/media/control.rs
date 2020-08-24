#[zbus::dbus_proxy(
    interface = "org.bluez.MediaControl1",
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX",
)]
pub trait MediaControl {
    #[deprecated]
    fn play(&self) -> zbus::Result<()>;
    #[deprecated]
    fn pause(&self) -> zbus::Result<()>;
    #[deprecated]
    fn stop(&self) -> zbus::Result<()>;
    #[deprecated]
    fn next(&self) -> zbus::Result<()>;
    #[deprecated]
    fn previous(&self) -> zbus::Result<()>;
    #[deprecated]
    fn volume_up(&self) -> zbus::Result<()>;
    #[deprecated]
    fn volume_down(&self) -> zbus::Result<()>;
    #[deprecated]
    fn fast_forward(&self) -> zbus::Result<()>;
    #[deprecated]
    fn rewind(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn player(&self) -> zbus::fdo::Result<super::player::MediaPlayer>;
}

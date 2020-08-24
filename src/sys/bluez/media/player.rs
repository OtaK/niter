#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaPlayer {
    object_path: String
}

impl std::str::FromStr for MediaPlayer {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::impl_tryfrom_zvariant!(MediaPlayer);
crate::to_proxy_impl!(MediaPlayer, MediaPlayerProxy, "org.bluez");

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub enum EqualizerStatus {
    On,
    Off,
}

crate::impl_tryfrom_zvariant!(EqualizerStatus);
crate::impl_enumto_zstruct!(EqualizerStatus);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub enum RepeatStatus {
    Off,
    SingleTrack,
    AllTracks,
    Group,
}

crate::impl_tryfrom_zvariant!(RepeatStatus);
crate::impl_enumto_zstruct!(RepeatStatus);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub enum ShuffleScanStatus {
    Off,
    AllTracks,
    Group,
}

crate::impl_tryfrom_zvariant!(ShuffleScanStatus);
crate::impl_enumto_zstruct!(ShuffleScanStatus);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum PlayerStatus {
    Playing,
    Stopped,
    Paused,
    ForwardSeek,
    ReverseSeek,
    Error,
}

crate::impl_tryfrom_zvariant!(PlayerStatus);
crate::impl_enumto_zstruct!(PlayerStatus);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum PlayerType {
    Audio,
    Video,
    #[strum(serialize = "Audio Broadcasting")]
    AudioBroadcasting,
    #[strum(serialize = "Video Broadcasting")]
    VideoBroadcasting,
}

crate::impl_tryfrom_zvariant!(PlayerType);
crate::impl_enumto_zstruct!(PlayerType);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum PlayerSubtype {
    Podcast,
    #[strum(serialize = "Audio Book")]
    AudioBook,
}

crate::impl_tryfrom_zvariant!(PlayerSubtype);
crate::impl_enumto_zstruct!(PlayerSubtype);

#[zbus::dbus_proxy(
    interface = "org.bluez.MediaPlayer1",
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/playerX",
)]
pub trait MediaPlayer {
    fn play(&self) -> zbus::Result<()>;
    fn pause(&self) -> zbus::Result<()>;
    fn stop(&self) -> zbus::Result<()>;
    fn next(&self) -> zbus::Result<()>;
    fn previous(&self) -> zbus::Result<()>;
    fn fast_forward(&self) -> zbus::Result<()>;
    fn rewind(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn equalizer(&self) -> zbus::fdo::Result<EqualizerStatus>;
    #[dbus_proxy(property)]
    fn set_equalizer(&self, equalizer: EqualizerStatus) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn repeat(&self) -> zbus::fdo::Result<RepeatStatus>;
    #[dbus_proxy(property)]
    fn set_repeat(&self, repeat: RepeatStatus) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn shuffle(&self) -> zbus::fdo::Result<ShuffleScanStatus>;
    #[dbus_proxy(property)]
    fn set_shuffle(&self, shuffle: ShuffleScanStatus) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn scan(&self) -> zbus::fdo::Result<ShuffleScanStatus>;
    #[dbus_proxy(property)]
    fn set_scan(&self, scan: ShuffleScanStatus) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::fdo::Result<PlayerStatus>;
    #[dbus_proxy(property)]
    fn position(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn track(&self) -> zbus::fdo::Result<super::item::MediaItemMetadata>;
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<crate::sys::bluez::device::Device>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property, name = "Type")]
    fn player_type(&self) -> zbus::fdo::Result<PlayerType>;
    #[dbus_proxy(property)]
    fn subtype(&self) -> zbus::fdo::Result<PlayerSubtype>;
    #[dbus_proxy(property)]
    fn browsable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn searchable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn playlist(&self) -> zbus::fdo::Result<String>;
}

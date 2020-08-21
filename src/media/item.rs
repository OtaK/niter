#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
    zvariant_derive::Type,
)]
#[strum(serialize_all = "lowercase")]
pub enum MediaItemType {
    Video,
    Audio,
    Folder,
}

crate::impl_tryfrom_zvariant!(MediaItemType);
crate::impl_enumto_zstruct!(MediaItemType);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
    zvariant_derive::Type,
)]
#[strum(serialize_all = "lowercase")]
pub enum MediaFolderType {
    Mixed,
    Titles,
    Albums,
    Artists,
}

crate::impl_tryfrom_zvariant!(MediaFolderType);
crate::impl_enumto_zstruct!(MediaFolderType);

#[derive(Debug, Clone, Default, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaItemMetadata {
    title: String,
    artist: String,
    album: String,
    genre: String,
    number_of_tracks: u32,
    number: u32,
    duration: u32
}

impl std::convert::TryFrom<zvariant::OwnedValue> for MediaItemMetadata {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let dict: zvariant::Dict = value.try_into()?;
        let mut ret = Self::default();
        ret.title = (dict.get("Title")? as Option<&str>).unwrap_or_default().into();
        ret.artist = (dict.get("Artist")? as Option<&str>).unwrap_or_default().into();
        ret.album = (dict.get("Album")? as Option<&str>).unwrap_or_default().into();
        ret.genre = (dict.get("Genre")? as Option<&str>).unwrap_or_default().into();
        ret.number_of_tracks = (dict.get("NumberOfTracks")?.copied() as Option<u32>).unwrap_or_default();
        ret.number = (dict.get("Number")?.copied() as Option<u32>).unwrap_or_default();
        ret.duration = (dict.get("Duration")?.copied() as Option<u32>).unwrap_or_default();

        Ok(ret)
    }
}

impl<'a> Into<zvariant::Value<'a>> for MediaItemMetadata {
    fn into(self) -> zvariant::Value<'a> {
        use zvariant::Type as _;
        let mut dict = zvariant::Dict::new(String::signature(), zvariant::Value::signature());
        let _ = dict.add("Title", zvariant::Value::Str(self.title.into()));
        let _ = dict.add("Artist", zvariant::Value::Str(self.artist.into()));
        let _ = dict.add("Album", zvariant::Value::Str(self.album.into()));
        let _ = dict.add("Genre", zvariant::Value::Str(self.genre.into()));
        let _ = dict.add("NumberOfTracks", self.number_of_tracks);
        let _ = dict.add("Number", self.number);
        let _ = dict.add("Duration", self.duration);

        zvariant::Value::Dict(dict)
    }
}

pub trait MediaItemDelegate<E: std::error::Error>: zvariant::Type {
    fn play(&mut self) -> Result<(), E>;
    fn add_to_now_playing(&mut self) -> Result<(), E>;

    //fn player(&self) -> &MediaPlayer;
    fn name(&self) -> &str;
    fn item_type(&self) -> MediaItemType;
    fn folder_type(&self) -> MediaFolderType;
    fn playable(&self) -> bool;
    fn metadata(&self) -> &MediaItemMetadata;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaItemTarget<T: MediaItemDelegate<zbus::fdo::Error>> {
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.MediaItem1")]
impl<T: MediaItemDelegate<zbus::fdo::Error>> MediaItemTarget<T> {
    fn play(&mut self) -> zbus::fdo::Result<()> {
        self.delegate.play()
    }
    fn add_to_now_playing(&mut self) -> zbus::fdo::Result<()> {
        self.delegate.add_to_now_playing()
    }

    #[dbus_interface(property, name = "Name")]
    fn item_name(&self) -> &str {
        self.delegate.name()
    }
    #[dbus_interface(property, name = "Type")]
    fn item_type(&self) -> MediaItemType {
        self.delegate.item_type()
    }
    #[dbus_interface(property)]
    fn folder_type(&self) -> MediaFolderType {
        self.delegate.folder_type()
    }
    #[dbus_interface(property)]
    fn playable(&self) -> bool {
        self.delegate.playable()
    }
    #[dbus_interface(property)]
    fn metadata(&self) -> zvariant::Value {
        self.delegate.metadata().clone().into()
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.MediaItem1",
    default_service = "org.bluez",
    //default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/playerX/itemX"
)]
pub trait MediaItemController {
    fn play(&self) -> zbus::Result<()>;
    fn add_to_now_playing(&self) -> zbus::Result<()>;

    // #[dbus_proxy(property)]
    // fn player(&self) -> zbus::fdo::Result<MediaPlayer>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property, name = "Type")]
    fn item_type(&self) -> zbus::fdo::Result<MediaItemType>;
    #[dbus_proxy(property)]
    fn folder_type(&self) -> zbus::fdo::Result<MediaFolderType>;
    #[dbus_proxy(property)]
    fn playable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn metadata(&self) -> zbus::fdo::Result<MediaItemMetadata>;
}

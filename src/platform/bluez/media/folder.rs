use crate::platform::bluez::media::item::{MediaItemAttibute, MediaItemMetadata};

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaPlayerFilter {
    start: u32,
    end: u32,
    attributes: Vec<MediaItemAttibute>
}

pub trait MediaFolderDelegate: zvariant::Type + Sized {
    fn search(&self, value: String, filter: MediaPlayerFilter) -> zbus::fdo::Result<MediaFolder<Self>>;
    fn list_items(&self, filter: MediaPlayerFilter) -> zbus::fdo::Result<Vec<(String, MediaItemMetadata)>>;
    fn change_folder(&self, folder: String) -> zbus::fdo::Result<()>;

    fn number_of_items(&self) -> u32;
    fn folder_name(&self) -> &str;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaFolder<T: MediaFolderDelegate> {
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.MediaFolder1")]
impl<T: MediaFolderDelegate + serde::Serialize> MediaFolder<T> {
    fn search(&self, value: String, filter: MediaPlayerFilter) -> zbus::fdo::Result<Self> {
        self.delegate.search(value, filter)
    }
    fn list_items(&self, filter: MediaPlayerFilter) -> zbus::fdo::Result<Vec<(String, MediaItemMetadata)>> {
        self.delegate.list_items(filter)
    }
    fn change_folder(&self, folder: String) -> zbus::fdo::Result<()> {
        self.delegate.change_folder(folder)
    }

    #[dbus_interface(property)]
    fn number_of_items(&self) -> u32 {
        self.delegate.number_of_items()
    }
    #[dbus_interface(property, name = "Name")]
    fn folder_name(&self) -> &str {
        self.delegate.folder_name()
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.MediaFolder1",
    default_service = "org.bluez",
    //default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/playerX"
)]
pub trait MediaFolderController {
    fn search(&self, value: String, filter: MediaPlayerFilter) -> zbus::Result<String>;
    fn list_items(&self, filter: MediaPlayerFilter) -> zbus::Result<Vec<(String, MediaItemMetadata)>>;
    fn change_folder(&self, folder: String) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn number_of_items(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property, name = "Name")]
    fn folder_name(&self) -> zbus::fdo::Result<String>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ZvariantableArray<T: zvariant::Type>(Vec<T>);

impl<T: zvariant::Type> std::ops::Deref for ZvariantableArray<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: zvariant::Type> From<Vec<T>> for ZvariantableArray<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<'a, T: zvariant::Type> Into<zvariant::Structure<'a>> for ZvariantableArray<T> where T: Into<zvariant::Value<'a>> {
    fn into(self) -> zvariant::Structure<'a> {
        let mut zstruct = zvariant::Structure::new();
        for item in self.0.into_iter() {
            zstruct.add_field(item.into());
        }
        zstruct
    }
}

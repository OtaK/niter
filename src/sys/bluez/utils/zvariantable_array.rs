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

impl<'a, T: zvariant::Type> Into<zvariant::Structure<'a>> for ZvariantableArray<T>
where
    T: Into<zvariant::Value<'a>>,
{
    fn into(self) -> zvariant::Structure<'a> {
        let mut zstruct = zvariant::Structure::new();
        for item in self.0.into_iter() {
            zstruct = zstruct.add_field(item.into());
        }
        zstruct
    }
}

impl<'a, T> std::convert::TryFrom<zvariant::OwnedValue> for ZvariantableArray<T>
where
    T: std::convert::TryFrom<zvariant::Value<'a>, Error = crate::NiterError> + zvariant::Type,
{
    type Error = crate::NiterError;
    fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
        use std::convert::TryInto as _;
        let zva: zvariant::Array = v.try_into()?;
        let zva_len = zva.len();
        let inner: Vec<T> = zva.iter().cloned().try_fold(
            Vec::with_capacity(zva_len),
            |mut acc, item| -> crate::NiterResult<Vec<T>> {
                acc.push(item.try_into()?);
                Ok(acc)
            },
        )?;
        Ok(Self(inner))
    }
}

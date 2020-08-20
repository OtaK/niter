use std::collections::HashMap;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct ZvariantableMap<
    K: std::hash::Hash + Eq + zvariant::Type + zvariant::Basic + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>,
    V: zvariant::Type + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>
>(HashMap<K, V>);

impl<K, V> std::ops::Deref for ZvariantableMap<K, V>
where
    K: std::hash::Hash + Eq + zvariant::Type + zvariant::Basic + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>,
    V: zvariant::Type + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>
{
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> From<HashMap<K, V>> for ZvariantableMap<K, V>
where
    K: std::hash::Hash + Eq + zvariant::Type + zvariant::Basic + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>,
    V: zvariant::Type + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>
{
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

impl<K, V> std::convert::TryFrom<zvariant::OwnedValue> for ZvariantableMap<K, V> where
    K: std::hash::Hash + Eq + zvariant::Type + zvariant::Basic + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>,
    V: zvariant::Type + std::convert::TryFrom<zvariant::Value<'static>> + std::convert::TryFrom<zvariant::OwnedValue>
{
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let zvdict: zvariant::Dict<'static, 'static> = value.try_into()?;
        let hash = zvdict.try_into()?;
        Ok(Self(hash))
    }
}


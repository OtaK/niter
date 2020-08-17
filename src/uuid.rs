use crate::impl_tryfrom_zvariant;
use crate::error::*;

const BASE_UUID: (u32, u16, u16, u64) = (0x00000000, 0x0000, 0x1000, 0x800000805F9B34FB);

pub trait SdpShortUUID<T: Into<u32>> {
    #[inline(always)]
    fn from_sdp_short_uuid(uuid: T) -> NiterResult<uuid::Uuid> {
        uuid::Uuid::from_fields(
            uuid.into(),
            BASE_UUID.1,
            BASE_UUID.2,
            &BASE_UUID.3.to_be_bytes(),
        )
        .map_err(Into::into)
    }
}

impl SdpShortUUID<u16> for uuid::Uuid {}
impl SdpShortUUID<u32> for uuid::Uuid {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Uuid(uuid::Uuid);

impl std::str::FromStr for Uuid {
    type Err = NiterError;
    fn from_str(s: &str) -> NiterResult<Self> {
        let inner = uuid::Uuid::parse_str(s)?;
        Ok(Self(inner))
    }
}

impl_tryfrom_zvariant!(Uuid);

impl From<uuid::Uuid> for Uuid {
    fn from(v: uuid::Uuid) -> Self {
        Self(v)
    }
}

impl zvariant::Basic for Uuid {
    const SIGNATURE_CHAR: char = String::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = String::SIGNATURE_STR;
    const ALIGNMENT: usize = String::ALIGNMENT;
}

impl zvariant::Type for Uuid {
    #[inline]
    fn signature() -> zvariant::Signature<'static> {
        String::signature()
    }
}

impl std::ops::Deref for Uuid {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct UuidArray(Vec<Uuid>);

impl<'a> From<zvariant::Array<'a>> for UuidArray {
    fn from(v: zvariant::Array<'a>) -> Self {
        Self(v
            .get()
            .into_iter()
            .map(std::convert::TryInto::try_into)
            .map(Result::<String, zvariant::Error>::ok)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|a| uuid::Uuid::parse_str(&a))
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .map(crate::Uuid::from)
            .collect()
        )
    }
}

impl From<zvariant::OwnedValue> for UuidArray {
    fn from(v: zvariant::OwnedValue) -> Self {
        use zvariant::Type as _;
        use std::convert::TryInto as _;
        let a: zvariant::Array<'_> = v.try_into().unwrap_or_else(|_| zvariant::Array::new(UuidArray::signature()));
        a.into()
    }
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! to_proxy_impl {
    ($struct: ident, $proxy: ident, $service: expr) => {
        impl $struct {
            pub fn get_proxy<'a>(
                &'a self,
                connection: &'a zbus::Connection,
            ) -> crate::error::NiterResult<$proxy<'a>> {
                Ok($proxy::new_for(connection, $service, &self.object_path)?)
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! impl_tryfrom_zvariant {
    ($thing:ident) => {
        impl<'a> std::convert::TryFrom<zvariant::Value<'a>> for $thing {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::Value<'a>) -> crate::NiterResult<Self> {
                use std::str::FromStr as _;
                let s: String = v.downcast().ok_or_else(|| zvariant::Error::IncorrectType)?;
                Ok(Self::from_str(&s)?)
            }
        }

        impl std::convert::TryFrom<zvariant::OwnedValue> for $thing {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
                use std::convert::TryInto as _;
                use std::str::FromStr as _;
                let s: String = v.try_into()?;
                Ok(Self::from_str(&s)?)
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! impl_enumto_zstruct {
    ($thing: ident) => {
        impl Into<zvariant::Structure<'_>> for $thing {
            fn into(self) -> zvariant::Structure<'static> {
                use std::string::ToString as _;
                let s = self.to_string();
                zvariant::StructureBuilder::new().append_field(zvariant::Value::Str(s.into())).build()
            }
        }
    };
}

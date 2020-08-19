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
        impl std::convert::TryFrom<zvariant::OwnedValue> for crate::ZvariantableArray<$thing> {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
                use std::convert::TryInto as _;
                let zva: zvariant::Array = v.try_into()?;
                let zva_len = zva.len();
                let inner: Vec<$thing> = zva.iter().cloned().try_fold(
                    Vec::with_capacity(zva_len),
                    |mut acc, item| -> crate::NiterResult<Vec<$thing>> {
                        acc.push(item.try_into()?);
                        Ok(acc)
                    },
                )?;
                Ok(Self(inner))
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
                let s = ::std::format!("{}", self);
                zvariant::Structure::new().append_field(zvariant::Value::Str(s.into()))
            }
        }
    };
}

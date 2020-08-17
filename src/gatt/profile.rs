#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattProfile<T: zvariant::Type> {
    uuids: Vec<crate::Uuid>,
    profile_impl: T
}

impl<T: zvariant::Type> GattProfile<T> {
    fn release(&self) -> zbus::fdo::Result<()> {
        // We're dropping self since that's a profile release signal
        // Side effect is that it'll drop the inner profile_impl
        drop(self);
        Ok(())
    }
}

impl<T: zvariant::Type> zbus::Interface for GattProfile<T> {
    fn name() -> &'static str { "org.bluez.GattProfile1" }

    fn get(&self, prop: &str) -> Option<zbus::fdo::Result<zvariant::OwnedValue>> {
        if prop != "UUIDs" {
            None
        } else {
            let mut buf = uuid::Uuid::encode_buffer();
            let arr: Vec<zvariant::Value> = self.uuids.iter().map(move |uuid| {
                let uuid_str = uuid
                    .to_hyphenated()
                    .encode_lower(&mut buf) as &str;

                zvariant::Value::Str(uuid_str.to_string().into())
            }).collect();

            Some(Ok(zvariant::Value::Array(arr.into()).into()))
        }
    }

    fn get_all(&self) -> std::collections::HashMap<String, zvariant::OwnedValue> {
        let mut ret = std::collections::HashMap::new();
        if let Some(uuids_result) = self.get("UUIDs") {
            if let Ok(uuids) = uuids_result {
                ret.insert("UUIDs".into(), uuids);
            }
        }
        ret
    }

    fn set(&mut self, property_name: &str, value: &zvariant::Value) -> Option<zbus::fdo::Result<()>> {
        if property_name == "UUIDs" {
            use std::convert::TryInto as _;
            let tryinto_res: Result<zvariant::Array, zvariant::Error> = value.try_into();
            match tryinto_res {
                Ok(value) => {
                    self.uuids = value.get()
                        .iter()
                        .map(|s_value| {
                            if let Some(s) = s_value.downcast_ref::<str>() {
                                uuid::Uuid::parse_str(s).unwrap_or_default()
                            } else {
                                uuid::Uuid::nil()
                            }
                        })
                        .map(Into::into)
                        .collect();
                    Some(Ok(()))
                }
                Err(e) => Some(Err(zbus::fdo::Error::ZBus(e.into())))
            }
        } else {
            None
        }
    }

    fn call(&self, _connection: &zbus::Connection, _msg: &zbus::Message, name: &str) -> Option<zbus::Result<u32>> {
        if name == "Release" {
            let _ = self.release();
            Some(Ok(1))
        } else {
            None
        }
    }

    fn call_mut(
            &mut self,
            _connection: &zbus::Connection,
            _msg: &zbus::Message,
            name: &str,
        ) -> Option<zbus::Result<u32>> {
        if name == "Release" {
            let _ = self.release();
            Some(Ok(1))
        } else {
            None
        }
    }

    fn introspect_to_writer(&self, _writer: &mut dyn std::fmt::Write, _level: usize) {
        // TODO: How about no
        todo!()
    }
}


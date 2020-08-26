pub trait Device: std::fmt::Debug {
    type Service; // FIXME: Actual traits
    type Characteristic; // FIXME: Actual traits
    type Descriptor; // FIXME: Actual traits

    fn address(&self) -> String;
    fn is_connected(&self) -> bool;
    fn rssi(&self) -> i16;
    fn connect(&self) -> crate::NiterResult<()>;
    fn disconnect(&self) -> crate::NiterResult<()>;
    fn characteristics(&self) -> crate::NiterResult<Vec<Self::Characteristic>>;
    fn services(&self) -> crate::NiterResult<Vec<Self::Service>>;
    fn descriptors(&self) -> crate::NiterResult<Vec<Self::Descriptor>>;
}

pub trait Central<D: Device>: std::fmt::Debug {
    fn start_scan(&self) -> crate::NiterResult<()>;
    fn stop_scan(&self) -> crate::NiterResult<()>;
    fn filter_duplicates(&mut self, filter: bool) -> crate::NiterResult<()>;
    fn devices(&self) -> crate::NiterResult<Vec<D>>;
    fn get_device(&self, ) -> crate::NiterResult<Option<D>>;
}

pub(crate) trait PlatformCentral<P: crate::PlatformPeripheral>: std::fmt::Debug {
    fn start_scan(&self) -> crate::NiterResult<()>;
    fn stop_scan(&self) -> crate::NiterResult<()>;
    fn filter_duplicates(&mut self, filter: bool) -> crate::NiterResult<()>;
    fn peripherals(&self) -> crate::NiterResult<Vec<P>>;
    fn get_peripheral(&self, ) -> crate::NiterResult<Option<P>>;
}

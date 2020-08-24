pub trait Central<P> {
    fn start_scan(&self) -> crate::NiterResult<()>;
    fn filter_duplicates(&self) -> crate::NiterResult<()>;
    fn stop_scan(&self) -> crate::NiterResult<()>;
    fn peripherals(&self) -> crate::NiterResult<Vec<P>>;
    fn get_peripheral(&self) -> crate::NiterResult<Option<P>>;
}

// #[derive(Debug)]
// pub struct Manager {}

// impl Manager {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn enumerate_adapters(&self) -> Result<Vec<Adapter>> {

//     }
// }


#[cfg(test)]
mod test {
    #[async_std::test]
    async fn find_bt_adapters() -> windows::core::Result<()> {
        // FIXME: Find a way to make async/await work on Windows; Might be related to the new bindgen impl
        // let radios = windows::Devices::Radios::Radio::GetRadiosAsync()?.await?;
        // println!("{:?}", radios);
        Ok(())
    }
}

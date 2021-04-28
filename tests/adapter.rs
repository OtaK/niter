#[test]
fn get_default_adapter() {
    let connection = zbus::Connection::new_system().unwrap();
    let adapter_proxy = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
    adapter_proxy.name().unwrap();
}

#[test]
fn get_default_adapter_uuids() {
    let connection = zbus::Connection::new_system().unwrap();
    let adapter_proxy = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
    adapter_proxy.uuids().unwrap();
}

#[test]
fn enumerate_adapters() {
    let connection = zbus::Connection::new_system().unwrap();
    let enumerator = niter::platform::adapter::AdapterProxy::enumerate_adapters(&connection).unwrap();
    println!("{:#?}", enumerator);
}

#[test]
fn enumerate_devices() {
    let connection = zbus::Connection::new_system().unwrap();
    let enumerator = niter::platform::device::DeviceProxy::enumerate_devices(&connection).unwrap();
    println!("{:#?}", enumerator);
}

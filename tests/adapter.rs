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

#[test]
fn connect_to_device() {
    let connection = zbus::Connection::new_system().unwrap();
    let adapter = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
    adapter.start_discovery().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    let mut devices = niter::platform::device::DeviceProxy::enumerate_devices(&connection).unwrap();
    if let Some(d) = devices.next() {
        println!("Device: {:#?}", d);
        let path: String = d.object_path;
        let device = niter::platform::device::DeviceProxy::new_for_owned(connection, "org.bluez".into(), path).unwrap();
        device.connect().unwrap();
    }

}

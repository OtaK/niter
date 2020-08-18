#[test]
fn get_default_adapter() {
    let connection = zbus::Connection::new_system().unwrap();
    let adapter_proxy = niter::adapter::AdapterProxy::new(&connection).unwrap();
    adapter_proxy.name().unwrap();
}

#[test]
fn get_default_adapter_uuids() {
    let connection = zbus::Connection::new_system().unwrap();
    let adapter_proxy = niter::adapter::AdapterProxy::new(&connection).unwrap();
    let uuids = adapter_proxy.uuids().unwrap();
    println!("UUIDS: {:?}", uuids);
}

#[test]
fn enumerate_adapters() {
    let connection = zbus::Connection::new_system().unwrap();
    for adapter in niter::adapter::AdapterProxy::enumerate_adapters(&connection).unwrap() {
        println!("{:?}", adapter);
    }
}
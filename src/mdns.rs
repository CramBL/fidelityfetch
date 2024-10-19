use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::{collections::HashMap, net::IpAddr};

const SERVICE_TYPE: &str = "_http._tcp.local.";
const SERVICE_NAME: &str = "fidelityfetch";

pub fn register_mdns(mdns_hostname: &str, port: u16, ip: IpAddr, root_dir: String) {
    let service_props: HashMap<String, String> = HashMap::from([
        ("root".to_owned(), root_dir),
        ("port".to_owned(), port.to_string()),
    ]);

    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        SERVICE_NAME,
        &format!("{mdns_hostname}.local."),
        ip.to_string(),
        port,
        service_props,
    )
    .expect("Failed creating service");

    mdns.register(service_info)
        .expect("Failed to register mDNS service");
}

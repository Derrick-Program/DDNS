use std::net::IpAddr;

pub trait Provider {
    fn update_record(&self, domain: &str, ip: &str) -> Result<(), String>;
}

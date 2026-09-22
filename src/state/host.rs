//! Components and utilities for querying hosts.

use std::net::IpAddr;

/// Marker component for enabled hosts.
pub struct Enabled;

/// Stores the hostname of a host.
pub struct HostName(pub String);

/// The last set credentials for a host.
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Clone)]
pub struct ConnectionInfo {
	pub ip: IpAddr,
	pub port: u16,
}

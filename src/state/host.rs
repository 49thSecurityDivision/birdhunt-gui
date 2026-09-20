//! Components and utilities for querying hosts.

/// Marker component for enabled hosts.
pub struct Enabled;

/// Stores the hostname of a host.
pub struct HostName(pub String);

/// The last set credentials for a host.
pub struct Credentials {
	pub username: String,
	pub password: String,
}

pub struct ConnectionInfo {
	pub ip: String,
	pub port: u16,
}

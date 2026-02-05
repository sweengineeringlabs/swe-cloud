pub mod storage;
pub mod docker;
pub mod mock;

#[cfg(target_os = "windows")]
pub mod hyperv;
#[cfg(target_os = "windows")]
pub mod network;

#[cfg(target_os = "linux")]
pub mod kvm;
#[cfg(target_os = "linux")]
pub mod linux_network;

pub use docker::DockerDriver;
#[cfg(target_os = "windows")]
pub use hyperv::HyperVDriver;
#[cfg(target_os = "windows")]
pub use network::HyperVNetworkDriver;

#[cfg(target_os = "linux")]
pub use kvm::KvmDriver;
#[cfg(target_os = "linux")]
pub use linux_network::LinuxNetworkDriver;

pub use mock::{MockComputeDriver, MockNetworkDriver};
pub use storage::FileSystemStorage;

#[cfg(test)]
mod tests;

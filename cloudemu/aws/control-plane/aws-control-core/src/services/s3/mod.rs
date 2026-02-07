//! S3 Service Implementation

pub mod handlers;
mod service;
mod xml;

pub use service::S3Service;

#[cfg(test)]
mod tests;

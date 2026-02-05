pub mod service;
pub mod handlers;

pub use service::SqsService;

#[cfg(test)]
mod tests;

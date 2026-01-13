pub mod service;
pub mod handlers;

pub use service::DynamoDbService;

#[cfg(test)]
mod tests;

// Pages Module
// Re-exports all page components
// RustScript: foo.page.rsc exports are accessed via @pages::Component

// Dashboard
pub mod dashboard;
pub use dashboard::Dashboard;

// Not Found
pub mod not_found;
pub use not_found::NotFound;

// Workflow Runner
pub mod workflow_runner;
pub use workflow_runner::WorkflowRunner;

// CloudEmu pages
pub mod cloudemu_overview;
pub mod cloudemu_provider;
pub mod cloudemu_service;
pub mod cloudemu_service_new;
pub mod cloudemu_service_detail;
pub mod cloudemu_logs;

pub use cloudemu_overview::CloudemuOverview;
pub use cloudemu_provider::CloudemuProvider;
pub use cloudemu_service::CloudemuService;
pub use cloudemu_service_new::CloudemuServiceNew;
pub use cloudemu_service_detail::CloudemuServiceDetail;
pub use cloudemu_logs::CloudemuLogs;

// CloudKit pages
pub mod cloudkit_overview;
pub mod cloudkit_resources;
pub mod cloudkit_resource_list;
pub mod cloudkit_resource_detail;
pub mod cloudkit_operations;
pub mod cloudkit_explorer;

pub use cloudkit_overview::CloudkitOverview;
pub use cloudkit_resources::CloudkitResources;
pub use cloudkit_resource_list::CloudkitResourceList;
pub use cloudkit_resource_detail::CloudkitResourceDetail;
pub use cloudkit_operations::CloudkitOperations;
pub use cloudkit_explorer::CloudkitExplorer;

// IAC pages
pub mod iac_overview;
pub mod iac_modules;
pub mod iac_module_detail;
pub mod iac_deploy;
pub mod iac_deployments;
pub mod iac_deployment_detail;
pub mod iac_state;
pub mod iac_plans;
pub mod iac_plan_detail;

pub use iac_overview::IacOverview;
pub use iac_modules::IacModules;
pub use iac_module_detail::IacModuleDetail;
pub use iac_deploy::IacDeploy;
pub use iac_deployments::IacDeployments;
pub use iac_deployment_detail::IacDeploymentDetail;
pub use iac_state::IacState;
pub use iac_plans::IacPlans;
pub use iac_plan_detail::IacPlanDetail;

// Settings pages
pub mod settings_index;
pub mod settings_providers;
pub mod settings_endpoints;
pub mod settings_credentials;
pub mod settings_users;

pub use settings_index::SettingsIndex;
pub use settings_providers::SettingsProviders;
pub use settings_endpoints::SettingsEndpoints;
pub use settings_credentials::SettingsCredentials;
pub use settings_users::SettingsUsers;

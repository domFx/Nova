use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}.")]
pub(crate) struct VulkanSuitabilityError(pub &'static str);

#[derive(Debug, Error)]
#[error("{0}.")]
pub(crate) struct VulkanConfigurationError(pub &'static str);
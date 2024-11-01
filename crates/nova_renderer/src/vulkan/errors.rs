use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}.")]
pub(crate) struct VulkanSuitabilityError(pub &'static str);
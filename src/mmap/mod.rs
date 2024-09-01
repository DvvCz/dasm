#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[non_exhaustive]
#[derive(Debug)]
pub enum MmapError {
	Failed
}

pub type MmapResult<T> = Result<T, MmapError>;

impl core::fmt::Display for MmapError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Failed => f.write_str("Failed to create mmap")
		}
	}
}

impl core::error::Error for MmapError {
	fn description(&self) -> &str {
		match self {
			Self::Failed => "Failed to create mmap"
		}
	}
}

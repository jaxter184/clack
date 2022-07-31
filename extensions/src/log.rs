use clack_common::extensions::{Extension, HostExtension};
use clap_sys::ext::log::{clap_host_log, clap_log_severity, CLAP_EXT_LOG};
use std::ffi::CStr;

mod error;
#[cfg(feature = "clack-host")]
pub mod implementation;
pub use error::LogError;

/// How significant a log message is, with `Debug` being the least
/// important and `Fatal` being the most important.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LogSeverity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Fatal = 4,

    HostMisbehaving = 5,
    PluginMisbehaving = 6,
}

impl LogSeverity {
    pub fn from_raw(raw: clap_log_severity) -> Option<Self> {
        use clap_sys::ext::log::*;
        use LogSeverity::*;

        match raw as i32 {
            CLAP_LOG_DEBUG => Some(Debug),
            CLAP_LOG_INFO => Some(Info),
            CLAP_LOG_WARNING => Some(Warning),
            CLAP_LOG_ERROR => Some(Error),
            CLAP_LOG_FATAL => Some(Fatal),
            CLAP_LOG_HOST_MISBEHAVING => Some(HostMisbehaving),
            CLAP_LOG_PLUGIN_MISBEHAVING => Some(PluginMisbehaving),
            _ => None,
        }
    }

    #[inline]
    pub fn to_raw(self) -> clap_log_severity {
        self as _
    }
}

#[repr(C)]
pub struct Log(clap_host_log);

// SAFETY: The API of this extension makes it so that the Send/Sync requirements are enforced onto
// the input handles, not on the descriptor itself.
unsafe impl Send for Log {}
unsafe impl Sync for Log {}

unsafe impl Extension for Log {
    const IDENTIFIER: &'static CStr = CLAP_EXT_LOG;
    type ExtensionType = HostExtension;
}

#[cfg(feature = "clack-plugin")]
mod plugin {
    use super::*;
    use clack_plugin::host::HostHandle;
    use std::ffi::CStr;

    impl Log {
        #[inline]
        pub fn log(&self, host: &HostHandle, log_severity: LogSeverity, message: &CStr) {
            if let Some(log) = self.0.log {
                unsafe { log(host.as_raw(), log_severity.to_raw(), message.as_ptr()) }
            }
        }
    }
}

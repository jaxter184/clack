use crate::host::PluginHoster;
pub use clack_common::extensions::*;
use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// A collection of all extensions supported for a given host type `H`.
///
/// Host can declare the different extensions they support by using the
/// [`register`](HostExtensions::register) method on this struct, during a call to
/// [`declare_extensions`](crate::host::PluginHoster::declare_extensions).
pub struct HostExtensions<'a, H> {
    found: Option<NonNull<c_void>>,
    requested: &'a CStr,
    plugin_type: PhantomData<H>,
}

impl<'a, 'b, H: PluginHoster<'a>> HostExtensions<'a, H> {
    #[inline]
    pub(crate) fn new(requested: &'a CStr) -> Self {
        Self {
            found: None,
            requested,
            plugin_type: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn found(&self) -> *const c_void {
        self.found
            .map(|p| p.as_ptr())
            .unwrap_or(::core::ptr::null_mut())
    }

    /// Adds a given extension implementation to the list of extensions this plugin supports.
    pub fn register<E: ExtensionImplementation<H, ExtensionType = PluginExtension>>(
        &mut self,
    ) -> &mut Self {
        if self.found.is_some() {
            return self;
        }

        let uri = unsafe { CStr::from_ptr(E::IDENTIFIER as *const _) };
        if uri == self.requested {
            self.found = NonNull::new(E::IMPLEMENTATION as *const _ as *mut _)
        }

        self
    }
}
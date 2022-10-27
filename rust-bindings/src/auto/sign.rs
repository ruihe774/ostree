// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2020_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
use crate::Repo;
use glib::object::IsA;
#[cfg(any(feature = "v2020_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
use glib::translate::*;
use std::fmt;
#[cfg(any(feature = "v2020_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
use std::ptr;

glib::wrapper! {
    #[doc(alias = "OstreeSign")]
    pub struct Sign(Interface<ffi::OstreeSign, ffi::OstreeSignInterface>);

    match fn {
        type_ => || ffi::ostree_sign_get_type(),
    }
}

impl Sign {
    pub const NONE: Option<&'static Sign> = None;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_get_all")]
    #[doc(alias = "get_all")]
    pub fn all() -> Vec<Sign> {
        unsafe { FromGlibPtrContainer::from_glib_full(ffi::ostree_sign_get_all()) }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_get_by_name")]
    #[doc(alias = "get_by_name")]
    pub fn by_name(name: &str) -> Result<Sign, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sign_get_by_name(name.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait SignExt: 'static {
    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_add_pk")]
    fn add_pk(&self, public_key: &glib::Variant) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_clear_keys")]
    fn clear_keys(&self) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_commit")]
    fn commit(
        &self,
        repo: &Repo,
        commit_checksum: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_commit_verify")]
    fn commit_verify(
        &self,
        repo: &Repo,
        commit_checksum: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Option<glib::GString>, glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_data")]
    fn data(
        &self,
        data: &glib::Bytes,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<glib::Bytes, glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_data_verify")]
    fn data_verify(
        &self,
        data: &glib::Bytes,
        signatures: &glib::Variant,
    ) -> Result<Option<glib::GString>, glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_load_pk")]
    fn load_pk(&self, options: &glib::Variant) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_metadata_format")]
    fn metadata_format(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_metadata_key")]
    fn metadata_key(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_set_pk")]
    fn set_pk(&self, public_key: &glib::Variant) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_set_sk")]
    fn set_sk(&self, secret_key: &glib::Variant) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    #[doc(alias = "ostree_sign_summary")]
    fn summary(
        &self,
        repo: &Repo,
        keys: &glib::Variant,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error>;
}

impl<O: IsA<Sign>> SignExt for O {
    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn add_pk(&self, public_key: &glib::Variant) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_add_pk(
                self.as_ref().to_glib_none().0,
                public_key.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn clear_keys(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_clear_keys(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn commit(
        &self,
        repo: &Repo,
        commit_checksum: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_commit(
                self.as_ref().to_glib_none().0,
                repo.to_glib_none().0,
                commit_checksum.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn commit_verify(
        &self,
        repo: &Repo,
        commit_checksum: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Option<glib::GString>, glib::Error> {
        unsafe {
            let mut out_success_message = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_commit_verify(
                self.as_ref().to_glib_none().0,
                repo.to_glib_none().0,
                commit_checksum.to_glib_none().0,
                &mut out_success_message,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(out_success_message))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn data(
        &self,
        data: &glib::Bytes,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut signature = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                &mut signature,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(signature))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn data_verify(
        &self,
        data: &glib::Bytes,
        signatures: &glib::Variant,
    ) -> Result<Option<glib::GString>, glib::Error> {
        unsafe {
            let mut out_success_message = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_data_verify(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                signatures.to_glib_none().0,
                &mut out_success_message,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(out_success_message))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::ostree_sign_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn load_pk(&self, options: &glib::Variant) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_load_pk(
                self.as_ref().to_glib_none().0,
                options.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn metadata_format(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ostree_sign_metadata_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn metadata_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ostree_sign_metadata_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn set_pk(&self, public_key: &glib::Variant) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_set_pk(
                self.as_ref().to_glib_none().0,
                public_key.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn set_sk(&self, secret_key: &glib::Variant) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_set_sk(
                self.as_ref().to_glib_none().0,
                secret_key.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_2")))]
    fn summary(
        &self,
        repo: &Repo,
        keys: &glib::Variant,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_sign_summary(
                self.as_ref().to_glib_none().0,
                repo.to_glib_none().0,
                keys.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Sign")
    }
}

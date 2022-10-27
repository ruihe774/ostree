// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RepoDevInoCache(Shared<ffi::OstreeRepoDevInoCache>);

    match fn {
        ref => |ptr| ffi::ostree_repo_devino_cache_ref(ptr),
        unref => |ptr| ffi::ostree_repo_devino_cache_unref(ptr),
        type_ => || ffi::ostree_repo_devino_cache_get_type(),
    }
}

impl RepoDevInoCache {
    #[doc(alias = "ostree_repo_devino_cache_new")]
    pub fn new() -> RepoDevInoCache {
        unsafe { from_glib_full(ffi::ostree_repo_devino_cache_new()) }
    }
}

impl Default for RepoDevInoCache {
    fn default() -> Self {
        Self::new()
    }
}

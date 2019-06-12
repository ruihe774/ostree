use crate::{Repo, RepoCheckoutFilterResult};
use glib::translate::{
    from_glib_borrow, from_glib_none, FromGlibPtrNone, Stash, ToGlib, ToGlibPtr,
};
use glib_sys::gpointer;
use libc::c_char;
use ostree_sys::{OstreeRepo, OstreeRepoCheckoutFilterResult};
use std::path::{Path, PathBuf};

/// A filter callback to decide which files to checkout from a [Repo](struct.Repo.html). The
/// function is called for every directory and file in the dirtree.
///
/// # Arguments
/// * `repo` - the `Repo` that is being checked out
/// * `path` - the path of the current file, as an absolute path rooted at the commit's root. The
///   root directory is '/', a subdir would be '/subdir' etc.
/// * `stat` - the metadata of the current file
///
/// # Return Value
/// The return value determines whether the current file is checked out or skipped.
pub struct RepoCheckoutFilter(Box<dyn Fn(&Repo, &Path, &libc::stat) -> RepoCheckoutFilterResult>);

impl RepoCheckoutFilter {
    /// Wrap a closure for use as a filter function.
    ///
    /// # Return Value
    /// The return value is always `Some` containing the value. It simply comes pre-wrapped for your
    /// convenience.
    pub fn new<F>(closure: F) -> Option<RepoCheckoutFilter>
    where
        F: Fn(&Repo, &Path, &libc::stat) -> RepoCheckoutFilterResult,
        F: 'static,
    {
        Some(RepoCheckoutFilter(Box::new(closure)))
    }

    /// Call the contained closure.
    fn call(&self, repo: &Repo, path: &Path, stat: &libc::stat) -> RepoCheckoutFilterResult {
        self.0(repo, path, stat)
    }
}

impl<'a> ToGlibPtr<'a, gpointer> for RepoCheckoutFilter {
    type Storage = ();

    fn to_glib_none(&'a self) -> Stash<gpointer, Self> {
        Stash(self as *const RepoCheckoutFilter as gpointer, ())
    }
}

impl FromGlibPtrNone<gpointer> for &RepoCheckoutFilter {
    // `ptr` must be valid for the lifetime of the returned reference.
    unsafe fn from_glib_none(ptr: gpointer) -> Self {
        assert!(!ptr.is_null());
        &*(ptr as *const RepoCheckoutFilter)
    }
}

/// Trampoline to be called by libostree that calls the Rust closure in the `user_data` parameter.
///
/// # Safety
/// All parameters must be valid pointers for the runtime of the function. In particular,
/// `user_data` must point to a [RepoCheckoutFilter](struct.RepoCheckoutFilter.html) value.
///
/// # Panics
/// If any parameter is a null pointer, the function panics.
unsafe extern "C" fn filter_trampoline(
    repo: *mut OstreeRepo,
    path: *const c_char,
    stat: *mut libc::stat,
    user_data: gpointer,
) -> OstreeRepoCheckoutFilterResult {
    // TODO: handle unwinding
    // We can't guarantee it's a valid pointer, but we can make sure it's not null.
    assert!(!stat.is_null());
    let stat = &*stat;
    // This reference is valid until the end of this function, which is shorter than the lifetime
    // of `user_data` so we're fine.
    let closure: &RepoCheckoutFilter = from_glib_none(user_data);
    // `repo` lives at least until the end of this function. This means we can just borrow the
    // reference so long as our `repo` is not moved out of this function.
    let repo = from_glib_borrow(repo);
    // This is a copy so no problems here.
    let path: PathBuf = from_glib_none(path);

    let result = closure.call(&repo, &path, stat);
    result.to_glib()
}

/// Returns the trampoline function in a `Some`.
///
/// This is mostly convenient because the full type needs to be written out in fewer places.
pub(super) fn trampoline() -> Option<
    unsafe extern "C" fn(
        *mut OstreeRepo,
        *const c_char,
        *mut libc::stat,
        gpointer,
    ) -> OstreeRepoCheckoutFilterResult,
> {
    Some(filter_trampoline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib::translate::ToGlibPtr;
    use ostree_sys::OSTREE_REPO_CHECKOUT_FILTER_SKIP;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    #[should_panic]
    fn trampoline_should_panic_if_repo_is_nullptr() {
        let path = CString::new("/a/b/c").unwrap();
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        let filter = RepoCheckoutFilter(Box::new(|_, _, _| RepoCheckoutFilterResult::Allow));
        unsafe {
            filter_trampoline(
                ptr::null_mut(),
                path.as_ptr(),
                &mut stat,
                filter.to_glib_none().0,
            );
        }
    }

    #[test]
    #[should_panic]
    fn trampoline_should_panic_if_path_is_nullptr() {
        let repo = Repo::new_default();
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        let filter = RepoCheckoutFilter(Box::new(|_, _, _| RepoCheckoutFilterResult::Allow));
        unsafe {
            filter_trampoline(
                repo.to_glib_none().0,
                ptr::null(),
                &mut stat,
                filter.to_glib_none().0,
            );
        }
    }

    #[test]
    #[should_panic]
    fn trampoline_should_panic_if_stat_is_nullptr() {
        let repo = Repo::new_default();
        let path = CString::new("/a/b/c").unwrap();
        let filter = RepoCheckoutFilter(Box::new(|_, _, _| RepoCheckoutFilterResult::Allow));
        unsafe {
            filter_trampoline(
                repo.to_glib_none().0,
                path.as_ptr(),
                ptr::null_mut(),
                filter.to_glib_none().0,
            );
        }
    }

    #[test]
    #[should_panic]
    fn trampoline_should_panic_if_user_data_is_nullptr() {
        let repo = Repo::new_default();
        let path = CString::new("/a/b/c").unwrap();
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        unsafe {
            filter_trampoline(
                repo.to_glib_none().0,
                path.as_ptr(),
                &mut stat,
                ptr::null_mut(),
            );
        }
    }

    #[test]
    fn trampoline_should_call_the_closure() {
        let repo = Repo::new_default();
        let path = CString::new("/a/b/c").unwrap();
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        let filter = {
            let repo = repo.clone();
            let path = path.clone();
            RepoCheckoutFilter(Box::new(move |arg_repo, arg_path, _| {
                assert_eq!(arg_repo, &repo);
                assert_eq!(&CString::new(arg_path.to_str().unwrap()).unwrap(), &path);
                RepoCheckoutFilterResult::Skip
            }))
        };
        let result = unsafe {
            filter_trampoline(
                repo.to_glib_none().0,
                path.as_ptr(),
                &mut stat,
                filter.to_glib_none().0,
            )
        };
        assert_eq!(result, OSTREE_REPO_CHECKOUT_FILTER_SKIP);
    }
}

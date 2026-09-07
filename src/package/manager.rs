#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Package Management for SigmaOS
/// Based on Roadmap Item: Package Management + Reproducible Builds
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type PackageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageState {
    Installed = 0,
    Available = 1,
    Updating = 2,
    Corrupted = 3,
}

pub trait Package {
    fn id(&self) -> PackageID;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn state(&self) -> PackageState;
}

#[repr(C)]
pub struct SimplePackage {
    pub id: PackageID,
    pub name: [u8; 64],
    pub version: [u8; 32],
    pub name_len: u8,
    pub version_len: u8,
    pub state: AtomicUsize,
}

impl SimplePackage {
    pub fn new(id: PackageID, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut version_array = [0u8; 32];
        let name_len = name.len().min(63);
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(
                version.as_ptr(),
                version_array.as_mut_ptr(),
                version_len,
            );
        }
        SimplePackage {
            id,
            name: name_array,
            version: version_array,
            name_len: name_len as u8,
            version_len: version_len as u8,
            state: AtomicUsize::new(PackageState::Available as usize),
        }
    }
}

impl Package for SimplePackage {
    fn id(&self) -> PackageID {
        self.id
    }
    fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }
    fn version(&self) -> &[u8] {
        &self.version[..self.version_len as usize]
    }
    fn state(&self) -> PackageState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst) as u32) }
    }
}

pub trait PackageManager {
    fn install(&mut self, package: Box<dyn Package>) -> Result<PackageID, PackageError>;
    fn uninstall(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn update(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn get_package(&self, id: PackageID) -> Option<&dyn Package>;
}

/// GUI App Store Manager (GNOME Software / KDE Discover Parity)
#[derive(Debug, Clone)]
pub struct AppReview {
    pub author: std::string::String,
    pub rating_stars: u8,
    pub comment: std::string::String,
}

#[derive(Debug, Clone)]
pub struct AppListing {
    pub app_id: std::string::String,
    pub display_name: std::string::String,
    pub category: std::string::String,
    pub description: std::string::String,
    pub average_rating: f32,
    pub reviews: std::vec::Vec<AppReview>,
    pub is_installed: bool,
}

pub struct GuiAppStoreManager {
    pub store_listings: std::vec::Vec<AppListing>,
}

impl GuiAppStoreManager {
    pub fn new() -> Self {
        GuiAppStoreManager {
            store_listings: std::vec::Vec::new(),
        }
    }

    pub fn publish_app(
        &mut self,
        app_id: &str,
        display_name: &str,
        category: &str,
        description: &str,
    ) {
        let listing = AppListing {
            app_id: std::string::String::from(app_id),
            display_name: std::string::String::from(display_name),
            category: std::string::String::from(category),
            description: std::string::String::from(description),
            average_rating: 5.0,
            reviews: std::vec::Vec::new(),
            is_installed: false,
        };
        self.store_listings.push(listing);
    }

    pub fn add_review(
        &mut self,
        app_id: &str,
        author: &str,
        rating_stars: u8,
        comment: &str,
    ) -> Result<(), &'static str> {
        let stars = rating_stars.clamp(1, 5);
        for app in &mut self.store_listings {
            if app.app_id == app_id {
                app.reviews.push(AppReview {
                    author: std::string::String::from(author),
                    rating_stars: stars,
                    comment: std::string::String::from(comment),
                });
                let total_stars: u32 = app.reviews.iter().map(|r| r.rating_stars as u32).sum();
                app.average_rating = total_stars as f32 / app.reviews.len() as f32;
                return Ok(());
            }
        }
        Err("App ID not found in store registry")
    }

    pub fn search_apps(&self, keyword: &str) -> std::vec::Vec<&AppListing> {
        self.store_listings
            .iter()
            .filter(|app| {
                app.display_name.contains(keyword)
                    || app.description.contains(keyword)
                    || app.category.contains(keyword)
            })
            .collect()
    }

    pub fn install_gui_app(&mut self, app_id: &str) -> Result<(), &'static str> {
        for app in &mut self.store_listings {
            if app.app_id == app_id {
                app.is_installed = true;
                return Ok(());
            }
        }
        Err("App ID not found in store")
    }
}

impl Default for GuiAppStoreManager {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageError {
    Success = 0,
    PackageNotFound = 1,
    InstallFailed = 2,
    UpdateFailed = 3,
}

pub struct SimplePackageManager {
    packages: Vec<Option<Box<dyn Package>>>,
    next_id: AtomicUsize,
}

impl SimplePackageManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimplePackageManager {
            packages: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PackageManager for SimplePackageManager {
    fn install(&mut self, package: Box<dyn Package>) -> Result<PackageID, PackageError> {
        let id = package.id();
        self.packages.push(Some(package));
        Ok(id)
    }
    fn uninstall(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id {
                    self.packages.clear();
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    fn update(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref mut pkg) = *pkg_option {
                if pkg.id() == id {
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    fn get_package(&self, id: PackageID) -> Option<&dyn Package> {
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id {
                    return Some(pkg.as_ref());
                }
            }
        }
        None
    }
}

pub trait Repository {
    fn add_package(&mut self, package: Box<dyn Package>) -> Result<(), PackageError>;
    fn remove_package(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn list_packages(&self) -> Vec<PackageID>;
}

pub struct SimpleRepository {
    packages: Vec<Option<Box<dyn Package>>>,
}

impl SimpleRepository {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleRepository {
            packages: Vec::new(),
        }
    }
}

impl Repository for SimpleRepository {
    fn add_package(&mut self, package: Box<dyn Package>) -> Result<(), PackageError> {
        self.packages.push(Some(package));
        Ok(())
    }
    fn remove_package(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id {
                    self.packages.clear();
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    fn list_packages(&self) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                ids.push(pkg.id());
            }
        }
        ids
    }
}

struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clear(&mut self) {
        self.len = 0;
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_package_caching() {
        let pkg = SimplePackage::new(1, b"curl", b"8.1.0");
        assert_eq!(pkg.id(), 1);
        assert_eq!(pkg.name(), b"curl");
        assert_eq!(pkg.version(), b"8.1.0");
        assert_eq!(pkg.name_len, 4);
        assert_eq!(pkg.version_len, 5);
    }

    #[test]
    fn test_gui_app_store_manager() {
        let mut store = GuiAppStoreManager::new();
        store.publish_app(
            "org.gimp.GIMP",
            "GIMP Image Editor",
            "Graphics",
            "Professional photo manipulation software",
        );
        assert_eq!(store.store_listings.len(), 1);

        assert!(store
            .add_review(
                "org.gimp.GIMP",
                "Alice",
                5,
                "Amazing open source photo editor!"
            )
            .is_ok());
        assert!(store
            .add_review("org.gimp.GIMP", "Bob", 3, "Good but complex UI")
            .is_ok());

        let app = &store.store_listings[0];
        assert_eq!(app.average_rating, 4.0);

        let results = store.search_apps("Image");
        assert_eq!(results.len(), 1);

        assert!(store.install_gui_app("org.gimp.GIMP").is_ok());
        assert!(store.store_listings[0].is_installed);
    }
}

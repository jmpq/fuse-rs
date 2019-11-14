//! Low-level filesystem.
//!
//! A filesystem based on inodes, which is also the kernel's internal representation.

mod argument;

mod attr;
pub use attr::{FileAttr, FileAttrTryFromError, FileType, FileTypeTryFromError};

mod channel;
pub(crate) use channel::{Channel, unmount};

mod filesystem;
pub use filesystem::{Filesystem, Result};

pub mod reply;

mod request;
pub use request::{Operation, Request, RequestError};

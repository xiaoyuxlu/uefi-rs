//! This module is used to simplify importing the most common UEFI types.
//!
//! This includes the system table types, `Status` codes, etc.

// Bring traits into scope
pub use crate::data_types::chars::Character;
pub use crate::result::{ResultExt, ResultExt2};

// Import frequently used types.
pub use crate::table::boot::BootServices;
pub use crate::table::runtime::RuntimeServices;
pub use crate::table::{Boot, Runtime, SystemTable};
pub use crate::Status;

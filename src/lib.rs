/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate rusqlite;

mod catalog;
mod collections;
mod folders;
mod images;
mod keywords;
mod keywordtree;
mod stack;

use thiserror::Error;

pub use catalog::{Catalog, CatalogVersion};
pub use collections::{Collection, CollectionType};
pub use folders::{Folder, Folders};
pub use images::Image;
pub use keywords::Keyword;
pub use keywordtree::KeywordTree;
pub use stack::Stack;

pub type CoId = i64;

#[derive(Error, Debug)]
pub enum Error {
    /// No database open.
    #[error("No database.")]
    NoDatabase,
    /// Unimplemented
    #[error("Unimplemented.")]
    Unimplemented,
    /// Unsupported catalog version.
    #[error("LrCat: Unsupported catalog version.")]
    UnsupportedVersion,
    /// Sql Error.
    #[error("Co: SQL error: {0}.")]
    Sql(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

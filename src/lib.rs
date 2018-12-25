/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate rusqlite;

mod catalog;
mod folders;
mod keywords;
mod keywordtree;

pub use catalog::{Catalog, CatalogVersion};
pub use folders::{Folder, Folders};
pub use keywords::{Keyword};
pub use keywordtree::{KeywordTree};

pub struct Collection {}
pub struct Image {}

pub type CoId = i64;

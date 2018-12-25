extern crate rusqlite;

mod catalog;
mod keywords;
mod keywordtree;

pub use catalog::{Catalog, CatalogVersion};
pub use keywords::{Keyword};
pub use keywordtree::{KeywordTree};

pub struct Collection {}
pub struct Image {}
pub struct Folders {}

pub type CoId = i64;

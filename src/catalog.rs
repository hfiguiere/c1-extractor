/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use super::CoId;
use super::{Collection, Folder, Folders, Image, Keyword, KeywordTree, Stack};

const DB_FILENAME: &str = "Capture One Catalog.cocatalogdb";

#[derive(Debug, PartialEq)]
pub enum CatalogVersion {
    Unknown,
    Co11,
    Co12,
}

impl Default for CatalogVersion {
    fn default() -> CatalogVersion {
        CatalogVersion::Unknown
    }
}

impl From<i32> for CatalogVersion {
    fn from(val: i32) -> Self {
        match val {
            1200 => CatalogVersion::Co12,
            1106 => CatalogVersion::Co11,
            _ => CatalogVersion::Unknown,
        }
    }
}

#[derive(Default)]
pub struct Catalog {
    /// Catalog path
    path: PathBuf,
    db_only: bool,
    pub version: i32,
    pub catalog_version: CatalogVersion,
    pub root_collection_id: CoId,

    /// The keywords, mapped in the local `CoId`
    keywords: BTreeMap<CoId, Keyword>,
    /// The folders (path location)
    folders: Folders,
    /// The collections
    collections: Vec<Collection>,
    /// Images
    images: Vec<Image>,
    /// Stacks
    stacks: Vec<Stack>,
    /// The entities
    entities_id_to_name: HashMap<CoId, String>,
    entities_name_to_id: HashMap<String, CoId>,
    /// The sqlite connection to the catalog
    dbconn: Option<Connection>,
}

impl Catalog {
    pub fn new(path: &Path) -> Self {
        let mut catalog = Catalog::default();
        catalog.path = PathBuf::from(path);
        catalog
    }

    pub fn open(&mut self) -> bool {
        let mut db_path = self.path.clone();
        self.db_only = !self.path.is_dir();
        if !self.db_only {
            db_path.push(DB_FILENAME);
        }
        let conn_attempt = Connection::open(&db_path);
        self.dbconn = conn_attempt.ok();
        self.dbconn.is_some()
    }

    pub fn load_version(&mut self) {
        if let Some(conn) = self.dbconn.as_ref() {
            if let Ok(mut stmt) =
                conn.prepare("SELECT ZVERSION FROM ZVERSIONINFO ORDER BY Z_PK DESC")
            {
                let mut rows = stmt.query(&[]).unwrap();
                if let Some(Ok(row)) = rows.next() {
                    self.version = row.get(0);
                    self.catalog_version = CatalogVersion::from(self.version);
                }
            }
            if self.catalog_version != CatalogVersion::Unknown {
                if let Ok(mut stmt) = conn.prepare("SELECT Z_ENT, ZNAME FROM ZENTITIES") {
                    let mut rows = stmt.query(&[]).unwrap();
                    while let Some(Ok(row)) = rows.next() {
                        let ent: CoId = row.get(0);
                        let name: String = row.get(1);
                        self.entities_id_to_name.insert(ent, name.clone());
                        self.entities_name_to_id.insert(name, ent);
                    }
                }
                if let Ok(mut stmt) = conn.prepare("SELECT ZROOTCOLLECTION FROM ZDOCUMENTCONTENT") {
                    let mut rows = stmt.query(&[]).unwrap();
                    if let Some(Ok(row)) = rows.next() {
                        self.root_collection_id = row.get(0);
                    }
                }
            }
        }
    }

    pub fn load_keywords_tree(&mut self) -> KeywordTree {
        let keywords = self.load_keywords();

        let mut tree = KeywordTree::new();
        let keyword = Keyword::default();
        tree.add_child(&keyword);
        tree.add_children(keywords);

        tree
    }

    pub fn load_keywords(&mut self) -> &BTreeMap<CoId, Keyword> {
        if self.keywords.is_empty() {
            if let Some(ref conn) = self.dbconn {
                if let Some(entity) = self.entities_name_to_id.get("Keyword") {
                    if let Ok(mut stmt) =
                        conn.prepare("SELECT Z_PK, ZNAME, ZPARENT FROM ZKEYWORD WHERE Z_ENT=?1")
                    {
                        let mut rows = stmt.query(&[entity]).unwrap();
                        while let Some(Ok(row)) = rows.next() {
                            let name: String = row.get(1);
                            let keyword =
                                Keyword::new(row.get(0), &name, row.get_checked(2).unwrap_or(0));
                            self.keywords.insert(keyword.id(), keyword);
                        }
                    }
                }
            }
        }
        &self.keywords
    }

    pub fn load_folders(&mut self) -> &Folders {
        if self.folders.is_empty() {
            if let Some(ref conn) = self.dbconn {
                if let Some(entity) = self.entities_name_to_id.get("PathLocation") {
                    self.folders = Folder::load_objects(&conn, *entity);
                }
            }
        }
        &self.folders
    }

    pub fn load_collections(&mut self) -> &Vec<Collection> {
        if self.collections.is_empty() {
            if let Some(ref conn) = self.dbconn {
                self.collections = Collection::load_objects(conn, &self.entities_id_to_name);
            }
        }
        &self.collections
    }

    pub fn load_images(&mut self) -> &Vec<Image> {
        if self.images.is_empty() {
            if let Some(ref conn) = self.dbconn {
                if let Some(entity) = self.entities_name_to_id.get("Image") {
                    self.images = Image::load_objects(conn, *entity);
                }
            }
        }
        &self.images
    }

    pub fn load_stacks(&mut self) -> &Vec<Stack> {
        if self.stacks.is_empty() {
            if let Some(ref conn) = self.dbconn {
                if let Some(entity) = self.entities_name_to_id.get("Stack") {
                    self.stacks = Stack::load_objects(conn, *entity);
                }
            }
        }
        &self.stacks
    }
}

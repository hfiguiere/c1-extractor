/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

const DB_FILENAME: &str = "Capture One Catalog.cocatalogdb";

#[derive(Debug, PartialEq)]
pub enum CatalogVersion {
    Unknown,
    Co12
}

impl Default for CatalogVersion {
    fn default() -> CatalogVersion {
        CatalogVersion::Unknown
    }
}

#[derive(Default)]
pub struct Catalog {
    /// Catalog path
    path: PathBuf,
    pub version: i32,
    pub catalog_version: CatalogVersion,
    pub root_collection_id: i64,

    /// The entities
    entities: HashMap<i32, String>,
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
        db_path.push(DB_FILENAME);
        let conn_attempt = Connection::open(&db_path);
        if let Ok(conn) = conn_attempt {
            self.dbconn = Some(conn);

            return true;
        }

        false
    }

    pub fn load_version(&mut self) {
        if let Some(conn) = self.dbconn.as_ref() {
            if let Ok(mut stmt) = conn.prepare("SELECT ZVERSION FROM ZVERSIONINFO ORDER BY Z_PK DESC") {
                let mut rows = stmt.query(&[]).unwrap();
                if let Some(Ok(row)) = rows.next() {
                    self.version = row.get(0);
                    self.catalog_version = match self.version {
                        1200 => CatalogVersion::Co12,
                        _ => CatalogVersion::Unknown,
                    }
                }
            }
            if self.catalog_version != CatalogVersion::Unknown {
                if let Ok(mut stmt) = conn.prepare("SELECT Z_ENT, ZNAME FROM ZENTITIES") {
                    let mut rows = stmt.query(&[]).unwrap();
                    while let Some(Ok(row)) = rows.next() {
                        let ent: i32 = row.get(0);
                        let name: String = row.get(1);
                        self.entities.insert(ent, name);
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
}

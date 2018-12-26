/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite;

use super::CoId;

pub type Folders = Vec<Folder>;

#[derive(Default)]
pub struct Folder {
    id: CoId,
    /// Indicate the path is relative to the catalog.
    pub is_relative: bool,
    /// Path from the `root_folder`
    pub path_from_root: String,
    /// Path of `root_folder`
    pub root_folder: String,
}

impl Folder {
    pub fn id(&self) -> CoId {
        self.id
    }

    pub fn load_objects(conn: &rusqlite::Connection, entity: CoId) -> Folders {
        let mut folders: Folders = vec![];

        if let Ok(mut stmt) = conn.prepare(
            "SELECT Z_PK, ZMACROOT, ZRELATIVEPATH, ZISRELATIVE FROM ZPATHLOCATION WHERE Z_ENT=?1",
        ) {
            let mut rows = stmt.query(&[&entity]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                folders.push(Folder {
                    id: row.get(0),
                    is_relative: row.get(3),
                    path_from_root: row.get(2),
                    root_folder: row.get(1),
                });
            }
        }
        folders
    }
}

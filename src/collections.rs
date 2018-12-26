/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite;
use std::collections::HashMap;
use std::fmt;

use super::CoId;

#[derive(Debug)]
pub enum CollectionType {
    Invalid,
    /// AlbumCollection entity (ZNAME)
    Album(String),
    /// VirtualFolderCollection entity (ZNAME)
    VirtualFolder(String),
    Project,
    CatalogAll,
    Trash,
    CatalogInternalImages,
    /// CatalogFolderCollection (ZFOLDERLOCATION on ZPATHLOCATION `Folder`)
    Folder(CoId),
}

impl Default for CollectionType {
    fn default() -> Self {
        CollectionType::Invalid
    }
}

impl fmt::Display for CollectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CollectionType::Album(ref s) => f.pad(format!("Alb: \"{}\"", s).as_str()),
            CollectionType::VirtualFolder(ref s) => f.pad(format!("VF: \"{}\"", s).as_str()),
            CollectionType::Project => f.pad("root"),
            CollectionType::CatalogAll => f.pad("All Images"),
            CollectionType::Trash => f.pad("Trash"),
            CollectionType::CatalogInternalImages => f.pad("All catalog images"),
            CollectionType::Folder(id) => f.pad(format!("Path folder: {}", id).as_str()),
            _ => f.pad("Invalid"),
        }
    }
}

pub struct Collection {
    pub id: CoId,
    pub collection_type: CollectionType,
    pub parent: CoId,
}

impl Collection {
    pub fn load_objects(
        conn: &rusqlite::Connection,
        entities: &HashMap<CoId, String>,
    ) -> Vec<Collection> {
        let mut collections: Vec<Collection> = vec![];

        if let Ok(mut stmt) =
            conn.prepare("SELECT Z_ENT, Z_PK, ZNAME, ZPARENT, ZFOLDERLOCATION FROM ZCOLLECTION")
        {
            let mut rows = stmt.query(&[]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                let entity: CoId = row.get(0);
                if let Some(entity_name) = entities.get(&entity) {
                    let id: CoId = row.get(1);
                    let parent: CoId = row.get_checked(3).unwrap_or(0);
                    let collection_type = match entity_name.as_str() {
                        "ProjectCollection" => CollectionType::Project,
                        "CatalogAllImagesCollection" => CollectionType::CatalogAll,
                        "CatalogInternalImagesCollection" => CollectionType::CatalogInternalImages,
                        "TrashCollection" => CollectionType::Trash,
                        "AlbumCollection" => CollectionType::Album(row.get(2)),
                        "CatalogFolderCollection" => CollectionType::Folder(row.get(4)),
                        "VirtualFolderCollection" => CollectionType::VirtualFolder(row.get(2)),
                        _ => {
                            println!("Unhandled entity {}", entity_name.as_str());
                            continue;
                        }
                    };
                    collections.push(Collection {
                        id,
                        collection_type,
                        parent,
                    });
                }
            }
        }

        collections
    }
}

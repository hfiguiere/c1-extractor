/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use super::CoId;

#[derive(Default)]
pub struct Stack {
    pub id: CoId,
    pub collection: CoId,
    pub pick: CoId,
    /// Content: id of images. None mean it hasn't been loaded.
    pub content: Option<Vec<CoId>>,
}

impl Stack {
    pub fn load_objects(conn: &rusqlite::Connection, entity: CoId) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = vec![];
        if let Ok(mut stmt) =
            conn.prepare("SELECT Z_PK, ZCOLLECTION, ZPICKEDIMAGE FROM ZSTACK WHERE Z_ENT=?1")
        {
            let mut rows = stmt.query([&entity]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                stacks.push(Stack {
                    id: row.get(0).unwrap(),
                    collection: row.get(1).unwrap(),
                    pick: row.get(2).unwrap(),
                    content: None,
                });
            }
        }

        stacks
    }

    pub fn get_content(&mut self, conn: &rusqlite::Connection) {
        let mut ids: Vec<CoId> = vec![];
        if let Ok(mut stmt) = conn.prepare("SELECT ZIMAGE FROM ZSTACKIMAGELINK WHERE ZSTACK=?1") {
            let mut rows = stmt.query([&self.id]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                ids.push(row.get(0).unwrap());
            }
            self.content = Some(ids);
        }
    }
}

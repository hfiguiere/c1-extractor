/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite;
use super::CoId;

#[derive(Default)]
pub struct Stack {
    pub id: CoId,
    pub collection: CoId,
    pub pick: CoId,
}


impl Stack {

    pub fn load_objects(conn: &rusqlite::Connection, entity: CoId) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = vec![];
        if let Ok(mut stmt) = conn.prepare("SELECT Z_PK, ZCOLLECTION, ZPICKEDIMAGE FROM ZSTACK WHERE Z_ENT=?1") {
            let mut rows = stmt.query(&[&entity]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                stacks.push(Stack {
                    id: row.get(0),
                    collection: row.get(1),
                    pick: row.get(2),
                });
            }
        }

        stacks
    }
}

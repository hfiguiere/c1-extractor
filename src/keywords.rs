/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use super::CoId;

#[derive(Default, Debug)]
pub struct Keyword {
    id: CoId,
    pub name: String,
    pub parent: CoId,
}

impl Keyword {
    pub fn new(id: CoId, name: &str, parent: CoId) -> Self {
        Keyword {
            id,
            name: name.to_string(),
            parent,
        }
    }

    pub fn id(&self) -> CoId {
        self.id
    }
}

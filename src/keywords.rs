
use super::CoId;

#[derive(Default, Debug)]
pub struct Keyword {
    id: CoId,
    pub name: String,
    pub parent: CoId,
}

impl Keyword {
    pub fn new(id: CoId, name: &str, parent: CoId) -> Self {
        Keyword{id, name: name.to_string(), parent}
    }

    pub fn id(&self) -> CoId {
        self.id
    }
}

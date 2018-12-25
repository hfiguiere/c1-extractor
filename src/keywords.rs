
use super::CoId;

#[derive(Default, Debug)]
pub struct Keyword {
    pub id: CoId,
    pub name: String,
    pub parent: CoId,
}

impl Keyword {

    pub fn id(&self) -> CoId {
        self.id
    }
}

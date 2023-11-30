use uuid::Uuid;
use crate::person::PersonId;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct GroupId(Uuid);

pub struct Group {
    pub id: GroupId,
    title: String,
    people: HashSet<PersonId>,
}

impl Group {
    pub fn new(title: &str) -> Self {
        Self {
            id: GroupId(Uuid::new_v4()),
            title: title.to_string(),
            people: HashSet::new(),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_people(&self) -> &HashSet<PersonId> {
        &self.people
    }

    pub fn add_person(&mut self, person: PersonId) {
        self.people.insert(person);
    }
}

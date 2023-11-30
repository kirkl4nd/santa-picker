use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct PersonId(Uuid);

pub struct Person {
    pub id: PersonId,
    name: String,
    email: String,
}

impl Person {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            id: PersonId(Uuid::new_v4()),
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

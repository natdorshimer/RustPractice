use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!(
            "User(id: {}, first_name: {}, last_name: {})",
            self.id.to_string(),
            self.first_name,
            self.last_name
        )
    }
}

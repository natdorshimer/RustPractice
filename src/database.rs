use crate::user::User;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

pub trait UserDao {
    fn find_user_by_id(&self, id: Uuid) -> Option<User>;

    fn find_all_by_first_name(&self, first_name: &str) -> Vec<User>;

    fn create_user(&self, first_name: &str, last_name: &str) -> User;

    fn find_all_users(&self) -> Vec<User>;

    fn delete_user(&self, uuid: Uuid) -> Result<User, &str>;
}

pub struct MemoryUserDao {
    users: Rc<RefCell<HashMap<Uuid, User>>>,
}

impl MemoryUserDao {
    pub fn new() -> MemoryUserDao {
        MemoryUserDao {
            users: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl UserDao for MemoryUserDao {
    fn find_user_by_id(&self, id: Uuid) -> Option<User> {
        self.users.borrow_mut().get(&id).cloned()
    }

    fn create_user(&self, first_name: &str, last_name: &str) -> User {
        let user = User {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };
        let id = user.id;
        self.users.borrow_mut().insert(id, user);
        self.find_user_by_id(id).expect("Could not find user by id")
    }

    fn find_all_by_first_name(&self, first_name: &str) -> Vec<User> {
        self.users
            .borrow_mut()
            .values()
            .filter(|u| u.first_name == first_name)
            .cloned()
            .collect()
    }

    fn find_all_users(&self) -> Vec<User> {
        self.users.borrow_mut().values().cloned().collect()
    }

    fn delete_user(&self, uuid: Uuid) -> Result<User, &str> {
        self.users.borrow_mut().remove(&uuid).ok_or("Nothing")
    }
}

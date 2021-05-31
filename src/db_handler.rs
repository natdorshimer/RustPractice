use crate::database::UserDao;
use crate::input_handler::{InputHandler, ProgramState, StdIoInputHandler};

pub struct ConsoleDbHandler<T: UserDao> {
    user_dao: T,
    input_handler: StdIoInputHandler,
}

impl<T: UserDao> ConsoleDbHandler<T> {
    pub fn new(user_dao: T) -> ConsoleDbHandler<T> {
        ConsoleDbHandler {
            user_dao,
            input_handler: StdIoInputHandler {},
        }
    }

    pub fn get_user_by_id(&self) {
        println!("Please enter a valid uuid");
        match self.input_handler.get_id_from_user() {
            Ok(uuid) => {
                match self.user_dao.find_user_by_id(uuid) {
                    Some(usr) => println!("{}", usr.to_string()),
                    None => println!("Unable to find a user with that UUID"),
                };
            }
            Err(err) => {
                println!("Unable to parse a valid UUID. Try again");
                println!("{}", err.to_string());
            }
        };
    }

    pub fn create_user(&self) {
        println!("Please enter the first name");
        let first_name = self.input_handler.get_line();
        println!("Please enter the last name");
        let last_name = self.input_handler.get_line();
        match (first_name, last_name) {
            (Ok(first_name), Ok(last_name)) => {
                self.user_dao.create_user(&first_name, &last_name);
            }
            (_, _) => {
                println!("Error while processing inputs. Could not create user.");
            }
        };
    }

    pub fn view_all(&self) {
        let users_as_str = self
            .user_dao
            .find_all_users()
            .iter()
            .map(|u| u.to_string())
            .reduce(|acc, curr| format!("{}\n{}", acc, curr))
            .unwrap_or_else(|| "No users found".to_string());
        println!("{}", users_as_str);
    }

    pub fn prompt_for_action(&self) -> ProgramState {
        self.input_handler.prompt_for_action()
    }

    pub fn find_by_first_name(&self) {
        println!("Please enter the first name");
        if let Ok(first_name) = self.input_handler.get_line() {
            let users_as_str = self
                .user_dao
                .find_all_by_first_name(&first_name)
                .iter()
                .map(|u| u.to_string())
                .reduce(|acc, curr| format!("{}\n{}", acc, curr))
                .unwrap_or_else(|| "No users found".to_string());
            println!("{}", users_as_str);
        } else {
            println!("Unable to parse the input");
        }
    }

    pub fn delete_user(&self) {
        println!("Please enter the uuid of the entry to delete");
        if let Ok(uuid) = self.input_handler.get_id_from_user() {
            match self.user_dao.delete_user(uuid) {
                Ok(_) => println!("Successfully deleted user"),
                Err(msg) => println!("No user was found with that uuid. Error: {}", msg),
            }
        } else {
            println!("Please enter a valid UUID.");
        }
    }
}

use std::io;
use uuid::Uuid;

pub trait InputHandler {
    fn get_id_from_user(&self) -> Result<Uuid, String>;
    fn prompt_for_action(&self) -> ProgramState;
}

pub struct StdIoInputHandler {}

impl StdIoInputHandler {
    pub fn get_line(&self) -> Result<String, io::Error> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map(|_| input.trim_end().to_string())
    }
}

pub enum ProgramState {
    CreateUser,
    GetUser,
    ViewAll,
    DeleteUser,
    FindByFirstName,
    Exit,
}

impl InputHandler for StdIoInputHandler {
    fn get_id_from_user(&self) -> Result<Uuid, String> {
        match self.get_line() {
            Ok(user_input) => Uuid::parse_str(&user_input).map_err(|err| err.to_string()),
            Err(msg) => Err(msg.to_string()),
        }
    }

    fn prompt_for_action(&self) -> ProgramState {
        println!("Continue? (1: Create User, 2: Delete User, 3: Get User, 4: View All, 5: Find By First Name, else: Exit");
        match self
            .get_line()
            .unwrap_or_else(|_| "exit".to_string())
            .as_str()
        {
            "1" => ProgramState::CreateUser,
            "2" => ProgramState::DeleteUser,
            "3" => ProgramState::GetUser,
            "4" => ProgramState::ViewAll,
            "5" => ProgramState::FindByFirstName,
            _ => ProgramState::Exit,
        }
    }
}

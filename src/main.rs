mod database;
mod db_handler;
mod input_handler;
mod user;

use database::MemoryUserDao;
use db_handler::ConsoleDbHandler;
use input_handler::ProgramState;

fn main() {
    let console_db_module = ConsoleDbHandler::new(MemoryUserDao::new());

    loop {
        match console_db_module.prompt_for_action() {
            ProgramState::GetUser => console_db_module.get_user_by_id(),
            ProgramState::CreateUser => console_db_module.create_user(),
            ProgramState::FindByFirstName => console_db_module.find_by_first_name(),
            ProgramState::ViewAll => console_db_module.view_all(),
            ProgramState::DeleteUser => console_db_module.delete_user(),
            ProgramState::Exit => break,
        }
        continue;
    }
}

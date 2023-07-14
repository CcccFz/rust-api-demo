pub struct User {
    pub user_id: u64,
    pub user_name: String
}

pub static mut USERS: Vec<User> = Vec::new();

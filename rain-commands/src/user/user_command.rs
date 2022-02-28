pub struct CreateUserCommand {
    pub user_id: u64,
    pub first_name: String,
    pub last_name: String,
}

pub struct UpdateUserCommand {
    pub user_id: u64,
    pub first_name: String,
    pub last_name: String,
}

pub struct CreateUserAddressCommand {
    pub city: String,
    pub post_code: u32,
}
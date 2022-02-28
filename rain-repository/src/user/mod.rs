mod user_address_read_repository;
mod user_address_write_repository;

pub use self::user_address_read_repository::{IUserAddressReadRepository, UserAddressReadRepository};
pub use self::user_address_write_repository::{IUserAddressWriteRepository, UserAddressWriteRepository};

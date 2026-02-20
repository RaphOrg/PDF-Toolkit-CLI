pub mod crypto;
pub mod metadata;

pub use crypto::{decrypt, encrypt, is_encrypted_file};
pub use metadata::{get_info_dict, get_info_value, set_info_value};

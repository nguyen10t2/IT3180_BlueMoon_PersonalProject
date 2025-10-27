use argon2::{
    password_hash::{rand_core::OsRng, SaltString}, 
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(&password.as_bytes(), &salt).unwrap().to_string();
    hashed_password
}

pub fn verify_password(hashed_password: String, password: String) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let hashed_password = PasswordHash::new(&hashed_password).unwrap();
    argon2.verify_password(&password.as_bytes(), &hashed_password)
}
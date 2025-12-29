pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub login_count: u64,
}

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    // OWNERSHIP: 'new' creates and returns OWNERSHIP of a UserManager
    pub fn new() -> Self {
        UserManager { users: Vec::new() }
    }

    // OWNERSHIP (Move): Takes ownership of a User struct to store it.
    // The caller gives up control of 'user'.
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    // IMMUTABLE BORROW (&self): Reads data, doesn't modify.
    pub fn get_user_by_id(&self, id: u64) -> Option<&User> {
        // Returns a REFERENCE to a user inside the vector.
        // The caller can look at the user, but not change it.
        self.users.iter().find(|u| u.id == id)
    }

    // MUTABLE BORROW (&mut self): Modifies the internal state.
    pub fn increment_login_count(&mut self, id: u64) -> bool {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == id) {
            user.login_count += 1;
            return true;
        }
        false
    }

    // OWNERSHIP (Consuming): 'self' (not &self) means this method consumes the manager.
    // After calling this, the UserManager is destroyed and cannot be used.
    pub fn close_system(self) {
        println!("Closing system. Goodbye to {} users.", self.users.len());
        // 'self' is dropped here, freeing all users.
    }
}

// Factory function
pub fn create_default_user(id: u64, name: &str) -> User {
    User {
        id,
        username: String::from(name),
        email: format!("{}@example.com", name),
        login_count: 0,
    }
}

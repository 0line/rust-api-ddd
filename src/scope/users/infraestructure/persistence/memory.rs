use std::sync::RwLock;
use crate::scope::users::domain::user::User;
use crate::scope::users::domain::user_repository::UserRepository;
// users: RwLock<Vec<User>>,
pub struct MemoryRepository {
    users: RwLock<std::collections::HashMap<String, User>>,
}


impl UserRepository for MemoryRepository {
    async fn save(&self, user: User) {
        println!("{:?}", user);
        //let mut users = self.users.lock().unwrap();
        users.insert(user.id.clone(), user);
        Ok(())
    }
}
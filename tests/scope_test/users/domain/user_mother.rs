use uuid::Uuid;
use fake::{Dummy, Fake, Faker};
use fake::faker::internet::en::Password;

#[derive(Debug)]
pub(crate) struct UserMother {
    id: Uuid,
    email: String,
    pwd: String,
    confirmpwd: String
}

impl UserMother {
    pub fn new() -> UserMother {
        let password = Password(8..10).fake();
        UserMother {
            id: Uuid::new_v4(),
            email: Faker::Internet.email(),
            pwd: password.clone(),
            confirmpwd: password.clone()
        }
    }

}
use fake::Fake;
use fake::faker::internet::en::Username;

pub(crate) fn generate_users(amount: i32) -> Vec<String> {
    let mut users = Vec::<String>::new();

    for _ in 0..amount {
        users.push(Username().fake());
        println!("{:?}", users.last().unwrap());
    }

    users
}
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}


fn main() {
    let json_data = r#"
        {
            "id": 1,
            "username": "john_doe",
            "email": "john.doe@example.com"
        }
    "#;

    let user: User = serde_json::from_str(json_data).unwrap();
    println!("{:?}", user);

    let user = User {
        id: 1,
        username: "john_doe".to_owned(),
        email: "john.doe@example.com".to_owned(),
    };

    let json_data = serde_json::to_string(&user).unwrap();
    println!("{}", json_data);
}

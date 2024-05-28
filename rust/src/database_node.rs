use godot::engine::Node;
use godot::prelude::*;
use surrealdb::Surreal;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, File};
use tokio::runtime::Runtime;

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    age: i32
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct DatabaseNode {
    #[export]
    database_path: GString,
    #[export]
    namespace: GString,
    #[export]
    database: GString,
    base: Base<Node>,
}

#[godot_api]
impl DatabaseNode {
    #[func]
    fn connect_database(&self) {
        let database_path = self.clone().database_path.to_string();
        let namespace = self.clone().namespace.to_string();
        let database = self.database.to_string();

        // Create a runtime for executing the async block
        let rt = Runtime::new().unwrap();

        // Use the runtime to block on the async function
        rt.block_on(async {
            match DB.connect::<File>(&database_path).await {
                Ok(_) => {
                    godot_print!("Connected to database at path: {}", database_path);

                    DB.use_ns(namespace).use_db(database).await.unwrap();
                },
                Err(err) => godot_print!("Error connecting to database: {}", err),
            }
        });
    }

    #[func]
    fn insert_user(&self) {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            age: 42,
        };

        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let _: Option<User> = DB
                .create(("user", "first_test"))
                .content(User {
                    id: 1,
                    name: "John Doe".to_string(),
                    age: 42,
                })
                .await
                .unwrap();
        });
    }

    #[func]
    fn query_users(&self) {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let users: Vec<User> = DB
                .select("user")
                .await
                .unwrap();

            godot_print!("{:?}", users);
        });
    }
}

#[godot_api]
impl INode for DatabaseNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            database_path: "./database".into(),
            namespace: "namespace".into(),
            database: "database".into(),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Ready from Rust here!");
        self.connect_database();
        self.insert_user();
        //self.query_users();
    }
}

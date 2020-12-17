use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::{from_row, params, Error as DBError, Row};

use crate::schemas::root::Context;
use crate::schemas::user::User;

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub price: f64,
}

#[juniper::object(Context = Context)]
impl Product {
    fn id(&self) -> Option<i32> {
        self.id
    }
    fn user_id(&self) -> i32 {
        self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }

    fn user(&self, context: &Context) -> Option<User> {
        let mut conn = context.dbpool.get().unwrap();
        let user: Result<Option<Row>, DBError> = conn.exec_first(
            "SELECT * FROM users WHERE id=:id",
            params! {"id" => &self.user_id},
        );
        if let Err(err) = user {
            None
        } else {
            let (id, name, email) = from_row(user.unwrap().unwrap());
            Some(User { id, name, email })
        }
    }
}

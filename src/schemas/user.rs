use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::{from_row, params};

use crate::schemas::product::Product;
use crate::schemas::root::Context;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> Option<i32> {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.dbpool.get().unwrap();

        conn.exec_iter(
            "select * from product where user_id=:user_id",
            params! {
                "user_id" => &self.id
            },
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    let (id, user_id, name, price) = from_row(row);
                    Product {
                        id,
                        user_id,
                        name,
                        price,
                    }
                })
                .collect()
        })
        .unwrap()
    }
}

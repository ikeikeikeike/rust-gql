use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::{from_row, params};

use crate::schemas::root::Context;
use crate::schemas::user::User;

/// Organization
#[derive(Default, Debug)]
pub struct Organization {
    pub id: Option<i32>,
    pub identifier: String,
}

#[juniper::object(Context = Context)]
impl Organization {
    fn id(&self) -> Option<i32> {
        self.id
    }
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn users(&self, context: &Context) -> Vec<User> {
        let mut conn = context.dbpool.get().unwrap();

        conn.exec_iter(
            "select u.id, u.identifier, u.email from users u
             join users_organizations as uo on u.id = uo.user_id
             where uo.organization_id = :organization_id
            ",
            params! {
                "organization_id" => &self.id
            },
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    let (id, name, email) = from_row(row);
                    User { id, name, email }
                })
                .collect()
        })
        .unwrap()
    }
}

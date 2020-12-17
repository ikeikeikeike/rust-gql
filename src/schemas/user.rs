use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::{from_row, params};

use crate::schemas::organization::Organization;
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

    fn organizations(&self, context: &Context) -> Vec<Organization> {
        let mut conn = context.dbpool.get().unwrap();

        // ;;select id, identifier from organizations where user_id=:user_id
        conn.exec_iter(
            "select o.id, o.identifier from organizations o
             join users_organizations as uo on o.id = uo.organization_id
             where uo.user_id = :user_id
            ",
            params! {
                "user_id" => &self.id
            },
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    let (id, identifier) = from_row(row);
                    Organization { id, identifier }
                })
                .collect()
        })
        .unwrap()
    }
}

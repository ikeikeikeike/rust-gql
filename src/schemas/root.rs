use juniper::{FieldError, FieldResult, RootNode};
use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::{from_row, params, Error as DBError, Row};

use crate::db::Pool;

use super::product::Product;
use super::user::User;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();
        let users = conn
            .exec_iter(
                "select id, identifier, email from users limit :limit offset :offset",
                params! {"limit" => limit, "offset" => offset},
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|mut row| {
                        let (id, identifier, email) = from_row(row);
                        User {
                            id,
                            name: identifier,
                            email,
                        }
                    })
                    .collect()
            })
            .unwrap();
        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let user: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * from users WHERE id=:id", params! {"id" => id});

        if let Err(err) = user {
            return Err(FieldError::new(
                "User Not Found",
                graphql_value!({ "not_found": "user not found" }),
            ));
        }

        let (id, name, email) = from_row(user.unwrap().unwrap());
        Ok(User { id, name, email })
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Product>> {
        let mut conn = context.dbpool.get().unwrap();
        let products = conn
            .exec_iter(
                "select * from product limit :limit offset :offset",
                params! {"limit" => limit, "offset" => offset},
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
            .unwrap();
        Ok(products)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let product: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * from users WHERE id=:id", params! {"id" => id});

        if let Err(err) = product {
            return Err(FieldError::new(
                "Product Not Found",
                graphql_value!({ "not_found": "product not found" }),
            ));
        }

        let (id, user_id, name, price) = from_row(product.unwrap().unwrap());
        Ok(Product {
            id,
            user_id,
            name,
            price,
        })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}

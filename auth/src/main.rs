use tonic::{transport::Server, Request, Response, Status};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::Result;
use uuid::Uuid;

use auth::auth_server::{Auth, AuthServer};
use auth::{UserId, User, Lastname};

pub mod auth {
    tonic::include_proto!("auth");
}

#[derive(Debug)]
pub struct MyAuth {
    pool: Pool<Postgres>
}

impl MyAuth {
    pub async fn init_database_connection() -> Result<MyAuth> {
        let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:root@localhost/postgres").await?;

        Ok(MyAuth {
            pool
        })
    }
}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn create_user(
        &self,
        request: Request<Lastname>,
    ) -> Result<Response<User>, Status> {
        let id = Uuid::new_v4();
        let user = User {
            id: id.as_bytes().to_vec(),
            lastname: request.into_inner().lastname
        };

        sqlx::query!(
            r#"insert into users (id, lastname) values($1::uuid, $2::text)"#,
            id,
            user.lastname
        )
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(Response::new(user))
    }

    async fn get_user(&self, request: Request<UserId>) -> Result<Response<User>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let auth = MyAuth::init_database_connection().await?;

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
use tonic::{transport::Server, Request, Response, Status};

use route::route_server::{Route, RouteServer};
use route::{Message};

pub mod route {
    tonic::include_proto!("route");
}

#[derive(Debug, Default)]
pub struct MyRoute {}

#[tonic::async_trait]
impl Route for MyRoute {
    async fn get_message(
        &self,
        request: Request<()>,
    ) -> Result<Response<Message>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Message {
          message: String::from("Hello")
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let route = MyRoute::default();

    Server::builder()
        .add_service(RouteServer::new(route))
        .serve(addr)
        .await?;

    Ok(())
}
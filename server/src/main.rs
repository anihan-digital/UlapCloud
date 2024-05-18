mod routes;
use crate::routes::create_routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
   let app = create_routes();

   let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app)
         .await
         .unwrap();
    
}

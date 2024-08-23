mod scope;
mod shared;
use shared::infraestruture::server::run;
#[actix_web::main]
async fn main() {
    run().await;
}

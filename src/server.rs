use actix_web::{App, HttpServer, middleware, web};


use crate::seed::{self, SeedBank};

pub(crate) struct AppState {
    pub(crate) seed_bank: seed::SeedBank,
}

pub struct Server;
impl Server {
    pub fn init() -> Self {
        Self { }
    }

    pub async fn start(&self) -> Result<(), std::io::Error>{
        HttpServer::new(|| {
                        App::new()
                            .app_data(web::Data::new(
                                AppState {
                                    seed_bank: SeedBank::init(),
                                }
                            ))
                            .wrap(middleware::Logger::default())
                            .service(web::resource("/seed").route(web::post().to(seed::get_seed)))
                            .service(seed::list)
                    })
                    .bind("0.0.0.0:9090")?
                    .run()
                    .await
    }
}


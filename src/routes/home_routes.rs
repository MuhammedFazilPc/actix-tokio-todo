use actix_web::web::{self, service};

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(handlers::home_handler::create_todo)
            .service(handlers::home_handler::get_all)
            .service(handlers::home_handler::update_todo)
            .service(handlers::home_handler::delete)
            .service(handlers::home_handler::start)
    );
}

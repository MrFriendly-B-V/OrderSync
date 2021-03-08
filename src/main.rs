mod environment;
mod database;
mod appdata;
mod endpoints;
mod types;
mod auth;
mod threads;

use actix_web::{HttpServer, App};
use std::process::exit;
use tera::Tera;
use crate::environment::get_environment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting OrderSync");

    //Check environmental variables
    println!("Running preflight checks.");
    if !environment::check_environment() {
        eprintln!("Preflight checks failed. Not all required environmental variables are set. Exiting!");
        exit(1);
    }

    //Preflight checks are complete
    println!("Preflight checks passed");

    //Create a database object
    let database = database::Database::new();

    //Create a Tera instance
    let mut tera = Tera::new("templates/**/*").expect("Tera error!");
    tera.autoescape_on(vec![]);

    let env = environment::get_environment();
    HttpServer::new(move || {
        let appdata = appdata::AppData {
            database: database.clone(),
            tera: tera.clone()
        };

        App::new()
            .data(appdata)

            //Static pages
            .service(actix_files::Files::new("/static", get_environment().static_web_dir)
                .show_files_listing()
                .index_file("index.html")
                .disable_content_disposition()
            )

            //API endpoints
            .service(endpoints::wix::get_install::get_install)
            .service(endpoints::wix::get_grant::get_grant)
            .service(endpoints::wix::webhooks::post_order_created::post_order_created)
            //.service(endpoints::wix::webhooks::post_order_paid)

            .data(actix_web::web::PayloadConfig::new(1 << 25))
    })
    .bind(format!("{bind_address}:{bind_port}", bind_address = env.bind_address, bind_port = env.bind_port))?
    .run()
    .await
}
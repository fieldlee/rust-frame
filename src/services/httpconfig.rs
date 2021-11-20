use actix_web::web;

use crate::services::configurations;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/").route(web::get().to(configurations::get_user)));
    // cfg.service(
    //     web::resource("/delete-user/").route(web::delete().to(configurations::delete_user)),
    // );
    cfg.service(
        web::resource("/update-user/").route(web::patch().to(configurations::put_user)),
    );
    cfg.service(
        web::resource("/create-user/").route(web::post().to(configurations::post_user)),
    );
}

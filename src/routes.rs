// use actix_web::web;
// use crate::handlers::{create_user, get_all_user, get_user_by_id,delete_user_by_id};
//
//
// pub fn configure_routes(cfg: &mut web::ServiceConfig)  {
//     cfg.service(
//         web::scope("/users")
//             .route("", web::post().to(create_user))
//             .route("", web::get().to(get_all_user))
//             .route("/{id}", web::get().to(get_user_by_id))
//             .route("/{id}", web::delete().to(delete_user_by_id)),
//     );
// }
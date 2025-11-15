use crate::handlers::{add_task, complete_task, delete_task, get_tasks};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
    cfg.service(add_task);
    cfg.service(complete_task);
    cfg.service(delete_task);
}
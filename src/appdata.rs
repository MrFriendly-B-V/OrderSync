use crate::database::Database;
use tera::Tera;

#[derive(Clone)]
pub struct AppData {
    pub database: Database,
    pub tera:     Tera
}
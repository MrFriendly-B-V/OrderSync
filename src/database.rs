use mysql::Pool;
use crate::environment::get_environment;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool
}

impl Database {
    pub fn new() -> Database {
        let env = get_environment();

        let mysql_uri = format!("mysql://{username}:{password}@{host}/{database}",
            username = env.mysql_username,
            password = env.mysql_password,
            host = env.mysql_host,
            database = env.mysql_database
        );

        let pool = Pool::new(mysql_uri);
        if pool.is_err() {
            eprintln!("Unable to connect to the database: {:?}", pool.err());
            std::process::exit(1);
        }

        Database {
            pool: pool.unwrap()
        }
    }
}
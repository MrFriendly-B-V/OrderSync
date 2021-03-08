use std::env;

#[derive(Clone)]
pub struct Environment {
    //MySQL
    pub mysql_host:     String,
    pub mysql_database: String,
    pub mysql_username: String,
    pub mysql_password: String,

    //Web server
    pub bind_address:   String,
    pub bind_port:      String,
    pub static_web_dir: String,

    //Authentication server
    pub auth_host:      String,
    pub auth_apikey:    String,

    //Host
    pub api_host:       String,
    pub frontend_host:  String,

    //Wix data
    pub wix_app_id:     String,
    pub wix_app_secret: String,

}

/**
Check if all required environmental variables are set
*/
pub fn check_environment() -> bool {
    if env::var("MYSQL_HOST").is_err() {
        eprintln!("Environmental variable 'MYSQL_HOST' is not set!");
        return false;
    }

    if env::var("MYSQL_DATABASE").is_err() {
        eprintln!("Environmental variable 'MYSQL_DATABASE' is not set!");
        return false;
    }

    if env::var("MYSQL_USERNAME").is_err() {
        eprintln!("Environmental variable 'MYSQL_USERNAME' is not set!");
        return false;
    }

    if env::var("MYSQL_PASSWORD").is_err() {
        eprintln!("Environmental variable 'MYSQL_PASSWORD' is not set!");
        return false;
    }

    if env::var("BIND_ADDRESS").is_err() {
        eprintln!("Environmental variable 'BIND_ADDRESS' is not set!");
        return false;
    }

    if env::var("BIND_PORT").is_err() {
        eprintln!("Environmental variable 'BIND_PORT' is not set!");
        return false;
    }

    if env::var("STATIC_WEB_DIR").is_err() {
        eprintln!("Environmental variable 'STATIC_WEB_DIR' is not set!");
        return false;
    }

    if env::var("AUTH_HOST").is_err() {
        eprintln!("Environmental variable 'AUTH_HOST' is not set!");
        return false;
    }

    if env::var("AUTH_APITOKEN").is_err() {
        eprintln!("Environmental variable 'AUTH_APITOKEN' is not set!");
        return false;
    }

    if env::var("API_HOST").is_err() {
        eprintln!("Environmental variable 'API_HOST' is not set!");
        return false;
    }

    if env::var("FRONTEND_HOST").is_err() {
        eprintln!("Environmental variable 'FRONTEND_HOST' is not set!");
        return false;
    }

    if env::var("WIX_APP_ID").is_err() {
        eprintln!("Environmental variable 'WIX_APP_ID' is not set!");
        return false;
    }

    if env::var("WIX_APP_SECRET").is_err() {
        eprintln!("Environmental variable 'WIX_APP_SECRET' is not set!");
        return false;
    }

    return true;
}

pub fn get_environment() -> Environment {
    Environment {
        mysql_host:         env::var("MYSQL_HOST").unwrap(),
        mysql_database:     env::var("MYSQL_DATABASE").unwrap(),
        mysql_username:     env::var("MYSQL_USERNAME").unwrap(),
        mysql_password:     env::var("MYSQL_PASSWORD").unwrap(),

        bind_address:       env::var("BIND_ADDRESS").unwrap(),
        bind_port:          env::var("BIND_PORT").unwrap(),
        static_web_dir:     env::var("STATIC_WEB_DIR").unwrap(),

        auth_host:          env::var("AUTH_HOST").unwrap(),
        auth_apikey:        env::var("AUTH_APITOKEN").unwrap(),

        api_host:           env::var("API_HOST").unwrap(),
        frontend_host:      env::var("FRONTEND_HOST").unwrap(),

        wix_app_id:         env::var("WIX_APP_ID").unwrap(),
        wix_app_secret:     env::var("WIX_APP_SECRET").unwrap(),
    }
}
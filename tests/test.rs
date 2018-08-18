#[macro_use] extern crate envconfig;

use std::env;
use envconfig::Error;

mod config {
    envconfig!(Config {
        db_host: String = "DB_HOST",
        db_port: u16 = "DB_PORT"
    });
}

use config::Config;


fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_inits_config_from_env_variables() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init().unwrap();
    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5432u16);
}

#[test]
fn test_checks_presence_of_env_vars() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init().err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "DB_PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_fails_if_can_not_parse_db_port() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "67000");

    let err = Config::init().err().unwrap();
    let expected_err = Error::ParseError { name: "DB_PORT" };
    assert_eq!(err, expected_err);
}

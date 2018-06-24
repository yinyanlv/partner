use common::config::Config;

lazy_static! {

    pub static ref CONFIG: Config = {

        Config::get()
    };
}

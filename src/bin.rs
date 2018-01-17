extern crate botejao;
extern crate flexi_logger;
extern crate log;
use botejao::Botejao;
use flexi_logger::{opt_format, Logger};
use std::env;



fn main() {
    Logger::with_str("info")
        .log_to_file()
        .directory("log_files")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let bot_token = env::var("TELEGRAM_BOT_ID")
        .ok()
        .expect("Can't find TELEGRAM_BOT_ID env variable")
        .parse::<String>()
        .unwrap();

    let botejao = Botejao::new(bot_token.clone());

    botejao.start();
}

extern crate log;
extern crate flexi_logger;
extern crate botejao;
use botejao::Botejao;
use std::path::{Path};
use flexi_logger::{Logger, opt_format};
use std::env;



fn main(){
    const LAST_DAY_WHEN_BROADCASTED_PATH: &str = "last_day_when_broadcasted.txt";
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

    let rep_chat_id = env::var("REP_CHAT_ID")
        .ok()
        .expect("Can't find REP_CHAT_ID env variable")
        .parse::<i64>()
        .unwrap();

    let path = Path::new(LAST_DAY_WHEN_BROADCASTED_PATH);
    let botejao = Botejao::new(bot_token.clone(),
                               vec![rep_chat_id],
                               path.to_path_buf());



    botejao.start();


}




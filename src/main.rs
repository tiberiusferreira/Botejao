extern crate webdriver_client;

use std::sync::{Once, ONCE_INIT};
use std::thread::sleep;
use std::time::Duration;
use webdriver_client::{Driver};
use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::ExecuteCmd;
use webdriver_client::messages::LocationStrategy;
fn main() {
    let gecko = GeckoDriver::build()
        .firefox_binary("/usr/bin/firefox")
        .spawn().unwrap();
    let sess = gecko.session().unwrap();
    let site =sess.go("https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6").unwrap();
    let six_sec = std::time::Duration::from_secs(2);
    std::thread::sleep(six_sec);

    println!("{:?}", sess.find_element("#almocoTerca", LocationStrategy::Css).unwrap().text().unwrap());

}
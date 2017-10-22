
extern crate tokio_core;
extern crate futures;
extern crate fantoccini;
use fantoccini::Client;
use futures::future::Future;
fn main() {

    let mut core = tokio_core::reactor::Core::new().unwrap();

    let (c, fin) = Client::new("http://localhost:4444", &core.handle());
    let c = core.run(c).unwrap();

    {
        // we want to have a reference to c so we can use it in the and_thens below
        let c = & c;

        // now let's set up the sequence of steps we want the browser to take
        // first, go to the Wikipedia page for Foobar
        let f = c.goto("https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6")
            .and_then( move |_| {
                                let three_sec = std::time::Duration::from_secs(3);
                                std::thread::sleep(three_sec);
                c.by_selector("#almocoSegunda")
            })
            .and_then(move |rsl| {
                rsl.text()
            }
            ).and_then(move |text| {
            println!("{}",text);
            Ok(())
        });






        // and set the browser off to do those things
        core.run(f).unwrap();
    }

    // drop the client to delete the browser session
    drop(c);
    // and wait for cleanup to finish
    core.run(fin).unwrap();
}
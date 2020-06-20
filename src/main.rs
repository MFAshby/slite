use slack;
use slack::{Event, RtmClient};

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        // find the general channel id from the `StartResponse`
        let channels = cli.start_response().channels.as_ref();
        println!("channels: {:?}", channels);
        let general_channel_id = channels
            .and_then(|channels| {
                channels
                    .iter()
                    .find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "ai",
                    })
            })
            .and_then(|chan| chan.id.as_ref())
            .expect("ai channel not found");
        let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
        // Send a message over the real time api websocket
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run --example slack_example -- <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
// use std::error::Error;
//
// fn main() -> Result<(), Box<dyn Error>> {
//     use linefeed::{Interface, ReadResult};
//
//     let reader = Interface::new("my-application")?;
//
//     reader.set_prompt("my-app> ")?;
//
//     while let ReadResult::Input(input) = reader.read_line()? {
//         println!("got input {:?}", input);
//     }
//
//     println!("Goodbye.");
//     Ok(())
// }

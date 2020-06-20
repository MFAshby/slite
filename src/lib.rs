pub mod cfg {
    use std::path::Path;

    pub trait Cfg {
        fn dbtype(&self) -> &str;
        fn dbfile(&self) -> &Path;
    }

    pub fn load() -> Box<Cfg> {
        use config_rs::load;
        load()
    }

    mod config_rs {
        use super::*;
        use config::{Config, Environment, File};

        struct CfgRs {
            config: Config,
        }

        impl Cfg for CfgRs {
            fn dbtype(&self) -> &str {
                self.config.get("dbtype").unwrap_or("sqlite")
            }

            fn dbfile(&self) -> &Path {
                Path::new(self.config.get("dbfile").unwrap_or("slite.db"))
            }
        }

        pub(super) fn load() -> Box<dyn Cfg> {
            let mut c = Config::new();
            Box::new(CfgRs {
                config: c
                    .merge(File::with_name("slite"))
                    .unwrap()
                    .merge(Environment::with_prefix("SLITE"))
                    .unwrap()
                    .to_owned(),
            })
        }
    }
}

pub mod db {
    use crate::cfg::Cfg;
    use std::path::Path;

    pub trait Db {
        // Add database methods here.
        // Take care not to expose sqlite details, just work on domain models (i.e. what slack
        // crate gives us)
    }

    pub fn new(c: &dyn Cfg) -> Box<dyn Db> {
        let dbtype = c.dbtype();
        let db = match dbtype {
            "sqlite" => sqlite::new(c.dbfile()),
            _ => panic!("Unexpected database type requested {}", dbtype),
        };
        Box::new(db)
    }

    mod sqlite {
        use super::*;
        use rusqlite::Connection;

        pub(super) struct SqliteDb {
            conn: Connection,
        }

        impl Db for SqliteDb {}

        pub(super) fn new(file: &Path) -> SqliteDb {
            let connection = Connection::open(file)
                .expect(format!("Unable to open database {:?}", file).as_str());
            // TODO: MFA - Add schema migrations
            SqliteDb { conn: connection }
        }
    }
}

mod rtm_listener {
    use std::error::Error;

    use slack;
    use slack::{Event, RtmClient};

    struct Handler;

    #[allow(unused_variables)]
    impl slack::EventHandler for Handler {
        fn on_event(&mut self, cli: &RtmClient, event: Event) {
            // Update the database with the new information received
            println!("on_event(event: {:?})", event);
        }

        fn on_close(&mut self, cli: &RtmClient) {
            // Oops, uh, reconnect I guess
            println!("on_close");
        }

        fn on_connect(&mut self, cli: &RtmClient) {
            // Update the database with start_response info
            println!("on_connect");

            // Sample code
            // find the general channel id from the `StartResponse`
            let channels = cli.start_response().channels.as_ref();
            println!("channels: {:?}", channels);
            let general_channel_id = channels
                .and_then(|channels| {
                    channels.iter().find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "ai",
                    })
                })
                .and_then(|chan| chan.id.as_ref())
                .expect("ai channel not found");
            let _ = cli
                .sender()
                .send_message(&general_channel_id, "Hello world! (rtm)");
            // Send a message over the real time api websocket
        }
    }

    pub fn start(api_key: &str) -> Result<(), Box<dyn Error>> {
        let mut handler = Handler;
        RtmClient::login_and_run(&api_key, &mut handler).map_err(Box::from)
    }
}

mod viewmodels {}

mod views {}

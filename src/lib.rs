pub mod cfg {
    use std::path::Path;
    use config::{Config, Environment, File};

    pub struct Cfg {
        config: Config,
    }

    impl Cfg {
        pub fn dbfile(&self) -> &Path {
            Path::new(self.config.get("dbfile").unwrap_or("slite.db"))
        }
        pub fn api_key(&self) -> String {
            self.config.get("api_key").expect("api_key must be set in config!")
        }
    }

    pub fn load() -> Cfg {
        let mut c = Config::new();
        Cfg {
            config: c
                .merge(File::with_name("slite"))
                .unwrap()
                .merge(Environment::with_prefix("SLITE"))
                .unwrap()
                .to_owned(),
        }
    }
}

pub mod db {
    use crate::cfg::Cfg;
    use rusqlite::{Connection,params};
    use std::error::Error;

    pub struct Db {
        conn: Connection,
    }

    impl Db {
        pub fn add_channel(&self, id: &str, name: &str) -> Result<(), Box<dyn Error>> {
            self.conn.execute("
                insert into channel (id, name) values (?1, ?2)
                ", params![id, name],).map_err(Box::new)?;
            Ok(())
        }

        pub fn delete_channels(&self) -> Result<(), Box<dyn Error>> {
            self.conn.execute("
                delete from channel
                ", params![],).map_err(Box::new)?;
            Ok(())
        }
    }

    pub fn load(c: &Cfg) -> Result<Db, Box<dyn Error>> {
        let file = c.dbfile();
        let conn = Connection::open(file).map_err(Box::new)?;
        conn.execute("
            create table if not exists channel (
                id text primary key,
                name text 
                );
            ", params![],).map_err(Box::new)?;
        Ok(Db { conn })
    }
}

pub mod rtm_listener {
    use std::error::Error;
    use slack::{Event, RtmClient};
    use crate::db::{Db};
    use crate::cfg::{Cfg};

    struct Handler<'a> {
        d: &'a Db
    }

    #[allow(unused_variables)]
    impl slack::EventHandler for Handler<'_> {
        fn on_event(&mut self, cli: &RtmClient, event: Event) {
            // Update the database with the new information received
            // println!("on_event(event: {:?})", event);
        }

        fn on_close(&mut self, cli: &RtmClient) {
            // Oops, uh, reconnect I guess
            // println!("on_close");
        }

        fn on_connect(&mut self, cli: &RtmClient) {
            // Update the database with start_response info
            // println!("on_connect");

            // Sample code
            // find the general channel id from the `StartResponse`
            let channels = cli.start_response().channels.as_ref();
            self.d.delete_channels()
                .expect("Failed to delete channels");
            
            println!("channels: {:?}", channels);
            for chan in channels.unwrap().iter() {
                self.d.add_channel(chan.id.as_ref().unwrap(), chan.name.as_ref().unwrap())
                    .expect("Failed to insert channel");
            }
            //let _ = cli
            //    .sender()
            //    .send_message(&general_channel_id, "Hello world! (rtm)");
            // Send a message over the real time api websocket
        }
    }

    pub fn start(c: &Cfg, d: &Db) -> Result<(), Box<dyn Error>> {
        let api_key = c.api_key();
        let mut handler = Handler {
            d
        };
        RtmClient::login_and_run(&api_key, &mut handler).map_err(Box::from)
    }
}

mod viewmodels {}

mod views {}

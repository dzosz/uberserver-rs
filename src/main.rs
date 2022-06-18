use clap::Parser;
use log::{info,error, set_max_level};
use signal_hook::{consts::SIGHUP, consts::SIGINT, iterator::Signals};
use std::process::Command;

#[macro_use]
extern crate diesel;
extern crate chrono;

mod chatserver;
mod client;
mod natserver;
mod protocol;
mod sqlusers;
mod schema;

/**Starts uberserver.

SQLURL Examples:
  "sqlite:///:memory:" or "sqlite:///"
     both make a temporary database in memory
  "sqlite:////absolute/path/to/database.txt"
     uses a database in the file specified
  "sqlite:///relative/path/to/database.txt"
     note sqlite is slower than a real SQL server
  "mysql://user:password@server:port/database?charset=utf8"
     requires the MySQLdb module
  "oracle://user:password@server:port/database"
     requires the cx_Oracle module
  "postgres://user:password@server:port/database"
     requires the psycopg2 module
  "mssql://user:password@server:port/database"
     requires pyodbc (recommended) or adodbapi or pymssql
  "firebird://user:password@server:port/database"
     requires the kinterbasdb module
**/
#[derive(Parser)]
struct DataHandler {
    /// Server will host on this port (default is 8200)
    #[clap(short, long, default_value = "8200")]
    port: u32,
    /// Server will use this port for NAT transversal (default is 8201)
    #[clap(short, long, default_value = "8201")]
    natport: u32,
    /// Reads additional command-line arguments from file, NOT IMPLEMENTED
    #[clap(short('g'), long, default_value = "")]
    loadargs: String,
    /// Writes console output to file (for logging)
    #[clap(short, long, default_value = "server.log")]
    output: String,
    /// Reload the server on SIGHUP (if SIGHUP is supported by OS)
    #[clap(short('u'), long)]
    sighup: bool,
    /// Sets latest Spring version to this string. Defaults to "*"
    #[clap(short('v'), long, default_value = "*")]
    min_spring_version: String,
    /// Uses SQL database at the specified sqlurl for user, channel, and ban storage.
    #[clap(short, long, default_value = "sqlite:///server.db")]
    sqlurl: String,
    /// Disables censoring of #main, #newbies, and usernames (default is to censor)
    #[clap(short('c'), long)]
    no_censor: bool,
    /// Path to proxies.txt, for trusting proxies to pass real IP through local IP
    #[clap(long, default_value = "")]
    proxies: String,
    // /// sets the pat to the agreement file which is sent to a client registering at the server
    // #[clap(short, long)]
    // agreement: String,
    /// redirects connecting clients to the given ip and port
    #[clap(short, long, default_value = "")]
    redirect: String,
}

impl DataHandler {
    fn parse() -> Self {
        let mut obj: Self = Parser::parse();
        obj.initialize_defaults();
        if !obj.loadargs.is_empty() {
            panic!("parsing args from file not implemented");
        }
        obj
    }

    fn initialize_defaults(&mut self) {
        if self.natport == 0 {
            self.natport = self.port + 1;
        }
    }

    fn parseFiles(&mut self) {}
    fn init(&mut self) {
        // TODO implement
        // self.parseFiles()

        let ver = get_server_version();
        /*
        let mut signals = Signals::new(&[SIGHUP]).unwrap();

        thread::spawn(move || {
            for sig in signals.forever() {
                info!("Received signal {:?}", sig);
                // TODO send a message to main thread channel with reload request
            }
        });*/
    }

    fn shutdown(self) {
        info!("Datahandler shutdown.");
        // TODO implement
        /*
        if self.chanserv and self.protocol:
            self.protocol.in_STATS(self.chanserv)
        */
    }
}

fn get_server_version() -> String {
    let result = match Command::new("git").args(["describe"]).output() {
        Ok(res) => String::from_utf8(res.stdout).unwrap().trim().to_string(),
        Err(err) => {
            error!("Cannot get server version: {}", err);
            "unknown".to_string()
        }
    };

    info!("Server version: {}", result);
    result
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut datahandler = DataHandler::parse();

    set_max_level(log::LevelFilter::Trace); // TODO doesnt work

    info!("Starting uberserver...");

    let serv = natserver::NATServer {};
    // 2. start NATserver
    let natport = datahandler.natport;
    tokio::spawn(async move {
        serv.start(natport).await;
        // TODO graceful shutdown and panic! if error?
    });

    // 3.
    datahandler.init();

    // 4. start chatfactory TCP connection
    let port = datahandler.port;
    tokio::spawn(async move {
        chatserver::ChatServer::start(port).await;
    });

    // 5. start scheduled clean 60*60*24
    // 6. start channel_mute_ban_timeout
    // 7. start decrement_recent_registrations
    // 8. start decrement_recent_renames

    // 9. listen to keyboard interrupts

    // TODO add signals to tokio::select!
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    for sig in signals.forever() {
        info!("Server killed by keyboard interrupt.");
        break;
    }
    // 10.
    datahandler.shutdown();
}

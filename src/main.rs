mod server;

use std::fs::File;

use clap::{Arg, ArgMatches, App};
use daemonize::Daemonize;
use hyper::{service::{make_service_fn, service_fn}, Server};
use server::http::process;


fn parse_args() -> ArgMatches<'static> {
    let matches = App::new("datalogts")
        .help_message("")
        .version("1.0")
        .author("liu.weihua@rytong.com")
        .arg(
            Arg::with_name("port")
                .short("hp")
                .long("hport")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(Arg::with_name(""))
        .get_matches();
    matches
}

async fn http_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], port).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(process)) });
    let server = Server::bind(&addr).serve(service);
    server.await?;
    Ok(())
}


fn main() {
    let matches = parse_args();
    let http_port = matches.value_of("http_port").unwrap_or("8000");
    let http_port = String::from(http_port).parse::<u16>().unwrap();

    let stdout = File::create("appms.out").unwrap();
    let stderr = File::create("appms.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("appms.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(".") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        // .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .chroot(".")
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| {
            println!("Executed before master process exits");
        })
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            http_server(http_port).await.unwrap();
        })
}
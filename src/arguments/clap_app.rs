use clap::{crate_name, crate_version, App as ClapApp, AppSettings, Arg};

pub fn build_app<'a>() -> ClapApp<'a, 'a> {
    ClapApp::new(crate_name!())
        .version(crate_version!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .about("Aget - Asynchronous Downloader")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .empty_values(false)
                .multiple(false)
                .help("URL to request.")
        )
        .arg(
            Arg::with_name("method")
                .short("X")
                .long("method")
                .default_value("GET")
                .multiple(false)
                .takes_value(true)
                .help("Request method, e.g. GET, POST.")
        )
        .arg(
            Arg::with_name("header")
                .short("H")
                .long("header")
                .multiple(true)
                .takes_value(true)
                .help("Request headers, e.g. -H \"User-Agent: aget\".")
        )
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .multiple(false)
                .takes_value(true)
                .help("Request with POST method with the data, e.g. -d \"a=b\".")
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .multiple(false)
                .takes_value(true)
                .help("The path of output for the request e.g. -o \"/path/to/file\".")
        )
        .arg(
            Arg::with_name("concurrency")
                .short("s")
                .long("concurrency")
                .default_value("10")
                .multiple(false)
                .takes_value(true)
                .help("The number of concurrency request e.g. -s 10")
        )
        .arg(
            Arg::with_name("chunk-size")
                .short("k")
                .long("chunk-size")
                .default_value("1m")
                .multiple(false)
                .takes_value(true)
                .help("The interval length of each concurrent request e.g. -k 100k")
        )
        // `awc` does not support proxy
        // .arg(
        //     Arg::with_name("proxy")
        //         .long("proxy")
        //         .multiple(false)
        //         .takes_value(true)
        //         .help("proxy (http/https/socks4/socks5) e.g. -p http://localhost:1024")
        // )
        //
        // Request timeout is the total time before a response must be received.
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .multiple(false)
                .takes_value(true)
                .help("Timeout(seconds) of request")
        )
        .arg(
            Arg::with_name("dns-timeout")
                .short("n")
                .long("dns-timeout")
                .default_value("10")
                .multiple(false)
                .takes_value(true)
                .help("DNS Timeout(seconds) of request")
        )
        .arg(
            Arg::with_name("max-retries")
                .long("max-retries")
                .default_value("5")
                .multiple(false)
                .takes_value(true)
                .help("The maximum times of retring")
        )
        .arg(
            Arg::with_name("retry-wait")
                .long("retry-wait")
                .default_value("0")
                .multiple(false)
                .takes_value(true)
                .help("The seconds between retries")
        )
        .arg(
            Arg::with_name("type")
                .long("type")
                .default_value("auto")
                .multiple(false)
                .takes_value(true)
                .help("Task type, auto/http/m3u8")
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Debug output. Print all trackback for debugging")
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .help("Quiet mode. Don't show progress bar and task information. But still show the error information"),
        )
        .help_message("Print this help message.")
        .version_message("Show version information.")
}
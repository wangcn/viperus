#[macro_use]
extern crate log;
extern crate clap;
extern crate viperus;
extern crate tempfile;
use std::io::Write;
use clap::{App, Arg, SubCommand};
use std::fs::File;


fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_watch() {
    init();


    let mut cfg=tempfile::NamedTempFile::new().unwrap();
    //let cfgFile= cfg.as_file();
    cfg.write_all("level1:\n   key1: true\n".as_bytes()).unwrap();
    let cfg_path=cfg.into_temp_path();
    debug!("temp file is {}",cfg_path.to_str().unwrap());
    
    viperus::load_file(cfg_path.to_str().unwrap(), viperus::Format::YAML).unwrap();
    viperus::watch_all().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    {
        debug!("write new file...{}",cfg_path.to_str().unwrap());

    let mut cfg_change=File::create(cfg_path.to_str().unwrap()).unwrap();
 
    cfg_change.write_all("level1:\n   key1: false\nlevel2: none\n".as_bytes()).unwrap();
    cfg_change.flush().unwrap();
    

    debug!("write new file...done");
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    let ok = viperus::get::<bool>("level1.key1").unwrap();
    assert_eq!(false, ok);
     
}

/// a mokup adapter for testonly
struct ZeroAdapter {}
impl viperus::ConfigAdapter for ZeroAdapter {
    fn parse(&mut self) -> viperus::AdapterResult<()> {
        Ok(())
    }

    fn get_map(&self) -> viperus::Map {
        let  res = viperus::Map::new();
        res
    }
}

#[test]
fn test_main() {
    init();
    info!("test clap args");

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                //.required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("nocapture")
                .long("nocapture")
                .help("enable no capture"),
        )
        .arg(
            Arg::with_name("showoutput")
                .long("show-output")
                .help("enable showoutput"),
        )
        .arg(Arg::with_name("quiet").long("quiet").help("enable quiet"))
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    viperus::load_file(".env", viperus::Format::ENV).unwrap();
    viperus::load_clap(matches).expect("strange...");
    viperus::bond_clap("v", "verbose");
    viperus::add("verbose", true);

    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    debug!("verbose {:?}", f_verbose);
    info!("RUST_LOG={}", dotenv::var("RUST_LOG").unwrap_or(String::from("none")));
    assert_eq!(true, f_verbose);

    viperus::reload().unwrap();
    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    assert_eq!(true, f_verbose);
}

#[test]
fn test_adapter() {
    init();
    info!("test adapter creation");

    viperus::load_file(".env", viperus::Format::ENV).unwrap();
    let mut adp = ZeroAdapter {};
    viperus::load_adapter(&mut adp).unwrap();
    viperus::add("verbose", true);

    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    assert_eq!(true, f_verbose);
}
use std::env;
use std::process;

use minigrep::Config;

fn main() { 
    // cp. 12 
    //let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("인수 분석 중 오류 발생: {}",err);
        process::exit(1);
    });

    println!("query: {}, targetfile: {}", config.query, config.filename);

    // run(config);
    if let Err(e) = minigrep::run(config) {
        eprintln!("app err: {}", e);
        process::exit(1);
    }

}

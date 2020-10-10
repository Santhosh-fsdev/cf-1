extern crate getopts;
use getopts::Options;
use std::io::Read;
use std::env;
use  std::string::String;

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();
    // let mut opts = Options::new();
    // opts.optopt("o", "", "set output file name", "NAME");
    // opts.optflag("h", "help", "print this help menu");
    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m) => { m }
    //     Err(f) => { panic!(f.to_string()) }
    // };
    // if matches.opt_present("h") {
    //     print_usage(&program, opts);
    // }
    let url = args[1].to_string();
    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}



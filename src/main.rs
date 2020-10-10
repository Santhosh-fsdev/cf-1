extern crate getopts;
use getopts::Options;
use std::convert::TryFrom;
use std::env;
use std::io::Read;
use std::process;
use std::string::String;
use std::time::Instant;

fn main() {
    //status codes
    let mut codes = Vec::new();
    //request timings in ms
    let mut timings = Vec::new();

    //collection CL arguments
    let args: Vec<String> = env::args().collect();

    //content length in bytes
    let mut content_length = Vec::new();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("u", "url", "url of the testing website", "");
    opts.optopt("n", "num", "Only Positive integars", "");
    opts.optflag("h", "help", "The below command is the right one \n ./target/debug/worker-test.exe --url https://my-assignment.santhoshfsdev.workers.dev/links \n ./target/debug/worker-test.exe - this is the executable file \n --url this is the option \n -u is the short form of it \n url of the site u want to test");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    //checking for help arg match
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }

    //checking for both help and num
    if matches.opt_present("u") && !matches.opt_present("n") {
        if args.len() == 3 {
            //getting the url
            let url = args[2].to_string();
            call_url(&url, &mut timings, &mut codes, &mut content_length);
        } else {
            println!("{} Wrong Arguments..exiting1", args.len());
            process::exit(1);
        }
    }
    if matches.opt_present("u") && matches.opt_present("n") {
        //getting the url
        let url = args[2].to_string();

        //getting the num value
        let count: u128 = args[4].parse().unwrap_or(0);
        if count < 1 {
            println!("Wrong Arguments..exiting");
            process::exit(1);
        } else {
            for x in 0..count {
                call_url(&url, &mut timings, &mut codes, &mut content_length);
            }

            timings.sort();
            let min_value = timings.iter().min();
            let max_value = timings.iter().max();
            let sum_elements: u128 = timings.iter().sum();
            let count: u128 = u128::try_from(timings.len()).unwrap();
            let half_count: usize = usize::try_from(count / 2).unwrap();
            println!(
                "{:?} is the total number of requests made to the url {}",
                count, url
            );
            println!(
                "{:?} is fastest request time taken in milliseconds",
                min_value
            );
            println!(
                "{:?} is slowest request time taken in milliseconds",
                max_value
            );
            println!(
                "{:?} is the mean of the request timings",
                sum_elements / count
            );
            println!(
                "{:?} is the median of the request timings",
                timings[half_count]
            );
            let mut percent = 0;
            for y in codes {
                if y != 200 {
                    println!("{} is the Error code that weren't success", y);
                    percent = percent + 1;
                }
            }
            if percent > 0 && percent < count {
                println!(
                    "{}% is the percentage of request that succeeded ",
                    (count / percent) * 100
                )
            } else if percent == 0 {
                println!("{}% is the percentage of request that succeeded ", 100);
                println!("{}% is the percentage of request that failed ", 0)
            } else {
                println!("{}% is the percentage of request that succeeded ", 0);
                println!("{}% is the percentage of request that failed ", 100)
            }

            let min_content_length = content_length.iter().min();
            let max_content_lenght = content_length.iter().min();
            println!(
                "{:?} is the size in bytes of the smallest response",
                min_content_length
            );
            println!(
                "{:?} is the size in bytes of the largest response",
                max_content_lenght
            );
        }
    }
}

//handling http requests
fn call_url(
    url: &str,
    timings: &mut Vec<u128>,
    codes: &mut Vec<reqwest::StatusCode>,
    content_length: &mut Vec<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    codes.push(res.status());
    let len = res
        .headers()
        .get("content-length")
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);
    content_length.push(len);
    let duration = start.elapsed().as_millis();
    timings.push(duration);
    Ok(())
}


//print usage
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options] \nright-command - ./target/debug/worker-test.exe -u  https://my-assignment.santhoshfsdev.workers.dev/links (or) \nright-command - ./target/debug/worker-test.exe -u  https://my-assignment.santhoshfsdev.workers.dev/links --num 4 ", program);
    print!("{}", opts.usage(&brief));
}

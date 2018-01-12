mod util {
    use std::fs::File;
    use std::io::{Read};

    pub fn slurp<P: AsRef<::std::path::Path>>(path: P) -> ::std::io::Result<String> {
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;

        Ok(s)
    }
}

mod focus {
    use std::fs::OpenOptions;
    use std::io::Write;

    use util;

    const PATH: &'static str = "/etc/hosts";
    const HEADER: &'static str = "FOCUS";

    pub fn enabled() -> bool {
        util::slurp(PATH).expect("could not read /etc/hosts").contains(HEADER)
    }

    pub fn enable() {
        let sites_path = ::std::env::home_dir().map(|mut h| {h.push(".focus"); h}).expect("no home folder?");

        let sites = util::slurp(&sites_path).expect("could not read ~/.focus");

        let mut f = OpenOptions::new().append(true).open(PATH).expect("error opening");
        f.write("\n# FOCUS\n".as_bytes()).expect("error writing");
        
        for domain in sites.lines() {
            f.write(format!("127.0.0.1  {}\n", domain).as_bytes()).expect("error writing");
            f.write(format!("127.0.0.1  www.{}\n", domain).as_bytes()).expect("error writing");
        }
        f.write("# END FOCUS\n".as_bytes()).expect("error writing");
    }

    pub fn disable() {
        let content = util::slurp(PATH)
            .expect("error reading")
            .lines()
            .take_while(|line| !line.contains("# FOCUS"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut f = OpenOptions::new().write(true).truncate(true).open("/etc/hosts").expect("error opening -- did you run as sudo?");

        f.write(content.as_bytes()).expect("error writing");
    }
}

fn main() {
    use std::env;

    let args = env::args().collect::<Vec<_>>();
    match args.get(1).map(|s| s.as_ref()) {
        Some("enable") => {
            println!("MAXIMUM EFFORT!!");
            focus::enable();
        },
        Some("disable") => {
            println!("SLACKER!");
            focus::disable();
        },
        Some(unknown) => {
            println!("don't know '{}'", unknown);
        },
        None =>  {
            if focus::enabled() {
                println!("SLACKER!");
                focus::disable();
            } else {
                println!("MAXIMUM EFFORT!!");
                focus::enable();
            }
        }
    }
}

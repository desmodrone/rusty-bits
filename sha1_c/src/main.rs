use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_c: <wordlist.txt> <sha1>hash>");
        return Ok(());
    }

    let hash = args[2].trim();

    if hash.len() != HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid length".into());
    }

    let worldlist = File::open(&args[1])?;
    let reader = BufReader::new(&worldlist);

    for line in reader.lines() {
        let line = line?;
        let common_ps = line.trim();
        if hash == &hex::encode(sha1::Sha1::digest(common_ps.as_bytes())) {
            println!("Password found: {}", &common_ps);
            return Ok(());
        }
        println!("{}", line);
    }

    Ok(())
}

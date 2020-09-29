use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    ipaddr: String,
}

fn main() -> Result<()>{
    let opt = Opt::from_args();

    println!("{:?}", opt);

    Ok(())
}

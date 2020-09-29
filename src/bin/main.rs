use anyhow::Result;
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    ipaddr: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let ipaddr: IpAddr = opt.ipaddr.parse()?;

    println!("{:?}", ipaddr);

    Ok(())
}

use anyhow::Result;
use ipaddress::IPAddress;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    ipaddr: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let ipaddr = IPAddress::parse(opt.ipaddr).unwrap();

    println!("{:?}", ipaddr);

    Ok(())
}

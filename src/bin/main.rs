use anyhow::Result;
use ipnet::IpNet;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    ipaddr: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let net = IpNet::from_str(opt.ipaddr.as_str())?;

    println!("Address: {}", net.addr());
    println!("Subnet: {}", net.netmask());
    println!("Prefix: {}", net.prefix_len());
    println!("Network: {}", net.network());
    println!("Broadcast: {}", net.broadcast());

    Ok(())
}

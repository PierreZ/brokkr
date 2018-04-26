extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate tempdir;

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;
use structopt::StructOpt;

mod docker;

#[derive(StructOpt, Debug)]
#[structopt(name = "Brokkr", about = "Generate KVM images from Docker images")]
struct Opt {
    /// Files to process
    #[structopt(name = "DOCKER_ARCHIVE.tar", parse(from_os_str))]
    archive: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", docker::decompress_and_squash(opt.archive));
}

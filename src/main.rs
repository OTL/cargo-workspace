#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use structopt::StructOpt;

use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Serialize, Deserialize)]
struct Cargo {
    workspace: Workspace,
}

#[derive(Serialize, Deserialize)]
struct Workspace {
    members: Vec<String>,
}

const CARGO_FILE: &str = "Cargo.toml";

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "i", long = "init")]
    init: bool,

    #[structopt(short = "u", long = "update")]
    update: bool,

    /// admin_level to consider
    #[structopt(short = "m", long = "members")]
    members: Vec<String>,
}

fn init_workspace(members: Vec<String>) {
    let mut f = BufWriter::new(fs::File::create(CARGO_FILE).unwrap());
    let cargo = Cargo {
        workspace: Workspace { members: members },
    };
    f.write_all(toml::to_string(&cargo).unwrap().as_bytes())
        .unwrap();
}

fn add_members(mut members: Vec<String>) {
    let mut buf = vec![];
    {
        let mut f = BufReader::new(fs::File::open(CARGO_FILE).unwrap());
        f.read_to_end(&mut buf).unwrap();
    }
    let mut cargo: Cargo = toml::from_str(std::str::from_utf8(&buf).unwrap()).unwrap();
    cargo.workspace.members.append(&mut members);
    let mut f = BufWriter::new(fs::File::create(CARGO_FILE).unwrap());
    f.write(toml::to_string(&cargo).unwrap().as_bytes())
        .unwrap();
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    if opt.init {
        init_workspace(opt.members);
    } else if opt.update {
        add_members(opt.members);
    }
}

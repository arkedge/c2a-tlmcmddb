use std::{
    collections::{btree_map, BTreeMap, HashSet},
    fs,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tlmcmddb::Database;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Bundle {
        tlm_db_dir: PathBuf,
        cmd_db_dir: PathBuf,
        output: PathBuf,
        #[clap(long)]
        pretty: bool,
    },
    Merge {
        #[clap(required = true)]
        tlmcmddbs: Vec<PathBuf>,
        #[clap(required = true, long, short)]
        output: PathBuf,
        #[clap(long)]
        pretty: bool,
    },
}

#[derive(Default)]
pub struct DatabaseBuilder {
    components: BTreeMap<String, tlmcmddb::Component>,
}

impl DatabaseBuilder {
    fn add_telemetry(&mut self, component: String, telemetry: tlmcmddb::tlm::Telemetry) {
        match self.components.entry(component) {
            btree_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().tlm.telemetries.push(telemetry);
            }
            btree_map::Entry::Vacant(vacant) => {
                let component = tlmcmddb::Component {
                    name: vacant.key().to_string(),
                    tlm: tlmcmddb::tlm::Database {
                        telemetries: vec![telemetry],
                    },
                    cmd: tlmcmddb::cmd::Database { entries: vec![] },
                };
                vacant.insert(component);
            }
        }
    }

    fn add_cmddb(&mut self, component: String, cmddb: tlmcmddb::cmd::Database) {
        match self.components.entry(component) {
            btree_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().cmd = cmddb;
            }
            btree_map::Entry::Vacant(vacant) => {
                let component = tlmcmddb::Component {
                    name: vacant.key().to_string(),
                    tlm: tlmcmddb::tlm::Database {
                        telemetries: vec![],
                    },
                    cmd: cmddb,
                };
                vacant.insert(component);
            }
        }
    }

    fn build(self) -> tlmcmddb::Database {
        let mut components: Vec<_> = self.components.into_values().collect();
        for component in components.iter_mut() {
            component
                .tlm
                .telemetries
                .sort_by(|a, b| a.name.cmp(&b.name));
        }
        tlmcmddb::Database { components }
    }
}

#[derive(Default, Debug)]
pub struct DatabaseSet {
    databases: Vec<Database>,
}

impl DatabaseSet {
    pub fn push_database(&mut self, database: Database) {
        self.databases.push(database)
    }

    pub fn merge(self) -> Result<Database> {
        self.validate()?;

        let components = self
            .databases
            .into_iter()
            .flat_map(|database| database.components)
            .collect::<Vec<_>>();

        Ok(Database { components })
    }

    fn validate(&self) -> Result<()> {
        let component_names_iter = self
            .databases
            .iter()
            .flat_map(|database| database.components.iter())
            .map(|component| &*component.name);
        let mut component_name_set = HashSet::new();
        for component_name in component_names_iter {
            if !component_name_set.insert(component_name) {
                // check component name duplication
                return Err(anyhow::anyhow!(
                    "Duplicate component found. {}",
                    component_name
                ));
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Bundle {
            tlm_db_dir,
            cmd_db_dir,
            output,
            pretty,
        } => {
            let mut builder = DatabaseBuilder::default();
            for entry in fs::read_dir(tlm_db_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    continue;
                }
                let filename = entry.file_name();
                let filename = filename.to_str().unwrap();
                if !filename.ends_with(".csv") {
                    // ignore non-csv files
                    continue;
                }
                let ctx = format!("TLM DB CSV: {:?}", entry.path());
                let tlmcmddb_csv::tlm::Filename {
                    component,
                    telemetry,
                } = filename.parse().context(ctx.clone())?;
                let file = fs::OpenOptions::new()
                    .read(true)
                    .open(entry.path())
                    .context(ctx.clone())?;
                let telemetry =
                    tlmcmddb_csv::tlm::parse_csv(telemetry, file).context(ctx.clone())?;
                builder.add_telemetry(component, telemetry);
            }
            for entry in fs::read_dir(cmd_db_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    continue;
                }
                let filename = entry.file_name();
                let filename = filename.to_str().unwrap();
                if !filename.ends_with("_CMD_DB.csv") {
                    // ignore non-command files
                    continue;
                }
                let ctx = format!("CMD DB CSV: {:?}", entry.path());
                let file = fs::OpenOptions::new()
                    .read(true)
                    .open(entry.path())
                    .context(ctx.clone())?;
                let (component, cmddb) = tlmcmddb_csv::cmd::parse_csv(file).context(ctx.clone())?;
                builder.add_cmddb(component, cmddb);
            }
            let db = builder.build();
            output_db(db, &output, pretty)?;
        }
        Command::Merge {
            tlmcmddbs,
            output,
            pretty,
        } => {
            let mut datbase_set = DatabaseSet::default();
            for entry_path in tlmcmddbs {
                let ctx = format!("TLM CMD DB Json: {:?}", entry_path);
                let file = fs::OpenOptions::new()
                    .read(true)
                    .open(entry_path)
                    .context(ctx.clone())?;
                let reader = BufReader::new(file);
                let database: Database = serde_json::from_reader(reader)?;

                datbase_set.push_database(database);
            }
            let db = datbase_set.merge()?;
            output_db(db, &output, pretty)?;
        }
    }
    Ok(())
}

fn output_db(db: Database, output: &Path, pretty: bool) -> Result<()> {
    let output_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .read(true)
        .truncate(true)
        .open(output)?;
    let bufwriter = io::BufWriter::new(output_file);
    if pretty {
        serde_json::to_writer_pretty(bufwriter, &db).map_err(|e| anyhow::anyhow!(e))
    } else {
        serde_json::to_writer(bufwriter, &db).map_err(|e| anyhow::anyhow!(e))
    }
}

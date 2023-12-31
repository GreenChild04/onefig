use std::{fs, path::{Path, PathBuf}};
use home::home_dir;
use clap::{Parser, Subcommand};
use flexar::prelude::Lext;
use hashbrown::HashSet;
use crate::{conff::ConffTree, lexer::Token, safe_unwrap, errors::RuntimeError, nodes::source_file::SourceFile, search::search};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about="A mere test command")]
    Test,
    #[command(about="Compiles onefig-scripts into a single onefig-binary")]
    Compile {
        #[arg(index=1, help="The onefig-script to compile.")]
        script: String,
        #[arg(index=2, help="The output onefig-binary that is compiled.")]
        output: String,
    },
    #[command(about="Checks the validity of an onefig-script without executing or compiling it")]
    Check {
        #[arg(short='b', long, help="Checks file as an onefig binary rather than script.")]
        is_binary: bool,
        #[arg(index=1, help="The onefig script/binary to check.")]
        file: String,
    },
    #[command(about="Executes an onefig script or binary", alias="r")]
    Run {
        // #[arg(short='u', long="unsafe", help="Stops onefig from caching the old configurations; disallowing for rollbacks.")]
        // not_safe: bool,
        #[arg(short='s', long, help="Interprets the files as onefig scripts rather than binaries.")]
        is_script: bool,
        #[arg(index=1, help="The onefig scripts or binaries to execute.")]
        file: String,
    },
    // #[command(about="Clears cache (configuration file history) (also disables rollbacks)")]
    // ClearCache,
    #[command(about="Lists most of the configuration files in your system (unix only)")]
    Search {
        #[arg(short, long, help="Sets if you want to also include `/etc` configs (requires sudo)")]
        etc: bool,
    },
    // #[command(about="Rolls back to the state of the system's config-files before an execution")]
    // Rollback {
    //     #[arg(short='s', long, help="Interprets the files as onefig scripts rather than binaries.")]
    //     is_script: bool,
    //     #[arg(short, long, help="The onefig script or binary to rollback on")]
    //     file: String,
    // },
}

impl Cli {
    pub fn execute(self) {
        let time = std::time::Instant::now();
        use Command as C;
        match self.command {
            C::Test => println!("Testing, testing! Wow, it seems like the cli is working :D!"),
            C::Check { is_binary, file } => if is_binary {
                ConffTree::load_compiled(file);
            } else {
                Self::get_conff_tree(file);
            },
            C::Compile { script, output } => Self::get_conff_tree(script).compile(output),
            C::Run { is_script, file } => if is_script {
                Self::get_conff_tree(file).generate();
            } else {
                ConffTree::load_compiled(file).generate();
            },
            C::Search { etc } => {
                let mut files = HashSet::new();

                search(home_dir().unwrap().join(".config"), &mut files);
                // search(false, home_dir().unwrap(), &mut files); // takes wayy too long
                if etc {
                    search(PathBuf::from("/etc"), &mut files);
                }

                // print the paths
                println!();
                for path in files.iter() {
                    println!("{}", path.to_string_lossy());
                } println!();
            },
        }
        println!("{}", flexar::colour_format![
            green("Finished successfully "),
            none("in "),
            yellow(&time.elapsed().as_secs_f64().to_string()),
            yellow("s"),
        ]);
    }

    #[inline(always)]
    fn get_conff_tree(path: impl AsRef<Path>) -> ConffTree {
        let contents = safe_unwrap!(fs::read_to_string(&path) => RT010, path.as_ref().to_string_lossy());
        let tokens = Token::tokenize(Lext::new(path.as_ref().to_string_lossy().to_string(), &contents));
        let nodes = SourceFile::parse(tokens);
        let action_tree = nodes.visit();
        ConffTree::from_att(action_tree)
    }
}
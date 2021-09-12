mod cli;
mod task;
use cli::{CommandLineArgs,Action::*};
use task::Task;
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::anyhow;

fn find_default_journal_file_or_create() -> Option<PathBuf>{

    home::home_dir().map(|mut path|{
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    
    //Get command line arguments
    let CommandLineArgs{
        action,journal_file
    } = CommandLineArgs::from_args();

    //unpack the journal file

    let journal_file = journal_file
    .or_else(find_default_journal_file_or_create)
    .ok_or(anyhow!("Failed to find journal file"))?;

    //Perform the action

    match action {
        Add { task } => task::add_task(journal_file, Task::new(task)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }?;
   
    Ok(())
}

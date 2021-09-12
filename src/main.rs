mod cli;
mod task;
use cli::{CommandLineArgs,Action::*};
use task::Task;
use structopt::StructOpt;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    
    //Get command line arguments
    let CommandLineArgs{
        action,journal_file
    } = CommandLineArgs::from_args();

    //unpack the journal file

    let journal_file = journal_file
    .ok_or(anyhow!("Failed to find journal file"))?;

    //Perform the action

    match action {
        Add { task } => task::add_task(journal_file, Task::new(task)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }?;
   
    Ok(())
}

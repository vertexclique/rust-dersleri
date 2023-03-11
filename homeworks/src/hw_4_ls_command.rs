use std::{fs, process, path};
use std::path::Path;
use std::error::Error;
fn main()
{
    if let Err( ref e) = run(Path::new(".")) //if returns Ok()
    {
        println!("{}",e);
        process::exit(1);
    }
}

fn run(dir:&Path) ->Result<(),Box<dyn Error>>
{
    if dir.is_dir()
    {
        for entry in fs::read_dir(dir)?
        {
            let item = entry?;

            let file_name = item.file_name().into_string()
            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            
            println!("{}", file_name);

        }

    }
    return Ok(());
}
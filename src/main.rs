extern crate unrar;

use unrar::Archive;
use std::path::Path;
use std::env;

fn main()
{
    let _args: Vec<String> = env::args().collect();

    if _args.len()<2
    {
        println!("USAGE: {} [input_rar_name*]",&_args[0]);
        println!("      * means required");
        std::process::exit(1);
    }

    let filename = &_args[1];
    let path = Path::new(&filename);

    
    if !path.exists()
    {
        println!("FILE NOT FOUND");
        std::process::exit(1);
    }

    let arch = Archive::new(filename.to_string());

    if !arch.is_archive()
        {
            println!("INPUT FILE IS NOT RAR");
            std::process::exit(1);
        }

    let outpt = path.file_stem().unwrap().to_os_string().into_string().unwrap();

    arch.extract_to(outpt).unwrap().process().unwrap();
    println!("DONE!");
    std::process::exit(0);
}
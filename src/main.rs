extern crate nom;
extern crate asalang;

use asalang::{start_interpreter, program};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
    let result = program(r#"if false {let z = 0; return z * 2;} else if false {let x = 1; let y = x + 1; return y;}"#);
    match result {
        Ok((unparsed,tree)) => {
            println!("Unparsed Text: {:?}", unparsed);
            println!("Parse Tree:\n {:#?}", tree);
            let result = start_interpreter(&tree);
            println!("{:?}", result);
        }
        Err(error) => {
            println!("ERROR {:?}", error);
        }
    }

    Ok(())
}

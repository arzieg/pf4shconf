extern crate pf4shconf;
extern crate diesel;

use self::pf4shconf::*;
// use std::io;
use clap::{load_yaml, App, AppSettings, Arg};


fn main() {
    use pf4shconf::schema::*;

    let connection = establish_connection();

    let yaml = load_yaml!("../xhana.yaml");

    let matches = App::from(yaml).get_matches();

   // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            // Now we have a reference to push's matches
            match add_matches.subcommand() {
                Some(("parameter", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let configversion = parameter_matches.value_of("configversion").unwrap();
                    let info = parameter_matches.value_of("info").unwrap();
                    let parameter = parameter_matches.value_of("name").unwrap();
                    let typ = parameter_matches.value_of("type").unwrap();
                    let mandatory = parameter_matches.value_of("mandatory").unwrap();
                    println!("Configversion: {}", configversion);
                    println!("Info to the technical parameter: {}", info );
                    println!("Technical Parametername: {}", parameter);
                    println!("Type of Parametervalue: {}", typ);
                    println!("Mandatory: {}", mandatory);
                    let xhp = add_xhanaparameter(&connection, &configversion, &info, &parameter, &typ, &mandatory);
                    println!("\nSaved configversion {}, info {}, parameter {}, typ {}, mandatory {}", configversion, info, parameter, typ, mandatory);
                    //query_hanaparameter(&connection, &configversion);
                }
                Some(("local", _)) => {
                    println!("'git push local' was used");
                }
                _ => unreachable!(),
            }
        }
        
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
    

    
    //println!("Value: ");
    //let mut value = String::new();
    //io::stdin().read_line(&mut value).unwrap();
    //let value = &value[..(value.len() - 1)];

    //let xhg = create_xhanageneral(&connection, &sid, &version, &parameter, &value);
    

}

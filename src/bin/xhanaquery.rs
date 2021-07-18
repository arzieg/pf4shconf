extern crate diesel;
/// # Query Funktion
/// Abfrage der Konfigurationsdatenbank. Als Parameter können übergeben werden:
///   - sid, die SID eines HANA Systems
///   - version, die Version der Konfiguration
///   - arctype, die Angabe eines Architekturtyps
///   - host, die Angabe eines spezifischen Hosts
///
/// Ausgabe:
/// Je nach Übergabeparameter werden folgende Repots erzeugt:
///  - nur sid: Ausgabe der vorhandenen Versionen für eine SID
///  - sid & version: Ausgabe der Konfigurationsparameter für eine SID
///  - arctype: Ausgabe der definierten Architekturtypen je SID
///  - host: Ausgabe der Konfigurationsparameter zu dem Host
//: Ausgabe
//: - nur sid
//:    xHANAVERSION
//:
//: - sid und version
//:  xHANAGENERAL, xHANAENVIRONMENT
//:
//: -arctype
//:  xHANAARC
//:
//:-host
//:  xHANAENVIRONMENT
extern crate pf4shconf;

use self::diesel::prelude::*;
use self::models::*;
use self::pf4shconf::*;
use clap::{load_yaml, App};

fn main() {
    // use pf4shconf::schema::xhanageneral::dsl::*;
    // Establish a connection
    let connection = establish_connection();
    // load clap parameter
    let yaml = load_yaml!("../xhanaquery.yaml");
    let m = App::from(yaml).get_matches();

    if let Some(sid) = m.value_of("sid") {
        println!("SID = {}", sid.to_uppercase().as_str());
    }

    if let Some(configversion) = m.value_of("configversion") {
        if let Some(sid) = m.value_of("sid") {
            println!("configversion {}", configversion);
            println!("SID {}", sid.to_uppercase().as_str());
            query_sid_version(&connection, sid.to_uppercase().as_str(), configversion);
        }
    }

    if let Some(arctype) = m.values_of("arctype") {
        for _a in arctype {
            match _a {
                "SU" => println!("ScaleUp"),
                "SO" => println!("ScaleOut"),
                "MajorityMaker" => println!("Mojority Maker"),
                "ISCSIServer" => println!("ISCSI Server"),
                "Toolserver" => println!("Toolserver"),
                _ => unreachable!(),
            }
        }
    }

    if let Some(host) = m.values_of("host") {
        for _a in host {
            println!("Host = {}", _a);
        }
    }
}

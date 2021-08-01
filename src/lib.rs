// Postgres starten: pg_ctl -D /var/lib/pgsql/data -l logfile start
pub mod models;
pub mod schema;

extern crate csv;
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate dotenv;
//extern crate serde_derive;

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::error::Error;
// use std::ffi::OsString;
// use std::process;

// Allgemeine Funktionen
// https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-string
fn remove_whitespace(s: String) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/* pub fn create_xhanageneral<'a>(
    conn: &PgConnection,
    sid: &'a str,
    version: &'a str,
    parameter: &'a str,
    value: &'a str,
) -> XHanaGeneral {
    use schema::xhanageneral;

    let new_xhg = NewXHanaGeneral {
        sid: sid,
        version: version,
        parameter: parameter,
        value: value,
    };

    diesel::insert_into(xhanageneral::table)
        .values(&new_xhg)
        .get_result(conn)
        .expect("Error saving new general parameter")
} */

/* pub fn query_sid_version<'a>(conn: &PgConnection, psid: &str, pversion: &str) {
    use schema::xhanageneral::dsl::*;

    println!("SID = {}, VERSION = {}", &psid, &pversion);

    let results = xhanageneral
        .filter(sid.eq(&psid))
        .filter(version.eq(&pversion))
        .get_results::<XHanaGeneral>(conn)
        .expect("Error loading parameters");

    println!("Displaying {} xHanaGeneral", results.len());
    for xhg in results {
        println!(
            "{} {} {} = {:?} ",
            xhg.sid, xhg.version, xhg.parameter, xhg.value
        );
        println!("------------------------------------------------------------------\n");
    }
} */

// Add HANA Parameter
// Save dataset in table xhanaparameter
pub fn add_xhanaparameter<'a>(
    conn: &PgConnection,
    version: &'a str,
    info: &'a str,
    parameter: &'a str,
    scope: &'a str,
    iotype: &'a str,
    valuetype: &'a str,
    mandatory: &'a str,
) -> XHanaParameterTable {
    use schema::xhanaparameter;

    let new_xhp = XHanaParameterInsert {
        version: version,
        parameter: parameter,
        info: info,
        scope: scope,
        iotype: iotype,
        valuetype: valuetype,
        mandatory: mandatory,
    };

    // check if version, parameter are already set

    diesel::insert_into(xhanaparameter::table)
        .values(&new_xhp)
        .on_conflict((xhanaparameter::version, xhanaparameter::parameter))
        .do_update()
        .set(&new_xhp)
        .get_result(conn)
        .expect("Error savong new parameter string")
}

pub fn query_hanaparameter<'a>(conn: &PgConnection, pversion: &str, pparameter: &str) {
    use schema::xhanaparameter::dsl::*;

    println!("VERSION = {}", &pversion);

    let xhp = xhanaparameter
        .filter(version.eq(&pversion))
        .filter(version.eq(&pparameter))
        .get_results::<XHanaParameterTable>(conn)
        .expect("Error loading parameters");
}

// Add HANA Architecture
// Save dataset in table xhanaarc
pub fn add_xhanaarc<'a>(conn: &PgConnection, sid: &'a str, arc: &'a str) -> XHanaArcTable {
    use schema::xhanaarc;

    let new_xha = XHanaArcInsert { sid: sid, arc: arc };

    diesel::insert_into(xhanaarc::table)
        .values(&new_xha)
        .on_conflict(xhanaarc::sid)
        .do_update()
        .set(&new_xha)
        .get_result(conn)
        .expect("Error savong new parameter string")
}

// Add HANA Datacenter
// Save dataset in table xhanadatacenter
pub fn add_xhanadc<'a>(conn: &PgConnection, dcid: &'a i32, name: &'a str) -> XHanaDCTable {
    use schema::xhanadatacenter;

    let new_xhd = XHanaDCInsert {
        dcid: dcid,
        name: name,
    };

    diesel::insert_into(xhanadatacenter::table)
        .values(&new_xhd)
        .on_conflict(xhanadatacenter::dcid)
        .do_update()
        .set(&new_xhd)
        .get_result(conn)
        .expect("Error savong new parameter string")
}

// Add HANA Configurationversion
// Save dataset in table xhanadatacenter
pub fn add_xhanaversion<'a>(
    conn: &PgConnection,
    sid: &'a str,
    version: &'a str,
    tag: &'a str,
) -> XHanaVersionTable {
    use schema::xhanaversion;

    let new_xhv = XHanaVersionInsert {
        sid: sid,
        version: version,
        tag: match tag {
            "EMPTY" => "",
            _ => tag,
        },
    };

    diesel::insert_into(xhanaversion::table)
        .values(&new_xhv)
        .on_conflict((xhanaversion::sid, xhanaversion::version))
        .do_update()
        .set(&new_xhv)
        .get_result(conn)
        .expect("Error saving new parameter string")
}

// Add general Parameter
// Save dataset in table xhanageneral
pub fn add_xhanageneral<'a>(
    conn: &PgConnection, 
    version: &'a str, 
    parameter: &'a str, 
    value: &'a str,) -> XHanaGeneralTable {

    use schema::xhanageneral;

    let new_xhg = XHanaGeneralInsert {
        version: version,
        parameter: parameter,
        value: value,
    };

    diesel::insert_into(xhanageneral::table)
        .values(&new_xhg)
        .on_conflict((xhanageneral::version, xhanageneral::parameter))
        .do_update()
        .set(&new_xhg)
        .get_result(conn)
        .expect("Error saving new parameter string")

}

#[macro_use]
extern crate serde_derive;
#[derive(Debug, Deserialize)]
struct Model {
    parameter: String,
    value: String,
}

pub fn add_xhanamodel<'a>(conn: &PgConnection, file: &'a str) -> Result<(), Box<dyn Error>> {

    use schema::xhanaparameter::dsl::*;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'=')
        .comment(Some(b'#'))
        .from_path(file)?;

    // nur für den Test
    let pversion = "1.0";
    // check if all parameters are defined
    for result in rdr.deserialize() {
        let record: Model = result?;
        let pparameter: String = remove_whitespace(record.parameter);
        print!("Check Parameter {} in Version {}: ", pparameter, pversion);
              
        xhanaparameter
        .filter(version.eq(&pversion))
        .filter(parameter.eq(&pparameter))
        .filter(iotype.eq("input").or(iotype.eq("both")))
        .get_result::<XHanaParameterTable>(conn)
        .expect(&format!("\nDid not found Parameter {} in Version {} - Could not load model!\n",
             pparameter, 
             pversion));
        //println!("DEBUG {:?}", xhp.parameter);
        println!("");
    }
    
    // also hier sind alle Parameter definiert in xhanaparameter. Nun kann das Modell geladen
    // werden
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'=')
        .comment(Some(b'#'))
        .from_path(file)?;

    for result in rdr.deserialize() {
        let record: Model = result?;
        let pparameter: String = remove_whitespace(record.parameter);
        let pvalue: String = remove_whitespace(record.value);
        println!("Parameter: {}, Value:{}", pparameter, pvalue);
        
        // fill xhanageneral
        add_xhanageneral(&conn, pversion, &pparameter, &pvalue);   
        println!("Insert Version {}, Parameter {}, Value {}: ", pversion, pparameter, pvalue);
    }
    
    // ToDo
    // Werte für Tabelle 
    // xhanahost +
    // xhanasid einfügen


    Ok(())        
}

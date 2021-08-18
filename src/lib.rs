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
use std::str::*;

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

// Add HANA Parametertemplate
// Save dataset in table xhanaparameter
pub fn add_xhanaparameter<'a>(
    conn: &PgConnection,
    parameterversion: &'a str,
    parameter: &'a str,
    info: &'a str,
    scope: &'a str,
    arc: &'a str,
    iotype: &'a str,
    valuetype: &'a str,
    mandatory: &'a str,
) -> XHanaParameterTable {
    use schema::xhanaparameter;

    let new_xhp = XHanaParameterInsert {
        parameterversion: parameterversion,
        parameter: parameter,
        info: info,
        scope: scope,
        arc: arc,
        iotype: iotype,
        valuetype: valuetype,
        mandatory: mandatory,
    };

    // check if version, parameter are already set

    diesel::insert_into(xhanaparameter::table)
        .values(&new_xhp)
        .on_conflict((
            xhanaparameter::parameterversion,
            xhanaparameter::parameter,
            xhanaparameter::arc,
            xhanaparameter::iotype,
        ))
        .do_update()
        .set(&new_xhp)
        .get_result(conn)
        .expect("Error savong new parameter string")
}

/*
pub fn query_hanaparameter<'a>(conn: &PgConnection, pversion: &str, pparameter: &str) {
    use schema::xhanaparameter::dsl::*;

    println!("VERSION = {}", &pversion);

    let xhp = xhanaparameter
        .filter(version.eq(&pversion))
        .filter(version.eq(&pparameter))
        .get_results::<XHanaParameterTable>(conn)
        .expect("Error loading parameters");
}
*/

// Add HANA Architecture
// Save dataset in table xhanaarc
pub fn add_xhanaarc<'a>(conn: &PgConnection, arc: &'a str) -> XHanaArcTable {
    use schema::xhanaarc;

    let new_xha = XHanaArcInsert { arc: arc };

    diesel::insert_into(xhanaarc::table)
        .values(&new_xha)
        .on_conflict(xhanaarc::arc)
        .do_update()
        .set(&new_xha)
        .get_result(conn)
        .expect("Error saving new parameter string")
}

// Add HANA Solutionname
// Save dataset in table xhanasolution
pub fn add_xhanasolution<'a>(conn: &PgConnection, solutionversion: &'a str) -> XHanaSolutionTable {
    use schema::xhanasolution;

    let new_xha = XHanaSolutionInsert {
        solutionversion: solutionversion,
    };

    diesel::insert_into(xhanasolution::table)
        .values(&new_xha)
        .on_conflict(xhanasolution::solutionversion)
        .do_update()
        .set(&new_xha)
        .get_result(conn)
        .expect("Error saving new parameter string")
}

// Add HANA Datacenter
// Save dataset in table xhanadatacenter
pub fn add_xhanadc<'a>(conn: &PgConnection, dcid: &'a i32, name: &'a str) -> XHanaDatacenterTable {
    use schema::xhanadatacenter;

    let new_xhd = XHanaDatacenterInsert {
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

// Add HANA SID
// Save dataset in table xhanasid
pub fn add_xhanasid<'a>(conn: &PgConnection, sid: &'a str, name: &'a str) -> XHanaSIDTable {
    use schema::xhanasid;

    let new_xhd = XHanaSIDInsert {
        sid: sid,
        name: match name {
            "EMPTY" => "",
            _ => name,
        },
    };

    diesel::insert_into(xhanasid::table)
        .values(&new_xhd)
        .on_conflict(xhanasid::sid)
        .do_update()
        .set(&new_xhd)
        .get_result(conn)
        .expect("Error savong new parameter string")
}

// Add dependency HANA SID<->Solution
// Save dataset in table xhana_solution_sid

pub fn add_xhana_solution_sid<'a>(
    conn: &PgConnection,
    solutionversion: &'a str,
    sid: &'a str,
    arc: &'a str,
    tag: &'a str,
) -> XHanaSolutionSIDTable {
    use schema::xhana_solution_sid;

    let new_xhv = XHanaSolutionSIDInsert {
        solutionversion: solutionversion,
        sid: sid,
        arc: arc,
        tag: match tag {
            "EMPTY" => "",
            _ => tag,
        },
    };

    diesel::insert_into(xhana_solution_sid::table)
        .values(&new_xhv)
        .on_conflict(xhana_solution_sid::sid)
        .do_update()
        .set(&new_xhv)
        .get_result(conn)
        .expect("Error saving new parameter string")
}


// Helperfunction for add_xhana_sid_host
pub fn add_xhana_host (conn: &PgConnection, phostname: &str) -> XHanaHostTable
{
    use schema::xhanahost; 

    let new_xhh = XHanaHostInsert {
        hostname: phostname,
    };

    diesel::insert_into(xhanahost::table)
        .values(&new_xhh)
        .on_conflict(xhanahost::hostname)
        .do_update()
        .set(&new_xhh)
        .get_result(conn)
        .expect("Error saving new parameter string")

}

pub fn add_xhana_sid_host<'a>(conn: &PgConnection, psid: &'a str, phostname: &'a str) -> XHanaSIDHostTable {
    use schema::xhana_sid_host::dsl::*;
    use schema::xhana_solution_sid::dsl::*;
    use schema::xhana_sid_host;
    

    let psolutionversion = xhana_solution_sid
        .select(schema::xhana_solution_sid::dsl::solutionversion)
        .filter(schema::xhana_solution_sid::dsl::sid.eq(&psid))
        .distinct()
        .load::<String>(conn)
        .expect("Error in Query");

    // Wenn ein Rückgabewert, dann weiter
    if psolutionversion.len() == 1 {
        let mut i_str = "";

        for i in psolutionversion.iter() {
            i_str = i.as_str();
        } 
    
        println!("w= {:?}",&i_str);

        // now we update xhanahost and xhana_sid_host
        // for xhanahost I use a helper function
        let rv : XHanaHostTable = add_xhana_host(conn, phostname); 
    
        let new_xhs = XHanaSIDHostInsert {
            solutionversion: i_str,
            sid : psid,
            hostname : phostname,
        };

        diesel::insert_into(xhana_sid_host)
        .values(&new_xhs)
        .on_conflict((xhana_sid_host::solutionversion, xhana_sid_host::sid, xhana_sid_host::hostname))
        .do_update()
        .set(&new_xhs)
        .get_result(conn)
        .expect("Error saving new parameter string")

    }
    else {
        panic!("Keinen Wert gefunden");
    }

    /*
        let new_xhs = XHanaSIDHostInsert {
            solutionversion : &psolutionversion,
            sid : psid,
            hostname : phostname,
        }
    */

    // select solutionversion from xhana_solution_sid where sid = sid
    // wenn empty, dann fehler "keine Solution definiert für SID XXX"
    // wenn ok, dann
    // insert into xhanahost (hostname)
    // insert into xhana_sid_host (soltionversion, sid, hostname)
    /*
    let results = xhana_sid_host
        .filter(sid.eq(&sid))
        .get_results::<XHanaSIDHostTable>(conn)
        .expect("Error loading parameters");


    let new_xhv = XHanaSolutionSIDInsert {
        solutionversion: solutionversion,
        sid: sid,
        arc: arc,
        tag: match tag {
            "EMPTY" => "",
            _ => tag,
        },
    };

    diesel::insert_into(xhana_solution_sid::table)
        .values(&new_xhv)
        .on_conflict(xhana_solution_sid::sid)
        .do_update()
        .set(&new_xhv)
        .get_result(conn)
        .expect("Error saving new parameter string")
    */
}

/*
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
*/

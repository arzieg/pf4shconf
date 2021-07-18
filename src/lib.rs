// Postgres starten: pg_ctl -D /var/lib/pgsql/data -l logfile start
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_xhanageneral<'a>(
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
}

pub fn query_sid_version<'a>(conn: &PgConnection, psid: &str, pversion: &str) {
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
}

// Add HANA Parameter 
// Save dataset in table xhanaparameter
pub fn add_xhanaparameter<'a>(conn: &PgConnection, version: &'a str, info: &'a str,
    parameter: &'a str, typ: &'a str, mandatory: &'a str) -> XHanaParameterTable {
    
    use schema::xhanaparameter;

    let new_xhp = XHanaParameterInsert {
        version: version,
        parameter: parameter,
        info: info,
        typ: typ,
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

//    diesel::insert_into(xhanaparameter::table)
//        .values(&new_xhp)
//        .get_result(conn)
//        .expect("Error savong new parameter string")


}

pub fn query_hanaparameter<'a>(conn: &PgConnection, pversion: &str, pparameter: &str)  {
    use schema::xhanaparameter::dsl::*;

    println!("VERSION = {}", &pversion);

    let xhp = xhanaparameter
        .filter(version.eq(&pversion))
        .filter(version.eq(&pparameter))
        .get_results::<XHanaParameterTable>(conn)
        .expect("Error loading parameters");

    
 //   println!("Displaying {} XHanaParameter", results.len());
 //   for xhp in results {
 //       println!(
 //           "{} {} {} = {:?} ",
 //           xhp.version, xhp.parameter, xhp.typ, xhp.mandatory
 //       );
 //       println!("------------------------------------------------------------------\n");
 //   }
}

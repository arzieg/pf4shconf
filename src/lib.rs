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
use std::str;

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
pub fn add_xhana_host(conn: &PgConnection, phostname: &str, pdcid: &i32) -> XHanaHostTable {
    use schema::xhanahost;

    let new_xhh = XHanaHostInsert {
        hostname: phostname,
        dcid: pdcid,
    };

    diesel::insert_into(xhanahost::table)
        .values(&new_xhh)
        .on_conflict(xhanahost::hostname)
        .do_update()
        .set(&new_xhh)
        .get_result(conn)
        .expect("Error saving new parameter string")
}

pub fn add_xhana_sid_host<'a>(
    conn: &PgConnection,
    psid: &'a str,
    phostname: &'a str,
) -> XHanaSIDHostTable {
    use schema::xhana_sid_host;
    use schema::xhanahost;
    use schema::xhana_sid_host::dsl::*;
    use schema::xhana_solution_sid::dsl::*;
    use schema::xhanahost::dsl::*;

    let psolutionversion = xhana_solution_sid
        .select(schema::xhana_solution_sid::dsl::solutionversion)
        .filter(schema::xhana_solution_sid::dsl::sid.eq(&psid))
        .distinct()
        .load::<String>(conn)
        .expect("Could not get solutionversion from xhana_solution_sid");

    let pdcid = xhanahost
        .select(schema::xhanahost::dsl::dcid)
        .filter(schema::xhanahost::dsl::hostname.eq(&phostname))
        .distinct()
        .load::<i32>(conn)
        .expect("Could not get datacenter ID from xhanahost");
    
    let pdcid_i32 : i32 = pdcid[0];

    // Wenn ein Rückgabewert, dann weiter
    if psolutionversion.len() == 1 {
        let mut i_str = "";

        for i in psolutionversion.iter() {
            i_str = i.as_str();
        }

        println!("w= {:?}", &i_str);
        // now we update xhanahost and xhana_sid_host
        // for xhanahost I use a helper function
        let rv: XHanaHostTable = add_xhana_host(conn, phostname, &pdcid_i32);
        let new_xhs = XHanaSIDHostInsert {
            solutionversion: i_str,
            sid: psid,
            hostname: phostname,
        };

        diesel::insert_into(xhana_sid_host)
            .values(&new_xhs)
            .on_conflict((
                xhana_sid_host::solutionversion,
                xhana_sid_host::sid,
                xhana_sid_host::hostname,
            ))
            .do_update()
            .set(&new_xhs)
            .get_result(conn)
            .expect("Error saving new parameter string")
    } else {
        panic!("Keinen Wert gefunden");
    }
}

fn add_xhana_general(
    conn: &PgConnection,
    pparameterversion: &str,
    pparameter: &str,
    psolutionversion: &str,
    psid: &str,
    parc: &str,
    piotype: &str,
    pvalue: &str,
) -> XHanaGeneralTable {
    use schema::xhanageneral;
    use schema::xhanageneral::dsl::*;

    let new_xhg = XHanaGeneralInsert {
        parameterversion: pparameterversion,
        parameter: pparameter,
        solutionversion: psolutionversion,
        sid: psid,
        arc: parc,
        iotype: piotype,
        value: match pvalue {
            "EMPTY" => "",
            _ => pvalue,
        },
    };

    diesel::insert_into(xhanageneral)
        .values(&new_xhg)
        .on_conflict((
            xhanageneral::parameterversion,
            xhanageneral::parameter,
            xhanageneral::solutionversion,
        ))
        .do_update()
        .set(&new_xhg)
        .get_result(conn)
        .expect("Error saving new parameter string")
}

fn add_xhana_host_para(
    conn: &PgConnection,
    phostname: &str,
    pparameterversion: &str,
    parc: &str,
    pparameter: &str,
    piotype: &str,
    pvalue: &str,
) -> XHanaHostParaTable {
    use schema::xhana_host_para;
    use schema::xhana_host_para::dsl::*;

    let new_xhhp = XHanaHostParaInsert {
        hostname: phostname,
        parameterversion: pparameterversion,
        arc: parc,
        parameter: pparameter,
        iotype: piotype,
        value: match pvalue {
            "EMPTY" => "",
            _ => pvalue,
        },
    };

    diesel::insert_into(xhana_host_para)
        .values(&new_xhhp)
        .on_conflict((
            xhana_host_para::parameterversion,
            xhana_host_para::parameter,
            xhana_host_para::hostname,
            xhana_host_para::arc,
        ))
        .do_update()
        .set(&new_xhhp)
        .get_result(conn)
        .expect("Error saving new parameter string in xhana_host_para!")
}


fn add_xhana_sid_para(
    conn: &PgConnection,
    psid: &str,
    pparameterversion: &str,
    pparameter: &str,
    pvalue: &str,
    parc: &str,
    piotype: &str,
) -> XHanaSIDParaTable {
    use schema::xhana_sid_para;
    use schema::xhana_sid_para::dsl::*;

    let new_xhsp = XHanaSIDParaInsert {
        sid: psid,
        parameterversion: pparameterversion,
        parameter: pparameter,
        value: match pvalue {
            "EMPTY" => "",
            _ => pvalue,
        },
        arc: parc,
        iotype: piotype,        
    };

    diesel::insert_into(xhana_sid_para)
        .values(&new_xhsp)
        .on_conflict((
            xhana_sid_para::sid,
            xhana_sid_para::parameterversion,
            xhana_sid_para::parameter,
            xhana_sid_para::arc,
        ))
        .do_update()
        .set(&new_xhsp)
        .get_result(conn)
        .expect("Error saving new parameter string in xhana_sid_para!")
}


pub fn add_xhana_config(
    conn: &PgConnection,
    pparameterversion: &str,
    pparameter: &str,
    psolutionversion: &str,
    psid: &str,
    phostname: &str,
    pvalue: &str,
) {
    use schema::xhanaparameter::dsl::*;
    use schema::xhanasolution::dsl::*;
   
    let xhp = xhanaparameter
        .filter(schema::xhanaparameter::dsl::parameter.eq(&pparameter))
        .filter(schema::xhanaparameter::dsl::parameterversion.eq(&pparameterversion))
        .filter(
            schema::xhanaparameter::dsl::iotype
                .eq("input")
                .or(schema::xhanaparameter::dsl::iotype.eq("both")),
        )
        .load::<XHanaParameterTable>(conn)
        .expect("Error in Query xhanaparameter");

    // when no returnvalue then panic
    if xhp.len() == 0 {
        panic! ("No Parameter / Version combination found")
    }

    for i in xhp {
        if i.scope == "general" {
            if psolutionversion != "EMPTY" {
                let xhs = xhanasolution
                    .count()
                    .filter(schema::xhanasolution::dsl::solutionversion.eq(&psolutionversion))
                    .get_result::<i64>(conn)
                    .expect("Error in Query count xhanasolution");

                println!("xhs={}",xhs);

                if xhs > 0 {
                    add_xhana_general(
                        conn,
                        &*i.parameterversion,
                        &*i.parameter,
                        &psolutionversion,
                        psid,
                        &*i.arc,
                        &*i.iotype,
                        pvalue,
                    );
                } else {
                    println!("Parameterversion: {:?}", i.parameterversion);
                    println!("Parameter: {:?}", i.parameter);
                    println!("Info: {:?}", i.info);
                    println!("Scope: {:?}", i.scope);
                    println!("IOtype: {:?}", i.iotype);
                    println!("arc: {:?}", i.arc);
                    println!("Mandatory: {:?}", i.mandatory);
                    println!("Solutionversion: {:?}", psolutionversion);            
                    panic!("Sorry Solution {:?} is not definied!", &psolutionversion)
                };
            } else {
                panic!("You will set a value of a general parameter but a solution is not definied!")
            }
        }
        if i.scope == "host" {
            if phostname != "EMPTY" {
                    add_xhana_host_para(conn, phostname, &*i.parameterversion, 
                             &*i.arc, &*i.parameter,&*i.iotype, pvalue);
                    } 
            else {
                println!("Parameterversion: {:?}", i.parameterversion);
                println!("Parameter: {:?}", i.parameter);
                println!("Info: {:?}", i.info);
                println!("Scope: {:?}", i.scope);
                println!("IOtype: {:?}", i.iotype);
                println!("arc: {:?}", i.arc);
                println!("Mandatory: {:?}", i.mandatory);
                println!("Solutionversion: {:?}", psolutionversion);        
                panic!("You will set a value of a host parameter but no host is definied!")
            }
        }
        if i.scope == "sap" {
            if psid != "EMPTY" {
                add_xhana_sid_para(conn, psid, &*i.parameterversion,
                    &*i.parameter, pvalue, &*i.arc, &*i.iotype);
                    } 
            else {
                println!("Parameterversion: {:?}", i.parameterversion);
                println!("Parameter: {:?}", i.parameter);
                println!("Info: {:?}", i.info);
                println!("Scope: {:?}", i.scope);
                println!("IOtype: {:?}", i.iotype);
                println!("arc: {:?}", i.arc);
                println!("Mandatory: {:?}", i.mandatory);
                println!("Solutionversion: {:?}", psolutionversion);        
                panic!("You will set a value of a SID parameter but no SID is definied!")
            }
        }
        

        // add entry in xhanageneral
        // &* wandelt String in &str um
    }

    // fill xhanageneral
}
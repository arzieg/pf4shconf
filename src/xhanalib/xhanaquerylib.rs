use crate::diesel::*;
use crate::models::*;
use diesel::pg::PgConnection;

pub fn query_xhana_sid(conn: &PgConnection, psid: &str, pparameterversion: &str) {
    use crate::schema::xhana_sid_host::dsl::*;
    use crate::schema::xhana_sid_para::dsl::*;
    use crate::schema::xhana_solution_sid::dsl::*;
    use crate::schema::xhanasid::dsl::*;

    // Get SID Information
    let query_xhanasid = xhanasid
        .filter(crate::schema::xhanasid::sid.eq(&psid))
        .load::<XHanaSIDTable>(conn)
        .expect("Could not find SID in table xhanasid");

    if query_xhanasid.len() == 0 {
        println!("ERROR: Could not find SID={} in table xhanasid\n", &psid);
        panic!("Exiting");
    }

    // Get Solution information for SID
    let query_xhana_solution_sid = xhana_solution_sid
        .filter(crate::schema::xhana_solution_sid::sid.eq(&psid))
        .load::<XHanaSolutionSIDTable>(conn)
        .expect("Could not find SID in table xhana_solution_sid");

    if query_xhana_solution_sid.len() == 0 {
        println!(
            "ERROR: Could not find SID={} in table xhana_solution_sid\n",
            &psid
        );
        panic!("Exiting");
    }

    // Get HOST Information for SID
    let query_xhana_sid_host = xhana_sid_host
        .filter(crate::schema::xhana_sid_host::sid.eq(&psid))
        .load::<XHanaSIDHostTable>(conn)
        .expect("Could not find SID in table xhana_sid_host");

    if query_xhana_sid_host.len() == 0 {
        println!(
            "ERROR: Could not find SID={} in table xhana_sid_host\n",
            &psid
        );
        panic!("Exiting");
    }

    // Get Parameter Information for SID
    let query_xhana_sid_para = {
        if pparameterversion == "EMPTY" {
            xhana_sid_para
                .filter(crate::schema::xhana_sid_para::sid.eq(&psid))
                .load::<XHanaSIDParaTable>(conn)
                .expect("Could not find SID in table xhana_sid_para")
        } else {
            xhana_sid_para
                .filter(crate::schema::xhana_sid_para::sid.eq(&psid))
                .filter(crate::schema::xhana_sid_para::parameterversion.eq(&pparameterversion))
                .load::<XHanaSIDParaTable>(conn)
                .expect("Could not find SID or Parameterversion in table xhana_sid_para")
        }
    };

    if query_xhana_sid_para.len() == 0 {
        println!(
            "ERROR: Could not find SID={} or Parameterversion={} in table xhana_sid_para\n",
            &psid, &pparameterversion
        );
        panic!("Exiting");
    }

    println!("Report:");
    println!("=================================");

    for i in query_xhana_solution_sid.iter() {
        println!("Solutionname: {}", i.solutionversion);
        println!("Architecture: {}", i.arc);
        println!(
            "Tag: {}",
            match i.tag {
                None => "no tag is set",
                Some(ref x) => x,
            }
        );
    }

    for i in query_xhanasid.iter() {
        print!("SID: {}, ", i.sid);
        println!(
            "Name: {}",
            match i.name {
                None => "",
                Some(ref x) => x,
            }
        );
    }

    println!("-----------------------------------------------------------------");

    println!("Solution is running on host(s): ");
    for i in query_xhana_sid_host.iter() {
        println!("  Hostname: {}", i.hostname);
    }

    println!("-----------------------------------------------------------------");

    println!("SID specific parametervalues:");
    for i in query_xhana_sid_para.iter() {
        println!(
            "  Version: {}, Type: {}, {}={}",
            i.parameterversion, i.iotype, i.parameter, i.value
        );
    }
}

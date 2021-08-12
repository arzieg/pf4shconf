table! {
    xhana_host_para (hostname, parameterversion, parameter, arc) {
        hostname -> Varchar,
        parameterversion -> Varchar,
        dcid -> Nullable<Int4>,
        arc -> Varchar,
        parameter -> Varchar,
        iotype -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    xhana_sid_host (solutionversion, sid, hostname) {
        solutionversion -> Varchar,
        sid -> Varchar,
        hostname -> Varchar,
    }
}

table! {
    xhana_sid_para (sid, parameterversion, parameter, arc) {
        sid -> Varchar,
        parameterversion -> Varchar,
        parameter -> Varchar,
        value -> Varchar,
        arc -> Varchar,
        iotype -> Varchar,
    }
}

table! {
    xhana_solution_sid (sid) {
        solutionversion -> Varchar,
        sid -> Varchar,
        arc -> Varchar,
        tag -> Nullable<Varchar>,
    }
}

table! {
    xhanaarc (arc) {
        arc -> Varchar,
    }
}

table! {
    xhanadatacenter (dcid) {
        dcid -> Int4,
        name -> Varchar,
    }
}

table! {
    xhanageneral (parameterversion, parameter, solutionversion) {
        parameterversion -> Varchar,
        parameter -> Varchar,
        solutionversion -> Varchar,
        sid -> Varchar,
        value -> Varchar,
        arc -> Varchar,
        iotype -> Varchar,
    }
}

table! {
    xhanahost (hostname) {
        hostname -> Varchar,
    }
}

table! {
    xhanaparameter (parameterversion, parameter, arc, iotype) {
        parameterversion -> Varchar,
        parameter -> Varchar,
        info -> Nullable<Varchar>,
        scope -> Varchar,
        arc -> Varchar,
        iotype -> Varchar,
        valuetype -> Varchar,
        mandatory -> Nullable<Bpchar>,
    }
}

table! {
    xhanasid (sid) {
        sid -> Varchar,
        name -> Nullable<Varchar>,
    }
}

table! {
    xhanasolution (solutionversion) {
        solutionversion -> Varchar,
    }
}

joinable!(xhana_host_para -> xhanadatacenter (dcid));
joinable!(xhana_host_para -> xhanahost (hostname));
joinable!(xhana_sid_host -> xhanahost (hostname));
joinable!(xhana_sid_host -> xhanasid (sid));
joinable!(xhana_sid_host -> xhanasolution (solutionversion));
joinable!(xhana_sid_para -> xhanasid (sid));
joinable!(xhana_solution_sid -> xhanaarc (arc));
joinable!(xhana_solution_sid -> xhanasolution (solutionversion));
joinable!(xhanageneral -> xhanasolution (solutionversion));
joinable!(xhanaparameter -> xhanaarc (arc));

allow_tables_to_appear_in_same_query!(
    xhana_host_para,
    xhana_sid_host,
    xhana_sid_para,
    xhana_solution_sid,
    xhanaarc,
    xhanadatacenter,
    xhanageneral,
    xhanahost,
    xhanaparameter,
    xhanasid,
    xhanasolution,
);

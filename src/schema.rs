table! {
    xhanaarc (sid) {
        sid -> Varchar,
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
    xhanaenvironment (id) {
        id -> Int4,
        sid -> Varchar,
        version -> Varchar,
        hostname -> Varchar,
        parameter -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    xhanageneral (version, parameter) {
        version -> Varchar,
        parameter -> Varchar,
        value -> Varchar,
    }
}

table! {
    xhanaparameter (version, parameter) {
        version -> Varchar,
        parameter -> Varchar,
        info -> Nullable<Varchar>,
        valuetype -> Varchar,
        mandatory -> Nullable<Bpchar>,
    }
}

table! {
    xhanaversion (sid, version) {
        sid -> Varchar,
        version -> Varchar,
        tag -> Nullable<Varchar>,
    }
}

table! {
    xhost (hostid, parameter) {
        hostid -> Varchar,
        version -> Varchar,
        dcid -> Nullable<Int4>,
        hostname -> Nullable<Varchar>,
        parameter -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    xsid (sid, version, parameter) {
        sid -> Varchar,
        version -> Varchar,
        parameter -> Varchar,
        value -> Varchar,
    }
}

table! {
    xsid_host (sid, hostid, version) {
        sid -> Varchar,
        hostid -> Varchar,
        version -> Varchar,
    }
}

joinable!(xhost -> xhanadatacenter (dcid));

allow_tables_to_appear_in_same_query!(
    xhanaarc,
    xhanadatacenter,
    xhanaenvironment,
    xhanageneral,
    xhanaparameter,
    xhanaversion,
    xhost,
    xsid,
    xsid_host,
);

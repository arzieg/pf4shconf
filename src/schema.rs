table! {
    xhanaarc (sid) {
        sid -> Varchar,
        arc -> Varchar,
    }
}

table! {
    xhanadatacenter (id) {
        id -> Int4,
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
    xhanageneral (id) {
        id -> Int4,
        sid -> Varchar,
        version -> Varchar,
        parameter -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    xhanaparameter (version, parameter) {
        version -> Varchar,
        parameter -> Varchar,
        info -> Nullable<Varchar>,
        typ -> Varchar,
        mandatory -> Nullable<Bpchar>,
    }
}

table! {
    xhanaversion (id) {
        id -> Int4,
        sid -> Varchar,
        version -> Varchar,
        tag -> Nullable<Varchar>,
    }
}

table! {
    xhost (id) {
        id -> Int4,
        hostid -> Varchar,
        version -> Varchar,
        dc -> Nullable<Int4>,
        hostname -> Nullable<Varchar>,
        parameter -> Nullable<Varchar>,
        value -> Nullable<Varchar>,
    }
}

table! {
    xsid_host (id) {
        id -> Int4,
        sid -> Varchar,
        hostid -> Varchar,
        version -> Varchar,
    }
}

joinable!(xhost -> xhanadatacenter (dc));

allow_tables_to_appear_in_same_query!(
    xhanaarc,
    xhanadatacenter,
    xhanaenvironment,
    xhanageneral,
    xhanaparameter,
    xhanaversion,
    xhost,
    xsid_host,
);

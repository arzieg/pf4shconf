use super::schema::*;    // alle Tabellen sollen importiert werden

#[derive(Queryable)]
pub struct XHanaGeneral {
    pub id: i32,
    pub sid: String,
    pub version: String,
    pub parameter: String,
    pub value: Option<String>     // Option<String> wird hier benötigt, da es ein Nullable - Value in Schema 
}

#[derive(Queryable)]
pub struct XHanaSidVersionStruct {
    pub id: i32,
    pub sid: String,
    pub version: String,
}

#[derive(Insertable)]
#[table_name="xhanageneral"]
pub struct NewXHanaGeneral<'a> {
    pub sid: &'a str,
    pub version: &'a str,
    pub parameter: &'a str,
    pub value: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanaparameter"]
pub struct XHanaParameterInsert<'a> {
    pub version: &'a str,
    pub parameter: &'a str,
    pub info: &'a str,
    pub typ: &'a str,
    pub mandatory: &'a str,
}

#[derive(Queryable)]
pub struct XHanaParameterTable {
    pub version: String,
    pub parameter: String,
    pub info: Option<String>,
    pub typ: String,
    pub mandatory: Option<String>,
}

use super::schema::*;    // alle Tabellen sollen importiert werden

#[derive(Queryable)]
pub struct XHanaGeneral {
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
    pub scope: &'a str,
    pub valuetype: &'a str,
    pub mandatory: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanaarc"]
pub struct XHanaArcInsert<'a> {
    pub sid: &'a str,
    pub arc: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhanadatacenter"]
pub struct XHanaDCInsert<'a> {
    pub dcid: &'a i32,
    pub name: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhanaversion"]
pub struct XHanaVersionInsert<'a> {
    pub sid: &'a str,
    pub version: &'a str,
    pub tag: &'a str,
}

#[derive(Queryable)]
pub struct XHanaParameterTable {
    pub version: String,
    pub parameter: String,
    pub info: Option<String>,
    pub scope: String,
    pub valuetype: String,
    pub mandatory: Option<String>,
}

#[derive(Queryable)]
pub struct XHanaArcTable {
    pub sid: String,
    pub arc: String,
}

#[derive(Queryable)]
pub struct XHanaDCTable {
    pub dcid: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct XHanaVersionTable {
    pub sid: String,
    pub version: String,
    pub tag: Option<String>,
}

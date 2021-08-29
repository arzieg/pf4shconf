use super::schema::*;    // alle Tabellen sollen importiert werden

#[derive(Insertable,AsChangeset)]
#[table_name="xhanageneral"]
pub struct XHanaGeneralInsert<'a> {
    pub parameterversion: &'a str,
    pub parameter: &'a str,
    pub solutionversion: &'a str,
    pub sid: &'a str,
    pub value: &'a str,
    pub arc: &'a str,
    pub iotype: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanaparameter"]
pub struct XHanaParameterInsert<'a> {
    pub parameterversion: &'a str,
    pub parameter: &'a str,
    pub info: &'a str,
    pub scope: &'a str,
    pub arc: &'a str,
    pub iotype: &'a str,
    pub valuetype: &'a str,
    pub mandatory: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanaarc"]
pub struct XHanaArcInsert<'a> {
    pub arc: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanasolution"]
pub struct XHanaSolutionInsert<'a> {
    pub solutionversion: &'a str,
}

#[derive(Insertable,AsChangeset)]
#[table_name="xhanasid"]
pub struct XHanaSIDInsert<'a> {
    pub sid: &'a str,
    pub name: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhanadatacenter"]
pub struct XHanaDatacenterInsert<'a> {
    pub dcid: &'a i32,
    pub name: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhanahost"]
pub struct XHanaHostInsert<'a> {
    pub hostname: &'a str,
    pub dcid: &'a i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhana_solution_sid"]
pub struct XHanaSolutionSIDInsert<'a> {
    pub solutionversion: &'a str,
    pub sid: &'a str,
    pub arc: &'a str,
    pub tag: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhana_sid_host"]
pub struct XHanaSIDHostInsert<'a> {
    pub solutionversion: &'a str,
    pub sid: &'a str,
    pub hostname: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhana_sid_para"]
pub struct XHanaSIDParaInsert<'a> {
    pub sid: &'a str,
    pub parameterversion: &'a str,
    pub parameter: &'a str,
    pub value: &'a str,
    pub arc: &'a str,
    pub iotype: &'a str, 
}

#[derive(Insertable, AsChangeset)]
#[table_name="xhana_host_para"]
pub struct XHanaHostParaInsert<'a> {
    pub hostname: &'a str,
    pub parameterversion: &'a str,
    pub arc: &'a str,
    pub parameter: &'a str,
    pub iotype: &'a str,
    pub value: &'a str, 
}

// ab hier Queryable

#[derive(Queryable)]
pub struct XHanaParameterTable {
    pub parameterversion: String,
    pub parameter: String,
    pub info: Option<String>,
    pub scope: String,
    pub arc: String,
    pub iotype: String,
    pub valuetype: String,
    pub mandatory: Option<String>,
}

#[derive(Queryable)]
pub struct XHanaArcTable {
    pub arc: String,
}

#[derive(Queryable)]
pub struct XHanaSolutionTable {
    pub solutionversion: String,
}

#[derive(Queryable)]
pub struct XHanaSIDTable {
    pub sid: String,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct XHanaDatacenterTable {
    pub dcid: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct XHanaHostTable {
    pub hostname: String,
    pub dcid: i32,
}

#[derive(Queryable)]
pub struct XHanaSolutionSIDTable {
    pub solutionversion: String,
    pub sid: String,
    pub arc: String,
    pub tag: Option<String>,
}

#[derive(Queryable)]
pub struct XHanaSIDParaTable {
    pub sid: String,
    pub parameterversion: String,
    pub parameter: String,
    pub value: String,
    pub arc: String, 
    pub iotype: String,
}

#[derive(Queryable)]
pub struct XHanaSIDHostTable {
    pub solutionversion: String,
    pub sid: String,
    pub hostname: String,
}

#[derive(Queryable)]
pub struct XHanaHostParaTable {
    pub hostname: String,
    pub parameterversion: String,
    pub arc: String,
    pub parameter: String,
    pub iotype: String,
    pub value: Option<String>,
}

#[derive(Queryable)]
pub struct XHanaGeneralTable {
    pub parameterversion: String,
    pub parameter: String,
    pub solutionversion: String,
    pub sid: String,
    pub value: String,   
    pub arc: String,
    pub iotype: String,
}
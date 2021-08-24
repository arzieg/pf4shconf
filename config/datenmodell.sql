-- Modell 
delete from xHANA_HOST_PARA;
delete from xHANA_SID_PARA;
delete from xHANA_SID_HOST;
delete from xHANAGENERAL;
delete from xHANA_SOLUTION_SID;
delete from xHANAPARAMETER;
delete from xHANADATACENTER;
delete from xHANASOLUTION;
delete from xHANAARC;
delete from xHANASID;
delete from xHANAHOST;


insert into xhanadatacenter (dcid, name) values (1, 'DC1');
insert into xhanadatacenter (dcid, name) values (2, 'DC2');
insert into xhanadatacenter (dcid, name) values (3, 'DC3');

insert into xhanaarc (arc) values ('general');
insert into xhanaarc (arc) values ('SU');
insert into xhanaarc (arc) values ('SO');
insert into xhanaarc (arc) values ('SUSR');
insert into xhanaarc (arc) values ('SOSR');
insert into xhanaarc (arc) values ('ISCSI');
insert into xhanaarc (arc) values ('MajorityMaker');
insert into xhanaarc (arc) values ('Toolserver');
insert into xhanaarc (arc) values ('NetApp');

insert into xHANASID (sid, name) values ('Y04', 'HANA BW ScaleOut');
insert into xHANASID (sid, name) values ('Y03', 'HANA BW ScaleUp');
insert into xHANASID (sid, name) values ('Netapp01', 'NetApp im DC1');
insert into xHANASID (sid, name) values ('Toolserver', 'Der Toolserver');
insert into xHANASID (sid, name) values ('ISCSIServer', 'Der ISCSI-Server');

insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_SAPREPO_VERSION', 'SAP Repo', 'general', 'SO', 'both', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.2', 'PF4SH_SAPREPO_VERSION', 'SAP Repo', 'general', 'SO', 'both', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_<ARC>_<ID>_UID_SIDADM', 'UID SIDADM', 'host', 'SO', 'input', 'string', 'y');
 insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.2', 'PF4SH_<ARC>_<ID>_UID_SIDADM', 'UID SIDADM', 'host', 'SO', 'input', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_TO_NET_MACS1', 'MAC Adresse des Toolservers', 'host','Toolserver', 'both', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_NAS_<DC>', 'NAS Datacenter', 'host', 'NetApp', 'input', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_<ARC>_NET_MACS<ID>_<DC>', 'MAC Adresse des Host', 'host', 'SO', 'output', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_ISCSI_HOSTNAMES_<ID>', 'Hosts with ISCSI Devices', 'host', 'MajorityMaker', 'both', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_ISCSI_HOSTNAMES_<ID>', 'Hosts with ISCSI Devices', 'host', 'SO', 'both', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_ISCSI_TARGET', 'ISCSI Server', 'host', 'ISCSI', 'both', 'string', 'y');

insert into xhanasolution (solutionversion) values ('BWonHANA');
insert into xhanasolution (solutionversion) values ('S4HANA');

insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'Y04', 'SOSR', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'ISCSI-Y04', 'ISCSI', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'MM-Y04', 'MajorityMaker', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'TS-Y04', 'Toolserver', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'netapp01', 'NetApp', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'netapp02', 'NetApp', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'ISCSIServer', 'ISCSI', 'production' );


insert into xHANAGENERAL (parameterversion, parameter, solutionversion, sid, arc, iotype, value)
  values ('1.0', 'PF4SH_SAPREPO_VERSION', 'BWonHANA', 'Y04', 'SO', 'both', 'BWoH_Release_1');
insert into xHANAGENERAL (parameterversion, parameter, solutionversion, sid, arc, iotype, value)
  values ('1.2', 'PF4SH_SAPREPO_VERSION', 'BWonHANA', 'Y04', 'SO', 'both', 'BWoH_Release_3');

insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Y04', '1.0', 'PF4SH_<ARC>_<ID>_UID_SIDADM', '6500', 'SO', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Y04', '1.2', 'PF4SH_<ARC>_<ID>_UID_SIDADM', '6500', 'SO', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Netapp01', '1.0', 'PF4SH_NAS_<DC>', 'DC1', 'NetApp', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Toolserver', '1.0', 'PF4SH_TO_NET_MACS1', 'A:B:C:D:E:F', 'Toolserver', 'both');

insert into xHANAHOST (hostname) values ('hdb10y04-0001');
insert into xHANAHOST (hostname) values ('hdb10y04-0002');
insert into xHANAHOST (hostname) values ('laszis158');
insert into xHANAHOST (hostname) values ('laszis159');


insert into xHANA_HOST_PARA (hostname, parameterversion, dcid, parameter, arc, value, iotype) 
  values ('hdb10y04-0001', '1.0', '1', 'PF4SH_ISCSI_HOSTNAMES_<ID>', 'SO','hdb10y04-0001', 'both');
insert into xHANA_HOST_PARA (hostname, parameterversion, dcid, parameter, arc, value, iotype) 
  values ('laszis158', '1.0', '3', 'PF4SH_ISCSI_HOSTNAMES_<ID>', 'MajorityMaker', 'laszis158', 'both');
insert into xHANA_HOST_PARA (hostname, parameterversion, dcid, parameter, arc, value, iotype) 
  values ('laszis159', '1.0', '3', 'PF4SH_ISCSI_TARGET', 'ISCSI', 'laszis159', 'both');

insert into xHANA_SID_HOST (solutionversion, sid, hostname) 
 values ('BWonHANA', 'Y04', 'hdb10y04-0001');
insert into xHANA_SID_HOST (solutionversion, sid, hostname) 
 values ('BWonHANA', 'Y04', 'hdb10y04-0002');
insert into xHANA_SID_HOST (solutionversion, sid, hostname) 
 values ('BWonHANA', 'ISCSIServer', 'laszis159');



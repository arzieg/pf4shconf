-- Modell 
delete from xHANA_HOST_PARA;
delete from xHANA_SID_PARA;
delete from xHANA_SID_HOST;
delete from xHANAGENERAL;
delete from xHANA_SOLUTION_SID;
delete from xHANAPARAMETER;
delete from xHANAHOST;
delete from xHANADATACENTER;
delete from xHANASOLUTION;
delete from xHANAARC;
delete from xHANASID;



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

insert into xHANASID (sid, name) values ('Z01', 'HANA BW ScaleOut');
insert into xHANASID (sid, name) values ('Z02', 'HANA BW ScaleUp');
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
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_<ARC>_<ID>_HANA_SID', 'HANA SID', 'sap', 'SO', 'input', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.0', 'PF4SH_<ARC>_<ID>_HANA_INR', 'HANA SystemNR', 'sap', 'SO', 'input', 'string', 'y');
insert into xhanaparameter (parameterversion, parameter, info, scope, arc, iotype, valuetype, mandatory) 
 values ('1.2', 'PF4SH_<ARC>_<ID>_HANA_INR', 'HANA SystemNR', 'sap', 'SU', 'input', 'string', 'y');


insert into xhanasolution (solutionversion) values ('BWonHANA');
insert into xhanasolution (solutionversion) values ('S4HANA');

insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'Z01', 'SOSR', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'ISCSI-Z01', 'ISCSI', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'MM-Z01', 'MajorityMaker', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'TS-Z01', 'Toolserver', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'netapp01', 'NetApp', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'netapp02', 'NetApp', 'production' );
insert into xHANA_SOLUTION_SID (solutionversion, sid, arc, tag) 
 values ('BWonHANA', 'ISCSIServer', 'ISCSI', 'production' );


insert into xHANAGENERAL (parameterversion, parameter, solutionversion, sid, arc, iotype, value)
  values ('1.0', 'PF4SH_SAPREPO_VERSION', 'BWonHANA', 'Z01', 'SO', 'both', 'BWoH_Release_1');
insert into xHANAGENERAL (parameterversion, parameter, solutionversion, sid, arc, iotype, value)
  values ('1.2', 'PF4SH_SAPREPO_VERSION', 'BWonHANA', 'Z01', 'SO', 'both', 'BWoH_Release_3');

insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Z01', '1.0', 'PF4SH_<ARC>_<ID>_UID_SIDADM', '6500', 'SO', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Z01', '1.2', 'PF4SH_<ARC>_<ID>_UID_SIDADM', '6500', 'SO', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Netapp01', '1.0', 'PF4SH_NAS_<DC>', 'DC1', 'NetApp', 'input');
insert into xHANA_SID_PARA (sid, parameterversion, parameter, value, arc, iotype) 
  values ('Toolserver', '1.0', 'PF4SH_TO_NET_MACS1', 'A:B:C:D:E:F', 'Toolserver', 'both');

insert into xHANAHOST (hostname, dcid) values ('hana10z01-0001', 1);
insert into xHANAHOST (hostname, dcid) values ('hana10z01-0002', 1);
insert into xHANAHOST (hostname, dcid) values ('majority01', 3);
insert into xHANAHOST (hostname, dcid) values ('iscsisrv01', 3);


insert into xHANA_HOST_PARA (hostname, parameterversion, parameter, arc, value, iotype) 
  values ('hana10z01-0001', '1.0', 'PF4SH_<ARC>_<ID>_UID_SIDADM', 'SO','hana10z01-0001', 'input');
insert into xHANA_HOST_PARA (hostname, parameterversion, parameter, arc, value, iotype) 
  values ('majority01', '1.0', 'PF4SH_ISCSI_HOSTNAMES_<ID>', 'MajorityMaker', 'majority01', 'both');
insert into xHANA_HOST_PARA (hostname, parameterversion, parameter, arc, value, iotype) 
  values ('iscsisrv01', '1.0', 'PF4SH_ISCSI_TARGET', 'ISCSI', 'iscsisrv01', 'both');

insert into xHANA_SID_HOST (solutionversion, sid, hostname, arc) 
 values ('BWonHANA', 'Z01', 'hana10z01-0001', 'SOSR');
insert into xHANA_SID_HOST (solutionversion, sid, hostname, arc) 
 values ('BWonHANA', 'Z01', 'hana10z01-0002', 'SOSR');
insert into xHANA_SID_HOST (solutionversion, sid, hostname, arc) 
 values ('BWonHANA', 'ISCSIServer', 'iscsisrv01', 'ISCSI');



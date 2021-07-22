-- diesel hat hier ein Problem. 
-- Vorgehen: diesel migration generate setup
--           dann die down.sql und up.sql pflegen
--           diesel migration run

--  Es gibt zwei Schlüsselfelder für die Versionierung. 
--    1. SID  = HANA SID 
--    2. VER  = Versionsstring
-- Tabellen
--   xHANAGENERAL     - generelle Parameter für die Gesamtsystemumgebung
--   xHANAENVIRONMENT - maschinenspezifische Parameter in der PF4SH_POST.conf
--*   xHANAARC         - Architektur (ScaleUp oder ScaleOut)
--*   xHANAVERSION     - Versionstabelle je SID
--*   xHANAPARAMETER   - Parametertabelle 

-- xHANAGENERAL
-- Abgespeichert werden hier die allgemeinen Parameter für die Gesamtumgebung
--   Key: SID, VERSION
--CREATE TABLE xHANAGENERAL (
--  id SERIAL PRIMARY KEY,
--  sid VARCHAR NOT NULL,
--  version VARCHAR NOT NULL,
--  parameter VARCHAR NOT NULL,
--  value VARCHAR
--);

-- xHANAARC
-- Architekturtyp je SID
-- Key: SID
CREATE TABLE xHANAARC (
  sid VARCHAR NOT NULL PRIMARY KEY UNIQUE,
  arc VARCHAR NOT NULL
);
 
-- xHANAVERSION
-- xHANA Versionstablle
-- Key: SID, VERSION
CREATE TABLE xHANAVERSION (
  sid VARCHAR NOT NULL REFERENCES xHANAARC, 
  version VARCHAR NOT NULL,
  tag VARCHAR,
  primary key(sid, version)
);

-- XHANAENVIRONMENT
-- maschinenspezifische Parameter für die Umgebung
--  Key: SID, VERSION
CREATE TABLE xHANAENVIRONMENT (
  id SERIAL PRIMARY KEY,
  sid VARCHAR NOT NULL,
  version VARCHAR NOT NULL,
  hostname VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  value VARCHAR
);




-- xHANAPARAMETER
-- xHANA Parametertabelle. In dieser Tabelle stehen die auszufüllenden Parameter und ob diese
-- Mandatory oder Optional sind.
CREATE TABLE xHANAPARAMETER (
  version VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  info VARCHAR,
  scope VARCHAR NOT NULL,
  iotype VARCHAR NOT NULL,
  valuetype VARCHAR NOT NULL,
  mandatory CHAR, 
  primary key(version, parameter)
);


-- xHANADATACENTER
-- Datacenter IDs
CREATE TABLE xHANADATACENTER (
  dcid INTEGER NOT NULL PRIMARY KEY UNIQUE,
  name VARCHAR NOT NULL
);

-- xHOST
-- HOST Informationen
CREATE TABLE xHOST (
  id SERIAL PRIMARY KEY,
  hostid VARCHAR NOT NULL,
  version VARCHAR NOT NULL, 
  dcid INTEGER REFERENCES xHANADATACENTER,
  hostname VARCHAR,
  parameter VARCHAR, 
  value VARCHAR,
  FOREIGN KEY (version, parameter) REFERENCES xHANAPARAMETER (version, parameter)
--  CONSTRAINT fk_dcid_xhanadatacenter
--    FOREIGN KEY(dcid) 
--	    REFERENCES xHANADATACENTER(dcid)
);

-- xSID_HOST
-- Mapping SID to HOST
CREATE TABLE xSID_HOST (
  sid VARCHAR NOT NULL REFERENCES xHANAARC, 
  hostid VARCHAR NOT NULL, 
  version VARCHAR NOT NULL,
  id INTEGER NOT NULL REFERENCES xHOST,
  primary key(sid,hostid,version)
);

-- xHANAGENERAL
-- allg. Parameter
CREATE TABLE xHANAGENERAL (
  version VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  value VARCHAR NOT NULL,
  primary key(version, parameter),
  FOREIGN KEY (version, parameter) REFERENCES xHANAPARAMETER (version, parameter)
);

-- xSID
-- Parameter specific to SID
CREATE TABLE xSID (
  sid VARCHAR NOT NULL REFERENCES xHANAARC,
  version VARCHAR NOT NULL, 
  parameter VARCHAR NOT NULL, 
  value VARCHAR NOT NULL,
  primary key(sid, version, parameter),
  FOREIGN KEY (version, parameter) REFERENCES xHANAPARAMETER (version, parameter)
);
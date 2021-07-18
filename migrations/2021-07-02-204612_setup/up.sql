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

-- xHANAARC
-- Architekturtyp je SID
-- Key: SID
CREATE TABLE xHANAARC (
  sid VARCHAR NOT NULL,
  arc VARCHAR NOT NULL,
  primary key(sid)
);
 
-- xHANAVERSION
-- xHANA Versionstablle
-- Key: SID, VERSION
CREATE TABLE xHANAVERSION (
  id SERIAL PRIMARY KEY,
  sid VARCHAR NOT NULL,
  version VARCHAR NOT NULL,
  tag VARCHAR
);

-- xHANAPARAMETER
-- xHANA Parametertabelle. In dieser Tabelle stehen die auszufüllenden Parameter und ob diese
-- Mandatory oder Optional sind.
CREATE TABLE xHANAPARAMETER (
  version VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  info VARCHAR,
  typ VARCHAR NOT NULL,
  mandatory CHAR, 
  primary key(version, parameter)
);


-- xHANADATACENTER
-- Datacenter IDs
CREATE TABLE xHANADATACENTER (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL
);

-- xHOST
-- HOST Informationen
CREATE TABLE xHOST (
  id SERIAL PRIMARY KEY,
  hostid VARCHAR NOT NULL,
  version VARCHAR NOT NULL, 
  dc INTEGER,
  hostname VARCHAR,
  parameter VARCHAR, 
  value VARCHAR,
  CONSTRAINT fk_dc
    FOREIGN KEY(dc) 
	    REFERENCES xHANADATACENTER(id)
--  CONSTRAINT fk_parameter
--    FOREIGN KEY(parameter) 
--	    REFERENCES xHANAPARAMETER(id)
);

-- xSID_HOST
-- Mapping SID to HOST
CREATE TABLE xSID_HOST (
  id SERIAL PRIMARY KEY,
  sid VARCHAR NOT NULL, 
  hostid VARCHAR NOT NULL, 
  version VARCHAR NOT NULL
);

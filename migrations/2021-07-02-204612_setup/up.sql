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
  arc VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE xHANASOLUTION (
  solutionversion VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE xHANASID (
  sid VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR
);

-- xHANADATACENTER
-- Datacenter IDs
CREATE TABLE xHANADATACENTER (
  dcid INTEGER NOT NULL PRIMARY KEY UNIQUE,
  name VARCHAR NOT NULL
);

CREATE TABLE xHANAHOST (
  hostname VARCHAR NOT NULL PRIMARY KEY UNIQUE
);


-- xHANAPARAMETER
-- xHANA Parametertabelle. In dieser Tabelle stehen die auszufüllenden Parameter und ob diese
-- Mandatory oder Optional sind.
CREATE TABLE xHANAPARAMETER (
  parameterversion VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  info VARCHAR,
  arc VARCHAR NOT NULL REFERENCES xHANAARC,
  iotype VARCHAR NOT NULL,
  valuetype VARCHAR NOT NULL,
  mandatory CHAR, 
  primary key(parameterversion, parameter, arc)
);
 
-- xSID
-- Parameter specific to SID
CREATE TABLE xHANA_SID_PARA (
  sid VARCHAR NOT NULL REFERENCES xHANASID,
  parameterversion VARCHAR NOT NULL, 
  parameter VARCHAR NOT NULL, 
  value VARCHAR NOT NULL,
  arc VARCHAR NOT NULL,
  primary key(sid, parameterversion, parameter, arc),
  FOREIGN KEY (parameterversion, parameter, arc) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc)
);

-- xHANAVERSION
-- xHANA Versionstablle
-- Key: SID, VERSION
CREATE TABLE xHANA_SOLUTION_SID (
  solutionversion VARCHAR NOT NULL REFERENCES xHANASOLUTION, 
  sid VARCHAR NOT NULL,
  arc VARCHAR NOT NULL REFERENCES xHANAARC,
  tag VARCHAR,
  primary key (sid)
);

-- xSID_HOST
-- Mapping SID to HOST
CREATE TABLE xHANA_SID_HOST (
  solutionversion VARCHAR NOT NULL REFERENCES xHANASOLUTION,
  sid VARCHAR NOT NULL REFERENCES xHANASID,
  hostname VARCHAR  NOT NULL REFERENCES xHANAHOST,
  primary key(solutionversion, sid, hostname)
);

-- xHOST
-- HOST Informationen
CREATE TABLE xHANA_HOST_PARA (
  hostname VARCHAR NOT NULL REFERENCES xHANAHOST,
  parameterversion VARCHAR NOT NULL, 
  dcid INTEGER REFERENCES xHANADATACENTER,
  arc VARCHAR NOT NULL,
  parameter VARCHAR, 
  value VARCHAR,
  PRIMARY KEY (hostname, parameterversion, parameter,arc),
  FOREIGN KEY (parameterversion, parameter,arc) REFERENCES xHANAPARAMETER (parameterversion, parameter,arc)
);

-- xHANAGENERAL
-- allg. Parameter
CREATE TABLE xHANAGENERAL (
  parameterversion VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL, 
  solutionversion VARCHAR NOT NULL,
  sid VARCHAR NOT NULL,
  value VARCHAR NOT NULL,
  arc VARCHAR NOT NULL,
  primary key(parameterversion, parameter, solutionversion),
  FOREIGN KEY (parameterversion, parameter, arc) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc),
  FOREIGN KEY (solutionversion) REFERENCES xHANASOLUTION (solutionversion)
);


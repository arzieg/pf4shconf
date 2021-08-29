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
-- Tabelle der Architekturtypen
-- Architekturtypen
--  SO = ScaleOut
--  SOR = ScaleOurReplication
--  SUR = ScaleUpReplication
--  SU = ScaleUp
--  ISCSI = iscsi Server
--  MajoritzMaker = MajorityMaker
--  Toolserver = Toolserver
--  NetApp = NetApp
CREATE TABLE xHANAARC (
  arc VARCHAR NOT NULL PRIMARY KEY
);
-- Tabelle der HANA Lösungen. Dies ist ein Freitextfeld
CREATE TABLE xHANASOLUTION (
  solutionversion VARCHAR NOT NULL PRIMARY KEY
);

-- Tabelle der SIDs (hierzu gehören z.B. auch ISCSI-Systeme und nicht nur SAP Systeme) + 
-- Freitext als Beschreibung
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

-- Tabelle der Hostnamen
CREATE TABLE xHANAHOST (
  hostname VARCHAR NOT NULL PRIMARY KEY UNIQUE,
  dcid INTEGER NOT NULL REFERENCES xHANADATACENTER
);


-- xHANAPARAMETER
-- xHANA Parametertabelle. In dieser Tabelle stehen die auszufüllenden Parametertemplates.
--   parameterversion = Version des Parametertemplates
--   parameter = Parametertemplate
--   info = Erklärungstext zum Parameter
--   arc = Architektur
--   iotype = input / output / both?
--   valuetype = erwarteter Wert (wird aktuell nicht abgefragt)
--   mandatory = (J)a / (N)ein
CREATE TABLE xHANAPARAMETER (
  parameterversion VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL,
  info VARCHAR,
  scope VARCHAR NOT NULL,
  arc VARCHAR NOT NULL REFERENCES xHANAARC,
  iotype VARCHAR NOT NULL,
  valuetype VARCHAR NOT NULL,
  mandatory CHAR, 
  primary key(parameterversion, parameter, arc, iotype)
);
 
-- Verbindungstabelle SID zu Parametern
CREATE TABLE xHANA_SID_PARA (
  sid VARCHAR NOT NULL REFERENCES xHANASID,
  parameterversion VARCHAR NOT NULL, 
  parameter VARCHAR NOT NULL, 
  value VARCHAR NOT NULL,
  arc VARCHAR NOT NULL,
  iotype VARCHAR NOT NULL,
  primary key(sid, parameterversion, parameter, arc),
  FOREIGN KEY (parameterversion, parameter, arc, iotype) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc, iotype)
);

-- Verbindungstabelle Solution zu SID
--  Solutionversion 
--  SID
--  Architekturtyp
--  Tag (Freitextfeld)
CREATE TABLE xHANA_SOLUTION_SID (
  solutionversion VARCHAR NOT NULL REFERENCES xHANASOLUTION, 
  sid VARCHAR NOT NULL,
  arc VARCHAR NOT NULL REFERENCES xHANAARC,
  tag VARCHAR,
  primary key (sid)
);

-- Mappingtabelle SID zu HOST
CREATE TABLE xHANA_SID_HOST (
  solutionversion VARCHAR NOT NULL REFERENCES xHANASOLUTION,
  sid VARCHAR NOT NULL REFERENCES xHANASID,
  hostname VARCHAR  NOT NULL REFERENCES xHANAHOST,
  primary key(solutionversion, sid, hostname)
);

-- Mappingtabelle Host zu Parametertemplate
CREATE TABLE xHANA_HOST_PARA (
  hostname VARCHAR NOT NULL REFERENCES xHANAHOST,
  parameterversion VARCHAR NOT NULL, 
  -- dcid INTEGER NOT NULL REFERENCES xHANADATACENTER,
  arc VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL, 
  iotype VARCHAR NOT NULL,
  value VARCHAR,
  PRIMARY KEY (hostname, parameterversion, parameter, arc),
  FOREIGN KEY (parameterversion, parameter, arc, iotype) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc, iotype)
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
  iotype VARCHAR NOT NULL,
  primary key(parameterversion, parameter, solutionversion),
  FOREIGN KEY (parameterversion, parameter, arc, iotype) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc, iotype),
  FOREIGN KEY (solutionversion) REFERENCES xHANASOLUTION (solutionversion)
);


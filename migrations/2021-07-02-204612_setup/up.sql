-- diesel hat hier ein Problem. 
-- Vorgehen: diesel migration generate setup
--           dann die down.sql und up.sql pflegen
--           diesel migration run

-- Wo möglich, gibt es ein Versionstring
-- 
-- Folgende Tabellen werden angelegt: 
--   xHANAARC           - Architekturtypen
--   xHANASOLUTION      - Name der Lösung
--   xHANADATACENTER    - Name der Datacenter
--   xHANASID           - SID des Systems
--   xHANAHOST          - Tabelle der Server
--   xHANAPARAMETER     - Templatetabelle der Parameter
--   xHANA_SID_PARA     - SID spezifische Parameter
--   xHANA_SOLUTION_SID - Mappingtabelle SID zur Lösung
--   xHANA_SID_HOST     - Mappingtabelle SID zu Server
--   xHANA_HOST_PARA    - HOST spezifische Parameter
--   xHANAGENERAL       - Allgemeine Parameter je Lösung

-- xHANAARC
-- Tabelle der Architekturtypen
-- Architekturtypen
--  SO = ScaleOut
--  SOR = ScaleOurReplication
--  SUR = ScaleUpReplication
--  SU = ScaleUp
--  ISCSI = iscsi Server
--  MajorityMaker = MajorityMaker
--  Toolserver = Toolserver
--  NetApp = NetApp
CREATE TABLE xHANAARC (
  arc VARCHAR NOT NULL PRIMARY KEY
);

-- Tabelle der Lösungen. Dies ist ein Freitextfeld
CREATE TABLE xHANASOLUTION (
  solutionversion VARCHAR NOT NULL PRIMARY KEY
);

-- Tabelle der SIDs, d.h. jedes System hat eine SID (nicht nur die SAP Systeme)
-- Freitext als Beschreibung
CREATE TABLE xHANASID (
  sid VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR
);

-- Datacenter IDs und Name als Freitext
CREATE TABLE xHANADATACENTER (
  dcid INTEGER NOT NULL PRIMARY KEY UNIQUE,
  name VARCHAR NOT NULL
);

-- Tabelle der Hostnamen und wo sie stehen
CREATE TABLE xHANAHOST (
  hostname VARCHAR NOT NULL PRIMARY KEY UNIQUE,
  dcid INTEGER NOT NULL REFERENCES xHANADATACENTER
);


-- xHANA Parametertabelle. In dieser Tabelle stehen die auszufüllenden Parametertemplates.
--   parameterversion = Version des Parametertemplates
--   parameter = Parametertemplate
--   info = Erklärungstext zum Parameter
--   scope = Gruppe der Parameter, mögliche Schlüsselwörter: general, host, sap
--   arc = Architekturtyp
--   iotype = Eingabe, Ausgabe oder Beides, Schlüsselwörter: input, output, both
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
 
-- Hier werden die SID spezifischen Parameter gespeichert
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

-- Mappingtabelle Solution zu SID
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
  arc VARCHAR NOT NULL REFERENCES xHANAARC,
  primary key(solutionversion, sid, hostname)
);

-- Hostspezifische Parameter
CREATE TABLE xHANA_HOST_PARA (
  hostname VARCHAR NOT NULL REFERENCES xHANAHOST,
  parameterversion VARCHAR NOT NULL, 
  arc VARCHAR NOT NULL,
  parameter VARCHAR NOT NULL, 
  iotype VARCHAR NOT NULL,
  value VARCHAR,
  PRIMARY KEY (hostname, parameterversion, parameter, arc),
  FOREIGN KEY (parameterversion, parameter, arc, iotype) REFERENCES xHANAPARAMETER (parameterversion, parameter, arc, iotype)
);

-- Lösungsspezifische, allgemeine Parameter
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


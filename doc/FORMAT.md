Capture One catalog format
==========================

Based on macOS, version 11 and 12.

The Capture One Catalog is a bundle, ie a directory with other files.

Catalog
+-> Capture One Catalog.cocatalogdb
       The main file, a sqlite3 database file
       First observation, both version 11 and 12 share the same schema.
+-> Originals/
       Where the origingal files "In catalog" are stored.
       See ZPATHLOCATION with ZISRELATIVE set to 1.
+-> Cache/


cocatalogdb
-----------

Tables:

ZDOCUMENTCONTENT

- Z_ENT
- Z_PK
- ZROOTCOLLECTION: (integer) id of the root collection

ZVERSIONINFO

Version information

- Z_ENT: always matched "VersionInfo".
- ZAUTHOR: (string) the version of the app.
- ZVERSION: numeric version. 1200 for 12.0, 11.06 of 11.3
- ZCOMPATIBLEVERSION which version it compatible with. Like ZVERSION.
- ZFORMAT: (string) indicate format. Same values as ZAUTHOR.
- ZCOMPATIBILITY (string) ???.

ZENTITIES

Association between table row and typed data.
Join on Z_ENT columns.

- Z_ENT: (integer) entity id.
- ZNAME: (string) the entity name.

ZCOLLECTION

The collections. Can be any type. Use the entity type to identify it.

- Z_ENT: the entity type.
- ZNAME: (string) name of the collection (or null). Restricted to certain types.
- ZFOLDERLOCATION: key to join with table ZPATHLOCATION
  For "CatalogFolderCollection" type.
- ZCOLLECTIONINDX: the index (order) of the collection in the UI
- ZPARENT the parent collection (or null). Join on Z_PK.
- ZDATECREATED / ZDATEMODIFIED creation and modification date.
- ZSORTORDER the key to sort on (string)
- ZPRIMARYVARIANT ???? (a ZIMAGE?)


ZPATHLOCATION

The path locations for "CatalogFolderCollection" entities

- Z_ENT always a match on "PathLocation"
- Z_PK primary key
- ZWINROOT / ZMACROOT the Windows or macOS root. Empty if ZISRELATIVE is 1
- ZISRELATIVE 1 of the path is relative to the catalog
- ZRELATIVEPATH the path to the folder from the root or the catalog.
- ZVOLUME empty if ZISRELATIVE is 1. Otherwise the volume name.
- ZWINATTRIBUTE

ZKEYWORD

- Z_ENT always a match on "Keyword"
- Z_PK primary key
- ZNAME keyword name.
- ZPARENT id of parent keyword.
- ZLEFT / ZRIGHT ???
- ZISEXPORTABLE
- ZDESCRIPTION

ZIMAGE

The images.

- Z_ENT: matches "Image"


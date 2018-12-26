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
  - ProjectCollection: the root
    Column ZTRASHCOLLECTION contain id of trash collection
  - VirtualFolderCollection: folder in the tree
  - CatalogAllImagesCollection: all images
  - TrashCollection: the trash collection
  - CatalogInternalImagesCollection: the images stored in the catalog
  - AlbumCollection: user collections (albums)
  - CatalogFolderCollection: filesystem folders. ZNAME is null.
- ZNAME: (string) name of the collection (or null). Restricted to certain types.
- ZFOLDERLOCATION: key to join with table ZPATHLOCATION
  For "CatalogFolderCollection" type.
- ZCOLLECTIONINDX: the index (order) of the collection in the UI.
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

The images, original files. Each seems to have a variant.

- Z_ENT: matches "Image".
- Z_PK: the id.
- ZIMAGEUUID: image uuid.
- ZSIDECARPATH: basename path from the Cache subdir for the sidecar
  (cop and cof files).
- ZDISPLAYNAME (string) the name for display.
- ZISINSIDECATALOG (integer) whether the image is in the catalog or not.
- ZIMAGELOCATION (integer) join on ZPATHLOCATION.Z_PK.
- ZIMAGEFILENAME (stirng) the file name in its directory.
- ZIMAGECLASSIFICATION (integer) type of image
  - 19 for JPEG
  - 17 for RAW (RAF in this case, unsure about other varieties)
  - 6 for MOV
- ZISTRASHED (integer) true if in trash.
- ZEXP_FORMAT (string) format for export ???
  -JPEG, RAW, MOVIE
- ZFILE_SIZE: file size in bytes.
- ZWIDTH and ZHEIGHT image dimensions.
- ZGPSALTITUDE, ZGPSLATITUDE, ZGPSLONGITUDE: GPS position.
(lot of other metadata related columns)

ZVARIANT

Variant are an edited version of an image.

- Z_ENT: matches "Variant".


ZPROCESSHISTORY

What has been exported.
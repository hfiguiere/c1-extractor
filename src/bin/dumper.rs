/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate c1;

use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use c1::{Catalog, CatalogVersion, CoId, Collection, Folder, Image, Keyword, KeywordTree, Stack};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Dump(DumpArgs),
    List(ListArgs),
    Audit,
}

#[derive(Debug, Parser)]
struct DumpArgs {
    /// Path to the catalog.
    path: PathBuf,
    /// Dump all.
    #[arg(long)]
    all: bool,
    /// Dump collections.
    #[arg(long)]
    collections: bool,
    /// Dump images.
    #[arg(long)]
    images: bool,
    /// Dump stacks.
    #[arg(long)]
    stacks: bool,
    /// Dump folders.
    #[arg(long)]
    folders: bool,
    /// Dump keywords.
    #[arg(long)]
    keywords: bool,
}

#[derive(Debug, Parser)]
struct ListArgs {
    /// Path to the catalog.
    path: PathBuf,
    #[arg(short)]
    dirs: bool,
    #[arg(short)]
    sort: bool,
}

fn main() -> c1::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::List(args) => process_list(&args),
        Command::Dump(args) => process_dump(&args),
        Command::Audit => process_audit(&args),
    }
}

fn process_list(args: &ListArgs) -> c1::Result<()> {
    let mut catalog = Catalog::new(&args.path);
    catalog.open()?;

    // XXX this is stupid everything fails if this isn't called.
    catalog.load_version()?;
    let folders = catalog.load_folders();

    let resolved_folders = BTreeMap::from_iter(folders.iter().map(|folder| {
        let resolved = if folder.is_relative {
            format!("./{}", folder.path_from_root)
        } else {
            format!("{}{}", folder.root_folder, folder.path_from_root)
        };
        (folder.id(), resolved)
    }));
    if args.dirs {
        let mut dirs = resolved_folders.values().collect::<Vec<&String>>();
        if args.sort {
            dirs.sort_unstable();
        }
        dirs.iter().for_each(|folder| println!("{}", folder));

        return Ok(());
    }

    let images = catalog.load_images();
    let mut image_files = images
        .iter()
        .filter_map(|image| {
            resolved_folders
                .get(&image.folder)
                .map(|folder| format!("{}/{}", folder, image.file_name))
        })
        .collect::<Vec<String>>();
    if args.sort {
        image_files.sort_unstable();
    }
    image_files.iter().for_each(|file| println!("{file}"));

    Ok(())
}

fn process_dump(args: &DumpArgs) -> c1::Result<()> {
    let mut catalog = Catalog::new(&args.path);
    catalog.open()?;

    catalog.load_version()?;
    println!("Catalog:");
    println!(
        "\tVersion: {} ({:?})",
        catalog.version, catalog.catalog_version
    );
    println!("\tRoot collection id: {}", catalog.root_collection_id);

    match catalog.catalog_version {
        CatalogVersion::Co1210 | CatalogVersion::Co1200 | CatalogVersion::Co1106 => {}
        _ => {
            println!("Unsupported catalog version");
            return Err(c1::Error::UnsupportedVersion);
        }
    }

    {
        let keywordtree = catalog.load_keywords_tree();
        let keywords = catalog.load_keywords();

        if args.all || args.keywords {
            dump_keywords(0, keywords, &keywordtree);
        }
    }

    {
        let folders = catalog.load_folders();
        if args.all || args.folders {
            dump_folders(folders);
        }
    }

    {
        let images = catalog.load_images();
        if args.all || args.images {
            dump_images(images);
        }
    }

    {
        let stacks = catalog.load_stacks();
        if args.all || args.stacks {
            dump_stacks(stacks);
        }
    }

    {
        let collections = catalog.load_collections();
        if args.all || args.collections {
            dump_collections(collections);
        }
    }

    Ok(())
}

fn print_keyword(level: i32, id: i64, keywords: &BTreeMap<CoId, Keyword>, tree: &KeywordTree) {
    if let Some(keyword) = keywords.get(&id) {
        let mut indent = String::from("");
        if level > 0 {
            for _ in 0..level - 1 {
                indent.push(' ');
            }
            indent.push_str("+ ")
        }
        println!(
            "| {:>7} | {:>7} | {}{}",
            keyword.id(),
            keyword.parent,
            indent,
            keyword.name
        );
        let children = tree.children_for(id);
        for child in children {
            print_keyword(level + 1, child, keywords, tree);
        }
    }
}

fn dump_keywords(root: i64, keywords: &BTreeMap<i64, Keyword>, tree: &KeywordTree) {
    println!("Keywords");
    println!("+---------+---------+----------------------------");
    println!("| id      | parent  | name");
    println!("+---------+---------+----------------------------");
    let children = tree.children_for(root);
    for child in children {
        print_keyword(0, child, keywords, tree);
    }
    println!("+---------+---------+----------------------------");
}

fn dump_folders(folders: &[Folder]) {
    println!("Folders");
    println!("+---------+---------+-------+----------------------------+----------");
    println!("| id      | root    | relat | path                        |");
    println!("+---------+---------+-------+----------------------------+----------");
    for folder in folders {
        println!(
            "| {:>7} | {:<7} | {:<5} | {:<26}",
            folder.id(),
            folder.root_folder,
            folder.is_relative,
            folder.path_from_root
        );
    }
    println!("+---------+---------+-------+----------------------------+----------");
}

fn dump_images(images: &[Image]) {
    println!("Images");
    println!("+---------+--------------------------------------+----------+--------+----+--------------+");
    println!("| id      | uuid                                 | DisplayN | format | cl | file name    |");
    println!("+---------+--------------------------------------+----------+--------+----+--------------+");
    for image in images {
        println!(
            "| {:>7} | {} | {:>8} | {:<6} | {:<2} | {} |",
            image.id, image.uuid, image.display_name, image.format, image.class, image.file_name,
        );
    }
    println!("+---------+--------------------------------------+---------+---------+----+--------------+");
}

fn dump_stacks(stacks: &[Stack]) {
    println!("Stacks");
    println!("+---------+------------+--------+------");
    println!("| id      | collection | pick   | count");
    println!("+---------+------------+--------+------");
    for stack in stacks {
        let count = if let Some(ref content) = stack.content {
            content.len()
        } else {
            0
        };
        println!(
            "| {:>7} | {:>10} | {:>7} | {}",
            stack.id, stack.collection, stack.pick, count
        );
    }
    println!("+---------+------------+--------+------");
}

fn dump_collections(collections: &[Collection]) {
    println!("Collections");
    println!("+---------+------------------------------------------+---------+-------");
    println!("| id      | name                                     | parent  | count");
    println!("+---------+------------------------------------------+---------+-------");
    for collection in collections {
        let count = if let Some(ref content) = collection.content {
            content.len()
        } else {
            0
        };
        println!(
            "| {:>7} | {:<40} | {:>7} | {}",
            collection.id, collection.collection_type, collection.parent, count
        )
    }
    println!("+---------+------------------------------------------+---------+-------");
}

fn process_audit(_: &Args) -> c1::Result<()> {
    Err(c1::Error::Unimplemented)
}

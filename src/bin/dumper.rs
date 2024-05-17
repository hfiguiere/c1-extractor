/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate c1;

use std::collections::BTreeMap;
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
    /// Dump libfiles.
    #[arg(long)]
    libfiles: bool,
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

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Dump(args) => process_dump(&args),
        Command::Audit => process_audit(&args),
    };
}

fn process_dump(args: &DumpArgs) {
    let mut catalog = Catalog::new(&args.path);
    if catalog.open() {
        catalog.load_version();
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
                return;
            }
        }

        {
            let keywordtree = catalog.load_keywords_tree();
            let keywords = catalog.load_keywords();

            if args.all || args.keywords {
                dump_keywords(0, &keywords, &keywordtree);
            }
        }

        {
            let folders = catalog.load_folders();
            if args.all || args.folders {
                dump_folders(&folders);
            }
        }

        {
            let images = catalog.load_images();
            if args.all || args.images {
                dump_images(&images);
            }
        }

        {
            let stacks = catalog.load_stacks();
            if args.all || args.stacks {
                dump_stacks(&stacks);
            }
        }

        {
            let collections = catalog.load_collections();
            if args.all || args.collections {
                dump_collections(&collections);
            }
        }
    }
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
    println!("| id      | collection | pick   |");
    println!("+---------+------------+--------+------");
    for stack in stacks {
        let count = if let Some(ref content) = stack.content {
            content.len()
        } else {
            0
        };
        println!(
            "| {:>7} | {:>7} | {:>7} | {}",
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

fn process_audit(_: &Args) {}

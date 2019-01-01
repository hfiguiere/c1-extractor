/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate c1;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::path::PathBuf;

use c1::{Catalog, CatalogVersion, CoId, Collection, Folder, Image, Keyword, KeywordTree, Stack};
use docopt::Docopt;

const USAGE: &str = "
Usage:
  dumper <command> ([--all] | [--collections] [--libfiles] [--images] [--folders] [--keywords] [--stacks]) <path>

Options:
    --all          Select all objects
    --collections  Select only collections
    --libfiles     Select only library files
    --images       Select only images
    --stacks       Select only stacks
    --folders      Select only folders
    --keywords     Select only keywords

Commands are:
    dump           Dump the objects
    audit          Audit mode: output what we ignored
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Command,
    arg_path: PathBuf,
    flag_all: bool,
    flag_collections: bool,
    flag_libfiles: bool,
    flag_images: bool,
    flag_stacks: bool,
    flag_folders: bool,
    flag_keywords: bool,
}

#[derive(Debug, Deserialize)]
enum Command {
    Dump,
    Audit,
    Unknown(String),
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());
    {
        match args.arg_command {
            Command::Dump => process_dump(&args),
            Command::Audit => process_audit(&args),
            _ => (),
        };
    }
}

fn process_dump(args: &Args) {
    let mut catalog = Catalog::new(&args.arg_path);
    if catalog.open() {
        catalog.load_version();
        println!("Catalog:");
        println!(
            "\tVersion: {} ({:?})",
            catalog.version, catalog.catalog_version
        );
        println!("\tRoot collection id: {}", catalog.root_collection_id);

        match catalog.catalog_version {
            CatalogVersion::Co12 | CatalogVersion::Co11 => {}
            _ => {
                println!("Unsupported catalog version");
                return;
            }
        }

        {
            let keywordtree = catalog.load_keywords_tree();
            let keywords = catalog.load_keywords();

            if args.flag_all || args.flag_keywords {
                dump_keywords(0, &keywords, &keywordtree);
            }
        }

        {
            let folders = catalog.load_folders();
            if args.flag_all || args.flag_folders {
                dump_folders(&folders);
            }
        }

        {
            let images = catalog.load_images();
            if args.flag_all || args.flag_images {
                dump_images(&images);
            }
        }

        {
            let stacks = catalog.load_stacks();
            if args.flag_all || args.flag_stacks {
                dump_stacks(&stacks);
            }
        }

        {
            let collections = catalog.load_collections();
            if args.flag_all || args.flag_collections {
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
        let count = if let Some(ref content) = stack.content { content.len() } else { 0 };
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
        let count = if let Some(ref content) = collection.content { content.len() } else { 0 };
        println!(
            "| {:>7} | {:<40} | {:>7} | {}",
            collection.id, collection.collection_type, collection.parent, count
        )
    }
    println!("+---------+------------------------------------------+---------+-------");
}

fn process_audit(_: &Args) {}

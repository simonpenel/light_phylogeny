// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{
    phyloxml_processing, read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options,
};
// use std::env;

fn main() {
    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.species_internal = true;
    options.uniform = false;
    //options.switches.push("species_3".to_string());
    options
        .hybrid
        .push(("PDECA".to_string(), "PSEXA".to_string()));
    options
        .hybrid
        .push(("PBIAU".to_string(), "PTETRD4".to_string()));

    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "recphylo_examples/FAM000667_reconciliated.recphylo".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    recphyloxml_processing(
        &mut sp_tree,
        &mut gene_trees,
        &mut options,
        &config,
        true,
        &transfers,
        "example_reti.svg".to_string(),
    );
    println!("Please open output file 'example_reti.svg' with your browser");
}

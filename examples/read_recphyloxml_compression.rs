// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{
    phyloxml_processing, read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options,
};
// use std::env;

fn main() {
    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    options.species_internal = true;
    options.species_compression = 0.0;
    read_recphyloxml_multi(
        "examples/FAM000297_reconciliated.recphylo".to_string(),
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
        "read_recphyloxml_FAM000297_compression_0.svg".to_string(),
    );
    println!(
        "Please open output file 'read_recphyloxml_FAM000297_compression_0.svg' with your browser"
    );

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    options.species_internal = true;
    options.species_compression = 1.0;
    read_recphyloxml_multi(
        "examples/FAM000297_reconciliated.recphylo".to_string(),
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
        "read_recphyloxml_FAM000297_compression_1.svg".to_string(),
    );
    println!(
        "Please open output file 'read_recphyloxml_FAM000297_compression_1.svg' with your browser"
    );

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    options.species_internal = true;
    options.species_compression = 2.0;
    read_recphyloxml_multi(
        "examples/FAM000297_reconciliated.recphylo".to_string(),
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
        "read_recphyloxml_FAM000297_compression_2.svg".to_string(),
    );
    println!(
        "Please open output file 'read_recphyloxml_FAM000297_compression_2.svg' with your browser"
    );
}

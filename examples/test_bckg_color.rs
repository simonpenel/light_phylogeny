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

    options.node_colors.push("03_duplication".to_string());
    options.node_colors.push("b4_duplication".to_string());

    read_recphyloxml_multi(
        "examples/example_dupli.recphylo".to_string(),
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
        "test_bckg_default.svg".to_string(),
    );
    println!("Please open output file 'test_bckg_1.svg' with your browser");

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    options.node_colors.push("03_duplication".to_string());
    options.node_colors.push("b4_duplication".to_string());

    options.bckg_color = "Black".to_string();

    read_recphyloxml_multi(
        "examples/example_dupli.recphylo".to_string(),
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
        "test_bckg_black.svg".to_string(),
    );
    println!("Please open output file 'test_bckg_black.svg' with your browser");

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    options.node_colors.push("03_duplication".to_string());
    options.node_colors.push("b4_duplication".to_string());

    options.bckg_color = "#E9C78C".to_string();

    read_recphyloxml_multi(
        "examples/example_dupli.recphylo".to_string(),
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
        "test_bckg_sepia.svg".to_string(),
    );
    println!("Please open output file 'test_bckg_sepia.svg' with your browser");
}

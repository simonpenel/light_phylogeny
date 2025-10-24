// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options};
// use std::env;

fn main() {
    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/FAM000696_reconciliated.recphylo".to_string(),
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
        "read_recphyloxml_portrait.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_portrait.svg' with your browser");

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.mid_dist = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/FAM000696_reconciliated.recphylo".to_string(),
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
        "read_recphyloxml_portrait_mid_dist.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_portrait_mid_dist.svg' with your browser");

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
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
        "read_recphyloxml_portrait_2.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_portrait_2.svg' with your browser");

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.mid_dist = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
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
        "read_recphyloxml_portrait_2_mid_dist.svg".to_string(),
    );
    println!(
        "Please open output file 'read_recphyloxml_portrait_2_mid_dist.svg' with your browser"
    );
}

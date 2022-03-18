// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,phyloxml_processing};
use std::env;

fn main() {

    // // ============================================================================================
    // let transfers = vec![];
    // let mut options: Options = Options::new();
    // options.real_length_flag = true;
    // options.scale = 4.0;
    // options.rotate = false;
    // // options.tidy = true;
    // // options.tidy_leaves_check = true;
    //
    // let config: Config = Config::new();
    // let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    // let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    // let mut global_roots: std::vec::Vec<usize> = Vec::new();
    // read_recphyloxml_multi("recphylo_examples/gene_tree_HBG011000.newick_recs.recphylo".to_string(),
    //     &mut sp_tree, &mut gene_trees, &mut global_roots);
    // recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
    //      &transfers, "read_recphyloxml_tidy.svg".to_string());
    // println!("Please open output file 'read_recphyloxml_tidy.svg' with your browser");
    //

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 1.0;
    // options.rotate = false;
    // options.tidy = true;
    // options.tidy_leaves_check = true;
    options.species_internal = true;

    let mut config: Config = Config::new();
    config.species_police_size = 40.0.to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000297_reconciliated_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "FAM000297_reconciliated_BL.svg".to_string());
    println!("Please open output file 'FAM000297_reconciliated_BL.svg' with your browser");


    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 0.20;
    // options.rotate = false;
    options.tidy = true;
    // options.tidy_leaves_check = true;
    options.species_internal = true;

    let mut config: Config = Config::new();
    config.species_police_size = 40.0.to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000297_reconciliated_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "FAM000297_reconciliated_BL_tidy.svg".to_string());
    println!("Please open output file 'FAM000297_reconciliated_BL_tidy.svg' with your browser");




}

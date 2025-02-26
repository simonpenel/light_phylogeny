// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,phyloxml_processing};
// use std::env;

fn main() {



    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    options.collapsed_nodes.push("species_1".to_string());
    options.collapsed_nodes.push("species_6".to_string());
    options.collapsed_nodes.push("species_15".to_string());
    options.collapsed_nodes.push("species_11".to_string());

    options.species_internal = true;
    read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_FAM000297_reconciliated.svg".to_string());
    println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated.svg' with your browser");


    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    options.collapsed_nodes.push("species_1".to_string());
    options.collapsed_nodes.push("species_6".to_string());
    options.collapsed_nodes.push("species_15".to_string());
    options.collapsed_nodes.push("species_11".to_string());

    options.species_internal = true;
    options.fill_species = true;
    read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_FAM000297_reconciliated_2.svg".to_string());
    println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated_2.svg' with your browser");

    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    options.collapsed_nodes.push("species_1".to_string());
    options.collapsed_nodes.push("species_6".to_string());
    options.collapsed_nodes.push("species_15".to_string());
    options.collapsed_nodes.push("species_11".to_string());

    options.species_internal = true;
    options.species_only_flag = true;
    read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_FAM000297_reconciliated_3.svg".to_string());
    println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated_3.svg' with your browser");


}

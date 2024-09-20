// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};
// use std::env;

fn main() {

    // ============================================================================================
    // Example 1

    // Original tree
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let mut config: Config = Config::new();
    config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    read_recphyloxml_multi("examples/reti2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_1_original.svg".to_string());
    println!("Please open output file 'example_1_original.svg' with your browser");


    // Hybridation 1
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("RETI TEST".to_string(),"RETI TEST 2".to_string()));
    read_recphyloxml_multi("examples/reti2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_1_hybridation_RETI.svg".to_string());
    println!("Please open output file 'example_1_hybridation_RETI.svg' with your browser");



    // ============================================================================================
    // Example 2

    // Original tree
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let mut config: Config = Config::new();
    config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_original.svg".to_string());
    println!("Please open output file 'example_2_original.svg' with your browser");


    // Hybridation 1
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("PJENN".to_string(),"PMUL".to_string()));
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_hybridation_PJENN_PMUL.svg".to_string());
    println!("Please open output file 'example_2_hybridation_PJENN_PMUL.svg' with your browser");


    // Hybridation 2
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("PPENT".to_string(),"PTRED".to_string()));
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_hybridation_PPENT_PTRED.svg".to_string());
    println!("Please open output file 'example_2_hybridation_PPENT_PTRED.svg' with your browser");

    // Hybridation 3
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let  config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("PPENT".to_string(),"PPRIMIR4".to_string()));
    // We need to switch children of species13 to be able to join PPENT and PPRIMIR4
    options.switches.push("species_13".to_string());
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_hybridation_PPENT_PPRIMIR4.svg".to_string());
    println!("Please open output file 'example_2_hybridation_PPENT_PPRIMIR4.svg' with your browser");


    // Hybridation 4
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("species_5".to_string(),"species_6".to_string()));
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_hybridation_species_5_species_6.svg".to_string());
    println!("Please open output file 'example_2_hybridation_species_5_species_6.svg' with your browser");


    // Hybridation 5 (problem)
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let transfers = vec![];
    let config: Config = Config::new();
    //config.fill_species = true;
    let mut options: Options = Options::new();
    options.species_internal = true ;
    // Define hybridation nodes
    options.hybrid.push(("species_12".to_string(),"species_14".to_string()));
    read_recphyloxml_multi("recphylo_examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "example_2_hybridation_species_12_species_14.svg".to_string());
    println!("Please open output file 'example_2_hybridation_species_12_species_14.svg' with your browser");




}

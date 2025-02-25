// Display a reconciled tree form recPhyloXML format
use std::collections::HashMap;
use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;
    options.gene_internal = true;
    options.species_internal = true;

    let mut time_line_1 = HashMap::new();
    time_line_1.insert("PPENT".to_string(),"%circle:green".to_string());
    time_line_1.insert("PQUAD".to_string(),"%square:yellow".to_string());
    time_line_1.insert("PBIAU".to_string(),"%triangle:yellow".to_string());
    time_line_1.insert("PTRED".to_string(),"green".to_string());
    time_line_1.insert("species_23".to_string(),"blue".to_string());
    time_line_1.insert("species_2".to_string(),"green".to_string());
    time_line_1.insert("species_12".to_string(),"purple".to_string());
    time_line_1.insert("species_19".to_string(),"#71cbbe".to_string());
    options.time_lines.push(time_line_1);

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/free_living_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_free_living_reconciliated.svg".to_string());
    println!("Please open output file 'read_recphyloxml_free_living_reconciliated.svg' with your browser");


    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;
    options.gene_internal = true;
    options.species_internal = true;

    let mut time_line_1 = HashMap::new();
    time_line_1.insert("buchnera_rooted.nwk4".to_string(),"blue".to_string());
    time_line_1.insert("buchnera_rooted.nwk7".to_string(),"green".to_string());
    options.time_lines.push(time_line_1);

    let mut time_line_2 = HashMap::new();
    time_line_2.insert("BCUMA".to_string(),"%circle:green".to_string());
    time_line_2.insert("BLAFE".to_string(),"%square:yellow".to_string());
    options.time_lines.push(time_line_2);

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/free_living_reconciliated_2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_free_living_reconciliated_2.svg".to_string());
    println!("Please open output file 'read_recphyloxml_free_living_reconciliated_2.svg' with your browser");


}

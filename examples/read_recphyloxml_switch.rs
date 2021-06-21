// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let  config: Config = Config::new();

    options.gene_internal = true;
    options.species_internal = true;


    let input_file = "examples/FAM000667_reconciliated.recphylo".to_string();
    let input_file2 = input_file.clone();
    // examples/FAM000108_reconciliated.xml
    // examples/gene_parasite_page2.recphylo
    // examples/FAM000667_reconciliated.recphylo
    // examples/FAM000696_reconciliated.recphylo

    // Version standard
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(input_file,&mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_switch_basic.svg".to_string());
    println!("Please open output file 'read_recphyloxml_switch_basic.svg' with your browser");

    // Version optimisation
    options.optimisation = true;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(input_file2, &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_switch_optimised.svg".to_string());
    println!("Please open output file 'read_recphyloxml_switch_optimised.svg' with your browser");


}

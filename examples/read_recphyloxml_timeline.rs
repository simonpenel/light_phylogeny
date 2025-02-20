// Display a reconciled tree form recPhyloXML format
use std::collections::HashMap;

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,phyloxml_processing};
// use std::env;

fn main() {

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let mut time_line_1 = HashMap::new();
    time_line_1.insert("PPENT".to_string(),"red".to_string());
    time_line_1.insert("PQUAD".to_string(),"green".to_string());
    time_line_1.insert("PTRED".to_string(),"green".to_string());
    time_line_1.insert("species_24".to_string(),"black".to_string());
    time_line_1.insert("species_12".to_string(),"pink".to_string());
    let mut time_line_2 = HashMap::new();
    time_line_2.insert("PPENT".to_string(),"pink".to_string());
    time_line_2.insert("PQUAD".to_string(),"yellow".to_string());
    time_line_2.insert("PTRED".to_string(),"orange".to_string());
    time_line_2.insert("species_13".to_string(),"orange".to_string());
    options.time_lines.push(time_line_1);
    options.time_lines.push(time_line_2);

    options.species_internal = true;
    // options.time_lines.insert("PPENT".to_string(),"red".to_string());
    // options.time_lines.insert("PQUAD".to_string(),"green".to_string());
    // options.time_lines.insert("PTRED".to_string(),"green".to_string());
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_timeline.svg".to_string());
    println!("Please open output file 'read_recphyloxml_timeline.svg' with your browser");


}

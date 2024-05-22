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

    
    read_recphyloxml_multi("examples/toyrec.xml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "toy1.svg".to_string());
    println!("Please open output file 'toy1.svg' with your browser");
    
    
    // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
 
  	options.node_colors.push("m3".to_string());   
   	options.node_colors.push("m35".to_string());
    options.node_colors.push("m26".to_string());  	   
    options.gene_colors.push("black".to_string());   
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("blue".to_string());
    options.gene_colors.push("orange".to_string());
    
    read_recphyloxml_multi("examples/toyrec.xml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "toy2.svg".to_string());
    println!("Please open output file 'toy2.svg' with your browser");
    
    
    
    
   // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
 
  	options.node_colors.push("m3".to_string());   
   	options.node_colors.push("m35".to_string());
    options.node_colors.push("m26".to_string());  	   
	options.node_colors.push("bidon".to_string()); 
    
    read_recphyloxml_multi("examples/toyrec.xml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "toy3.svg".to_string());
    println!("Please open output file 'toy3.svg' with your browser");    


   // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
 
    options.gene_colors.push("black".to_string());   
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("blue".to_string());
    options.gene_colors.push("orange".to_string());

    options.node_colors.push("03_duplication".to_string());   
    options.node_colors.push("b4_duplication".to_string());
    
    read_recphyloxml_multi("examples/example_dupli.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "dupli_color.svg".to_string());
    println!("Please open output file 'dupli_color.svg' with your browser"); 


   // ============================================================================================
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
 

    options.node_colors.push("03_duplication".to_string());   
    options.node_colors.push("b4_duplication".to_string());
    
    read_recphyloxml_multi("examples/example_dupli.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "dupli_color2.svg".to_string());
    println!("Please open output file 'dupli_color2.svg' with your browser");        

}


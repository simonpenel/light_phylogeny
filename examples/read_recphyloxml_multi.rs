// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,
    phyloxml_processing,reset_pos};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    options.species_internal = true;
    let config: Config = Config::new();

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/recgs_mult_host_dtl.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    // Version recphylo
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_multi.svg".to_string());
     // Version phylo
    reset_pos(&mut sp_tree);
    phyloxml_processing(&mut sp_tree, &mut options, &config,
        "read_phyloxml_multi.svg".to_string());

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/ex2comp.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    // Version recphylo
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_multi_2.svg".to_string());
         
    reset_pos(&mut sp_tree);
    phyloxml_processing(&mut sp_tree, &mut options, &config,
        "read_phyloxml_multi_2.svg".to_string());

    println!("Please open output files 'read_recphyloxml_multi.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi.svg' with your browser");
    println!("Please open output files 'read_recphyloxml_multi_2.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_2.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_2.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_2.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_3.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_3.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_4.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_4.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_5.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_5.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_6.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_6.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_7.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_7.svg' with your browser");

    println!("Please open output files 'read_recphyloxml_multi_8.svg' with your browser");
    println!("Please open output files 'read_phyloxml_multi_8.svg' with your browser");

}

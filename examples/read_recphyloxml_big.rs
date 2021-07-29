// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    options.height = 2.0;
    options.ratio = 1.5;
    // Version portrait
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/gene_tree_HBG011000.newick_recs.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_portrait.svg".to_string());
    println!("Please open output file 'read_recphyloxml_portrait.svg' with your browser");

    // Version paysage
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/gene_tree_HBG011000.newick_recs.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    options.rotate = false;
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_landscape.svg".to_string());
    println!("Please open output file 'read_recphyloxml_landscape.svg' with your browser");




}

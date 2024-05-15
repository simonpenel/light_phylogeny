// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;
    options.gene_internal = true;
    options.species_internal = true;


    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/free_living_reconciliated_quadruple.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "test_colors1.svg".to_string());
    println!("Please open output file 'test_colors1.svg' with your browser");


    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;
    options.gene_internal = true;
    options.species_internal = true;
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("pink".to_string());
    options.gene_colors.push("#7A5050".to_string());

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/free_living_reconciliated_quadruple.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "test_colors2.svg".to_string());
    println!("Please open output file 'test_colors2.svg' with your browser");
}

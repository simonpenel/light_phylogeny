// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options};

fn main() {
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/free_living_reconciliated_double.recphylo".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    recphyloxml_processing(
        &mut sp_tree,
        &mut gene_trees,
        &mut options,
        &config,
        true,
        &transfers,
        "read_recphyloxml_free_living_reconciliated_double.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_free_living_reconciliated_double.svg' with your browser");

    let mut options: Options = Options::new();
    let config: Config = Config::new();

    options.free_living = true;
    options.free_living_shift = true;

    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/free_living_reconciliated_double.recphylo".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    recphyloxml_processing(
        &mut sp_tree,
        &mut gene_trees,
        &mut options,
        &config,
        true,
        &transfers,
        "read_recphyloxml_free_living_reconciliated_double_shifted.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_free_living_reconciliated_double_shifted.svg' with your browser");
}

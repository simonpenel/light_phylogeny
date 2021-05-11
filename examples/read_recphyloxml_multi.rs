// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    // Version portrait
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/recgs_mult_host_dtl.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_multi.svg".to_string());
    println!("Please open output file 'read_recphyloxml_multi.svg' with your browser");




}

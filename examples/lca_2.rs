use light_phylogeny::{ArenaTree,Options,read_recphyloxml_multi,lca};
use std::env;

fn main() {
    let mut options: Options = Options::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/gene_parasite_page4.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    options.verbose = true;
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let j = sp_tree.get_index("big_host0".to_string()).expect("Error : unable to find big_host0");
    println!("Index of leaf big_host0 is {}",j);
    let l = sp_tree.get_index("p4d".to_string()).expect("Error : unable to find p4d");
    println!("Index of leaf p4d is {}",l);
    let lca_jl = lca(&mut sp_tree,j,l);
    println!("Index of lca  is {} ({})",lca_jl,sp_tree.arena[lca_jl].name);
    assert_eq!(sp_tree.arena[lca_jl].name,"big_host0".to_string());
}

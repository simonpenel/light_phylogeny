// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{
    draw_sptree_gntrees, read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options,
};

fn main() {
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    // Version portrait
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
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
        "read_recphyloxml_original.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_original.svg with your browser");

    let mut sp_tree_clone = sp_tree.copie();
    let mut gene_tree_clones: std::vec::Vec<ArenaTree<String>> = Vec::new();
    for mut tree in gene_trees {
        gene_tree_clones.push(tree.copie());
    }

    draw_sptree_gntrees(
        &mut sp_tree_clone,
        &mut gene_tree_clones,
        "read_recphyloxml_copy.svg".to_string(),
        &options,
        &config,
        &transfers,
    );
    println!("Please open output file 'read_recphyloxml_copy.svg with your browser");
}

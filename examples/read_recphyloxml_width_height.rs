// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,
    phyloxml_processing};

fn main() {

    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    // Version de base
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml.svg".to_string());
    println!("Please open output file 'read_recphyloxml.svg' with your browser");

    // Version height
    options.height = 3.0;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_height.svg".to_string());
    println!("Please open output file 'read_recphyloxml_height.svg' with your browser");

    // Version width
    options.height = 1.0;
    options.width = 3.0;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_width.svg".to_string());
    println!("Please open output file 'read_recphyloxml_width.svg' with your browser");

    // Version width phyloxml
    options.width = 1.0;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    phyloxml_processing(&mut gene_trees[0], &mut options, &config,
         "read_phyloxml.svg".to_string());
    println!("Please open output file 'read_phyloxml.svg' with your browser");

    // Version width phyloxml and height
    options.height = 3.0;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
        let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    phyloxml_processing(&mut gene_trees[0], &mut options, &config,
         "read_phyloxml_height.svg".to_string());
    println!("Please open output file 'read_phyloxml_height.svg' with your browser");

    // Version width phyloxml and with
    options.height = 1.0;
    options.width = 3.0;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    phyloxml_processing(&mut gene_trees[0], &mut options, &config,
         "read_phyloxml_width.svg".to_string());
    println!("Please open output file 'read_phyloxml_width.svg' with your browser");

}

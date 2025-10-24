// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{
    get_gtransfer, read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options,
};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    let transfers = vec![];
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    options.verbose = true;
    env::set_var("RUST_LOG", "info");

    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();

    // Version de base
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );

    //  Recupere les transferts
    let nb_gntree = gene_trees.len().clone();
    let mut gene_transfers = get_gtransfer(&mut gene_trees[0]);
    let mut i = 1;
    while i < nb_gntree {
        let gene_transfer = get_gtransfer(&mut gene_trees[i]);
        for val in gene_transfer {
            gene_transfers.push(val);
        }
        i = i + 1;
    }

    println!("Transfers = {:?}", gene_transfers);

    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut gene_trees,
        &mut options,
        &config,
        true,
        &transfers,
        "read_recphyloxml_threshold_1.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_1.svg' with your browser");

    // Version opacite
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.thickness_flag = true;
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_threshold_2.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_2.svg' with your browser");

    // Version opacite et seuil
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.thickness_flag = true;
    options.thickness_thresh = 0;
    options.thickness_disp_score = true;
    options.rotate = false;
    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_threshold_3.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_3.svg' with your browser");

    // Version opacite et seuil
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.thickness_flag = true;
    options.thickness_thresh = 0;
    options.thickness_disp_score = true;
    options.rotate = false;
    options.trans_end = Some("species_17".to_string());
    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_threshold_4.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_4.svg' with your browser");

    // Version opacite et seuil
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.thickness_flag = true;
    options.thickness_thresh = 0;
    options.thickness_disp_score = true;
    options.rotate = false;
    options.trans_end = Some("species_18".to_string());
    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_threshold_5.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_5.svg' with your browser");

    // Version opacite et seuil
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/concat.xml".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.thickness_flag = true;
    options.thickness_thresh = 0;
    options.thickness_disp_score = true;
    options.rotate = false;
    options.trans_end = None;
    options.thickness_gene = 0;
    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    //selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_threshold_6.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_threshold_6.svg' with your browser");
}

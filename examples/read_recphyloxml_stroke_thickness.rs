// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{
    get_gtransfer, read_recphyloxml_multi, recphyloxml_processing, ArenaTree, Config, Options,
};

fn main() {
    // let transfers = vec![];
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();

    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();

    // Version 1
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
        "read_recphyloxml_stroke_thickness_1.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_stroke_thickness_1.svg' with your browser");

    // Version 2
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

    options.gthickness = 1;

    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_stroke_thickness_2.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_stroke_thickness_2.svg' with your browser");

    // Version 3
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

    options.gthickness = 10;
    options.squaresize = 20.0;

    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_stroke_thickness_3.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_stroke_thickness_3.svg' with your browser");

    // Version 4
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

    options.gthickness = 3;
    options.squaresize = 10.0;
    options.sthickness = 12;
    let mut selected_gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    selected_gene_trees.push(gene_trees.remove(options.thickness_gene));
    recphyloxml_processing(
        &mut sp_tree,
        &mut selected_gene_trees,
        &mut options,
        &config,
        true,
        &gene_transfers,
        "read_recphyloxml_stroke_thickness_4.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_stroke_thickness_4.svg' with your browser");

    // ============================================================================================
    // Version avec losses
    let transfers = vec![];
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        "examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree,
        &mut gene_trees,
        &mut global_roots,
    );
    options.rotate = false;
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    options.gthickness = 3;
    options.squaresize = 20.0;
    options.sthickness = 12;
    recphyloxml_processing(
        &mut sp_tree,
        &mut gene_trees,
        &mut options,
        &config,
        true,
        &transfers,
        "read_recphyloxml_landscape_stroke.svg".to_string(),
    );
    println!("Please open output file 'read_recphyloxml_landscape_stroke.svg' with your browser");
}

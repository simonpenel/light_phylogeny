// Display a reconciled tree form recPhyloXML format

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,phyloxml_processing};
// use std::env;

fn main() {

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_portrait.svg".to_string());
    println!("Please open output file 'read_recphyloxml_portrait.svg' with your browser");

    // ============================================================================================
    // Version paysage
    let transfers = vec![];
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    options.rotate = false;
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_landscape.svg".to_string());
    println!("Please open output file 'read_recphyloxml_landscape.svg' with your browser");


	// ============================================================================================
    // Version phyloxml (1 gene)
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000715_reconciliated_2genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    phyloxml_processing(&mut gene_trees[0], &mut options, &config,"read_recphyloxml_1gene.svg".to_string());
    println!("Please open output file 'read_recphyloxml_1gene.svg' with your browser");


	// ============================================================================================
    // Version phyloxml (1 gene) avec un autre fichier
    let mut options: Options = Options::new();
    let mut config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/gene_parasite.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    phyloxml_processing(&mut gene_trees[0], &mut options, &config,"read_recphyloxml_1gene_bis.svg".to_string());
    println!("Please open output file 'read_recphyloxml_1gene_bis.svg' with your browser");





    // ============================================================================================
    // Version multiple
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/test_5genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_mult.svg".to_string());
    println!("Please open output file 'read_recphyloxml_mult.svg' with your browser");

    // ============================================================================================
    // Version multiple uniforme
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    options.uniform = true;
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/test_5genes.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_mult_uni.svg".to_string());
    println!("Please open output file 'read_recphyloxml_mult_uni.svg' with your browser");

    // ============================================================================================
    // Version pour verifier la gestion des duplication
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_FAM000297_reconciliated.svg".to_string());
    println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated.svg' with your browser");

    // ============================================================================================
    // Version pour verifier la gestion des duplication et option uniforme
    let mut options: Options = Options::new();
    options.uniform = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_FAM000297_reconciliated_uni.svg".to_string());
    println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated_uni.svg' with your browser");


    // ============================================================================================
    // Versions avec longueur de branche

    // Version  simple
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL.svg' with your browser");

    // Version pour  avec l option uniform
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.species_internal = true;
    options.uniform = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_uni.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_uni.svg' with your browser");

    // Version pour  avec l option real length
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    let mut config: Config = Config::new();
    config.species_police_size = "70".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real.svg' with your browser");

    // Version pour  avec l option real length et landcsape
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    options.rotate = false;
    let mut config: Config = Config::new();
    config.species_police_size = "70".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real_landscape.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real_landscape.svg' with your browser");

    // Version pour  avec l option real length et un facteur 5
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 5.0;
    options.species_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real_5.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real_5.svg' with your browser");


    // ============================================================================================
    // Versions avec longueur de branche 2

    // Version pour  avec l option uniform
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.uniform = true;
    options.species_internal = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_uni_2.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_uni_2.svg' with your browser");

    // Version pour  avec l option real length
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real_2.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real_2.svg' with your browser");


    // Version pour verifier les font
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    let mut config: Config = Config::new();
    config.species_police_size = 60.to_string();
    config.gene_police_size = 5.to_string();
    config.species_police_color = "green".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real_fonts.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real_fonts.svg' with your browser");

    // Version ladnscape
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    options.rotate = false;
    let mut config: Config = Config::new();
    config.species_police_size = 60.to_string();
    config.gene_police_size = 5.to_string();
    config.species_police_color = "green".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_2.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_real_fonts_landcsape.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_real_fonts_landcsape.svg' with your browser");

    // Version tidy
    // env::set_var("RUST_LOG", "info");
    env_logger::init();
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    options.rotate = false;
    options.tidy = true;
    let mut config: Config = Config::new();
    config.species_police_size = 60.to_string();
    config.gene_police_size = 5.to_string();
    config.species_police_color = "green".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_3.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_3_real_fonts_landcsape_tidy.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_3_real_fonts_landcsape_tidy.svg' with your browser");

    // Version nono tidy
    // env::set_var("RUST_LOG", "warn");
    let transfers = vec![];
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.species_internal = true;
    options.branch = true;
    options.rotate = false;
    let mut config: Config = Config::new();
    config.species_police_size = 60.to_string();
    config.gene_police_size = 5.to_string();
    config.species_police_color = "green".to_string();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi("examples/hote_parasite_page4_BL_3.recphylo".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "hote_parasite_page4_BL_3_real_fonts_landcsape.svg".to_string());
    println!("Please open output file 'hote_parasite_page4_BL_3_real_fonts_landcsape.svg' with your browser");




    // ============================================================================================
    // Version pour  real_branch avec branche nulles (-> erreur)
    // let transfers = vec![];
    // let mut options: Options = Options::new();
    // options.real_length_flag = true;
    // let config: Config = Config::new();
    // let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    // let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    // let mut global_roots: std::vec::Vec<usize> = Vec::new();
    // read_recphyloxml_multi("examples/FAM000297_reconciliated.recphylo".to_string(),
    //     &mut sp_tree, &mut gene_trees, &mut global_roots);
    // recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
    //      &transfers, "read_recphyloxml_FAM000297_reconciliated_real.svg".to_string());
    // println!("Please open output file 'read_recphyloxml_FAM000297_reconciliated_real.svg' with your browser");



}

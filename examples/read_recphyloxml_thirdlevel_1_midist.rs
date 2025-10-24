// Visualisation  of  3 levels reconciliation
//  Case there are several gene trees

use light_phylogeny::{
    get_gtransfer, map_gene_host, map_parasite_g2s, map_parasite_s2g, map_transfer,
    map_transfer_mul, phyloxml_processing, read_recphyloxml_multi, recphyloxml_processing,
    reset_pos, ArenaTree, Config, Options,
};
use log::info;

fn main() {
    let mut options: Options = Options::new();
    options.gene_internal = true;
    options.species_internal = true;
    options.mid_dist = true;
    let mut config: Config = Config::new();
    config.species_opacity = "0.7".to_string();
    config.gene_opacity = "0.9".to_string();
    let transfers = vec![];
    // let infile_gs = "examples/recgs_mult_host_dtl.recphyloxml".to_string();
    let infile_gs = "examples/test1/recgs_dtl.recphyloxml".to_string();
    let infile_sh = "examples/test1/rechp_dtl.recphyloxml".to_string();
    // Traitement de 2 fichier fichiers recPhyloXML
    println!("Two reconciled files => displaying 3-levels reconciliations. ");
    let outfile_gene_para = String::from("gene_para.svg");
    let outfile_para_host = String::from("para_host.svg");
    let outfile_mapped_1 = String::from("mapped_1.svg");
    let outfile_mapped_2 = String::from("mapped_2.svg");

    // ---------------------------------------------------------
    // Create a structure Arena for the global parasite pipe
    // tree and a vector of structures Arena for gene path trees
    // ---------------------------------------------------------
    let mut global_pipe_parasite: ArenaTree<String> = ArenaTree::default();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    let mut path_genes: std::vec::Vec<ArenaTree<String>> = Vec::new();
    // ---------------------------------------------------------
    // Fill global parasite pipe tree and is roots and path
    // genes trees
    // ---------------------------------------------------------
    read_recphyloxml_multi(
        infile_gs,
        &mut global_pipe_parasite,
        &mut path_genes,
        &mut global_roots,
    );
    let nb_gntree = path_genes.len().clone();
    println!("Number of gene trees : {}", nb_gntree);
    info!("List of gene trees : {:?}", path_genes);
    let nb_parasite_pipe = global_roots.len().clone();
    println!("Number of parasite trees : {}", nb_parasite_pipe);
    println!("List of species trees roots : {:?}", global_roots);
    info!("Global parasite pipe tree : {:?}", global_pipe_parasite);
    // ---------------------------------------------------------
    // Generate svg of the lobal parasite pipe tree and  path
    // genes trees
    // ---------------------------------------------------------
    recphyloxml_processing(
        &mut global_pipe_parasite,
        &mut path_genes,
        &mut options,
        &config,
        true,
        &transfers,
        outfile_gene_para,
    );
    // ---------------------------------------------------------
    // Create a structure Arena for the host pipe tree and a
    // vector of structures Arena for parasite path trees
    // ---------------------------------------------------------
    let mut tree_host_pipe: ArenaTree<String> = ArenaTree::default();
    let mut path_para_trees: std::vec::Vec<ArenaTree<String>> = Vec::new();
    // ---------------------------------------------------------
    // Fill  host pipe tree and is roots and path parasite trees
    // ---------------------------------------------------------
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(
        infile_sh,
        &mut tree_host_pipe,
        &mut path_para_trees,
        &mut global_roots,
    );
    let nb_parasite_path = path_para_trees.len().clone();
    println!(
        "Number of pipe parasite trees in gene-parasite file : {}",
        nb_parasite_pipe
    );
    println!(
        "Number of path parasite trees in parasite-host file : {}",
        nb_parasite_path
    );
    if nb_parasite_path != nb_parasite_pipe {
        println!();
        println!("==============================================");
        println!("Error! Different number of parasite trees in the 2 files!");
        println!("       Resulting svg will be incomplete.");
        println!("==============================================");
        println!();
    }
    // ---------------------------------------------------------
    // Generate svg of the host pipe tree and path parasite trees
    // ---------------------------------------------------------
    recphyloxml_processing(
        &mut tree_host_pipe,
        &mut path_para_trees,
        &mut options,
        &config,
        true,
        &transfers,
        outfile_para_host,
    );
    // ---------------------------------------------------------
    // Generation of first 3 levels svg
    // ---------------------------------------------------------
    info!("Parasite trees as a 'path tree' : {:?}", path_para_trees);
    info!(
        "Parasite tree as a 'pipe tree' : {:?}",
        global_pipe_parasite
    );
    println!("==============================================");
    println!("Map parasite as 'path' to parasite as 'pipe'");
    println!("==============================================");
    let mut i = 0;
    while i < nb_parasite_pipe {
        map_parasite_g2s(&mut global_pipe_parasite, &mut path_para_trees[i]);
        i = i + 1;
    }
    info!(
        "Global parasite tree wih events : {:?}",
        global_pipe_parasite
    );
    reset_pos(&mut global_pipe_parasite);
    let mut i = 0;
    while i < nb_gntree {
        reset_pos(&mut path_genes[i]);
        i = i + 1;
    }
    println!("==============================================");
    println!("Map parasite as 'species' to parasite as 'gene'");
    println!("==============================================");
    let mut i = 0;
    while i < nb_parasite_pipe {
        map_parasite_s2g(
            &mut global_pipe_parasite,
            &mut path_para_trees[i],
            &mut path_genes,
        );
        i = i + 1;
    }
    info!(
        "Global pipe parasite after mapping s2g : {:?}",
        global_pipe_parasite
    );
    println!("==============================================");
    println!("Map parasite as 'gene' to parasite as 'species' again");
    println!("==============================================");
    let mut i = 0;
    while i < nb_parasite_pipe {
        map_parasite_g2s(&mut global_pipe_parasite, &mut path_para_trees[i]);
        i = i + 1;
    }
    reset_pos(&mut global_pipe_parasite);
    let mut i = 0;
    while i < nb_gntree {
        reset_pos(&mut path_genes[i]);
        i = i + 1;
    }
    // attention on ne remape pas
    recphyloxml_processing(
        &mut global_pipe_parasite,
        &mut path_genes,
        &mut options,
        &config,
        false,
        &transfers,
        outfile_mapped_1,
    );
    // ---------------------------------------------------------
    // Generation of second 3 levels svg
    // ---------------------------------------------------------
    let mut i = 0;
    let gene_transfers = get_gtransfer(&mut path_genes[i]);
    info!("Transfers = {:?}", gene_transfers);
    let mut mapped_gene_transfers = map_transfer_mul(gene_transfers, &mut path_para_trees);
    info!("Mapped transfers = {:?}", mapped_gene_transfers);
    i = i + 1;
    while i < nb_gntree {
        let gene_transfers = get_gtransfer(&mut path_genes[i]);
        info!("Transfers = {:?}", gene_transfers);
        let mapped = map_transfer(gene_transfers, &mut path_para_trees[0]);
        info!("Mapped transfers = {:?}", mapped);
        for val in mapped {
            mapped_gene_transfers.push(val);
        }
        i = i + 1;
    }
    reset_pos(&mut tree_host_pipe);
    let mut i = 0;
    while i < nb_parasite_pipe {
        reset_pos(&mut path_para_trees[i]);
        i = i + 1;
    }
    println!(
        "Building svg 2:  parasite tree within host pipe tree and mapped tarnsfers {}",
        outfile_mapped_2
    );
    // attention on ne remape pas
    recphyloxml_processing(
        &mut tree_host_pipe,
        &mut path_para_trees,
        &mut options,
        &config,
        false,
        &mapped_gene_transfers,
        outfile_mapped_2,
    );

    reset_pos(&mut global_pipe_parasite);
    phyloxml_processing(
        &mut global_pipe_parasite,
        &mut options,
        &config,
        "para_simple.svg".to_string(),
    );
    reset_pos(&mut tree_host_pipe);
    phyloxml_processing(
        &mut tree_host_pipe,
        &mut options,
        &config,
        "host_simple.svg".to_string(),
    );
    let mut i = 0;
    while i < nb_parasite_pipe {
        reset_pos(&mut path_para_trees[i]);
        phyloxml_processing(
            &mut path_para_trees[i],
            &mut options,
            &config,
            ("gene_simple_".to_owned() + &i.to_string() + ".svg").to_string(),
        );
        i = i + 1;
    }

    //llmlmlmlmlmlm
    // mapping des gene sur les hotes via les parasites
    println!("Building svg 3: gene  tree within host pipe tree");
    map_gene_host(&mut path_genes, &mut path_para_trees, &mut tree_host_pipe);

    // map_gene_host(&mut path_genes, &mut path_para_trees[1], &mut tree_host_pipe);
    reset_pos(&mut tree_host_pipe);
    let mut i = 0;
    while i < nb_gntree {
        reset_pos(&mut path_genes[i]);
        i = i + 1;
    }

    recphyloxml_processing(
        &mut tree_host_pipe,
        &mut path_genes,
        &mut options,
        &config,
        true,
        &vec![],
        "mapped_3.svg".to_string(),
    );
    // println!("DEBUG  FINAL HOST = {:?}",tree_host_pipe);
    // lmlmlmlm

    println!("Output files:");

    println!(" - host_simple.svg ...... 1 level: host tree");
    let mut i = 0;
    while i < nb_parasite_pipe {
        println!(" - para_simple.svg ...... 2 levels: gene_simple_{}.svg", &i);
        i = i + 1;
    }
    println!(" - para_simple.svg ...... 2 levels: parasite tree(s)");
    println!(" - gene_para.svg ........ 2 levels: pipe parasite tree(s) with gene tree(s) inside");
    println!(" - para_host.svg ........ 2 levels: pipe host tree with parasite tree(s) inside");
    println!(
        " - mapped_1.svg ........  3 levels: reconciled pipe parasite tree(s) with gene tree(s)"
    );
    println!(
        " - mapped_2.svg ........  3 levels: parasite-host reconciliation plus gene transfers"
    );
    println!(" - mapped_3.svg ........  3 levels: pipe host tree with gene tree(s) inside");

    if nb_parasite_path != nb_parasite_pipe {
        println!();
        println!("==============================================");
        println!("Error! Different number of parasite trees in the 2 files!");
        println!("       Resulting svg will be incomplete.");
        println!("==============================================");
        println!();
    }
}

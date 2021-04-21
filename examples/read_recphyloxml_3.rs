// Visualisation  of  3 levels reconciliation
//  Case there are several gene trees

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml,recphyloxml_processing,
phyloxml_processing,reset_pos,map_transfer,get_gtransfer,summary,
map_parasite_g2s,map_parasite_s2g};
use log::{info};

fn main() {


    let mut options: Options = Options::new();
    let mut config: Config = Config::new();    
    config.species_opacity = "0.3".to_string();
    config.gene_opacity = "0.6".to_string();
    let transfers = vec![];
    let infile_gene_para = "examples/recgs_dtl.recphyloxml".to_string();
    let infile_para_host = "examples/rechp_dtl.recphyloxml".to_string();

    // let infile_gene_para = "examples/gene_parasite_page4.recphylo".to_string();
    // let infile_para_host = "examples/hote_parasite_page4.recphylo".to_string();


    let outfile_gene_para = String::from("gene_para.svg");
    let outfile_para = String::from("para.svg");
    let outfile_para_rec = String::from("para_rec.svg");
    let outfile_para_host = String::from("para_host.svg");
    let outfile_host = String::from("host.svg");

    // On cree une structure Arena pour l'arbre d'espece
    // et un vecteur de  structures Arena pour le(s) arbres de gènes
    // -------------------------------------------------------------
    // Creation de la structure ArenaTree pour l'arbre d'espece
    // --------------------------------------------------------
    let mut tree_para_pipe: ArenaTree<String> = ArenaTree::default();
    // Creation du vecteur de structure ArenaTree pour les genes
    // ---------------------------------------------------------
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    read_recphyloxml(infile_gene_para,&mut tree_para_pipe,&mut gene_trees);
    let  nb_gntree =  gene_trees.len().clone();
    println!("Number of gene trees : {}",nb_gntree);
    info!("List of gene trees : {:?}",gene_trees);
    recphyloxml_processing(&mut tree_para_pipe,&mut  gene_trees, &mut options, &config,true,
        &transfers,outfile_gene_para);
    reset_pos(&mut tree_para_pipe);
    phyloxml_processing(&mut tree_para_pipe, &mut options, &config,outfile_para);

    // On cree une structure Arena pour l'arbre d'espece
    // et un vecteur de  structures Arena pour le(s) arbres de gènes
    // -------------------------------------------------------------
    // Creation de la structure ArenaTree pour l'arbre d'espece
    // --------------------------------------------------------
    let mut tree_host_pipe: ArenaTree<String> = ArenaTree::default();
    // Creation du vecteur de structure ArenaTree pour les genes
    // ---------------------------------------------------------
    let mut para_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    read_recphyloxml(infile_para_host,&mut tree_host_pipe,&mut para_trees);
    let  nb_paratree =  para_trees.len().clone();
    assert_eq!(nb_paratree,1,"I want only one parasite tree");
    println!("Number of gene trees : {}",nb_paratree);
    info!("List of gene trees : {:?}",para_trees);
    recphyloxml_processing(&mut tree_host_pipe,&mut  para_trees, &mut options, &config,
        true, &transfers,outfile_para_host);
    reset_pos(&mut tree_host_pipe);
    phyloxml_processing(&mut tree_host_pipe, &mut options, &config,outfile_host);
    reset_pos(&mut para_trees[0]);
    phyloxml_processing(&mut para_trees[0], &mut options, &config,outfile_para_rec);

    // Generation du svg 3 niveaux
    println!("Parasite trees as a 'gene tree' : {:?}",para_trees);
    println!("Parasite tree as a 'species tree' : {:?}",tree_para_pipe);
    println!("Map parasite as 'gene' to parasite as 'species'");
    println!("==============================================");
    map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
    println!("Map parasite as 'species' to paraiste as 'gene'");
    println!("==============================================");
    map_parasite_s2g(&mut tree_para_pipe, &mut para_trees[0], &mut gene_trees);
    println!("MAP AGAIN!");
    println!("MON GROS DEBUG PARAPIPE1{:?}",&mut tree_para_pipe);
    println!("Map parasite as 'gene' to parasite as 'species'");
    println!("==============================================");
    map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
    summary(&mut tree_para_pipe);


    reset_pos(&mut tree_para_pipe);
    let mut i = 0;
    while i < nb_gntree {
    reset_pos(&mut gene_trees[i]);
    i = i + 1;
    }
    // reset_pos(&mut gene_trees[0]);
    // reset_pos(&mut gene_trees[1]);
    // reset_pos(&mut gene_trees[2]);
    // reset_pos(&mut gene_trees[2]);
    // options.species_internal = true;
    // options.gene_internal = true;
//     options.verbose = true;
//     env::set_var("RUST_LOG", "info");
// env_logger::init();
// info!("Verbosity set to Info");

    // attention on ne remape pas
    recphyloxml_processing(&mut tree_para_pipe, &mut gene_trees, &mut options, &config, false,
        &transfers,"test_mapped.svg".to_string());

    // Generation des svg hote parsite  +transfert gene

    let gene_transfers = get_gtransfer(&mut gene_trees[0]);
    println!("Transfers = {:?}",gene_transfers);
    let mut mapped_gene_transfers = map_transfer(gene_transfers, &mut para_trees[0]);
    println!("Mapped transfers = {:?}",mapped_gene_transfers);
    let mut i = 1;
    while i < nb_gntree {
        let gene_transfers = get_gtransfer(&mut gene_trees[i]);
        println!("Transfers = {:?}",gene_transfers);
        let mapped = map_transfer(gene_transfers, &mut para_trees[0]);
        println!("Mapped transfers = {:?}",mapped);
        for val in mapped {
            mapped_gene_transfers.push(val);
        }
        i = i + 1;
    }
    reset_pos(&mut tree_host_pipe);
    reset_pos(&mut para_trees[0]);
    // attention on ne remape pas
    recphyloxml_processing(&mut tree_host_pipe, &mut para_trees, &mut options, &config,
        false, &mapped_gene_transfers,"test_mapped_2levels.svg".to_string());
}

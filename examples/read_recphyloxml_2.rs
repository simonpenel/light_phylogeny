// Visualisation  of  3 levels reconciliation
//  Case there is only 1 gene tree
use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing,
phyloxml_processing,reset_pos,map_transfer,get_gtransfer,summary,
map_parasite_g2s,map_parasite_s2g};
use log::{info};

fn main() {


    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let transfers = vec![];
    let infile_gene_para = "examples/gene_parasite_page2.recphylo".to_string();
    let infile_para_host = "examples/hote_parasite_page2.recphylo".to_string();

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
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(infile_gene_para,&mut tree_para_pipe,&mut gene_trees, &mut global_roots);
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
    let mut global_roots: std::vec::Vec<usize> = Vec::new();
    read_recphyloxml_multi(infile_para_host,&mut tree_host_pipe,&mut para_trees, &mut  global_roots);
    let  nb_paratree =  para_trees.len().clone();
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
    println!("Mapping again!");
    println!("Map parasite as 'gene' to parasite as 'species'");
    println!("==============================================");
    map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
    summary(&mut tree_para_pipe);

    reset_pos(&mut tree_para_pipe);
    reset_pos(&mut gene_trees[0]);
    recphyloxml_processing(&mut tree_para_pipe, &mut gene_trees, &mut options, &config, false,
        &transfers,"test_mapped.svg".to_string());

    // Generation des svg hote parsite  +transfert gene

    let gene_transfers = get_gtransfer(&mut gene_trees[0]);
    println!("Transfers = {:?}",gene_transfers);
    let mapped_gene_transfers = map_transfer(gene_transfers, &mut para_trees[0]);
    println!("Mapped transfers = {:?}",mapped_gene_transfers);
    reset_pos(&mut tree_host_pipe);
    reset_pos(&mut para_trees[0]);
    recphyloxml_processing(&mut tree_host_pipe, &mut para_trees, &mut options, &config,
        false, &mapped_gene_transfers,"test_mapped_2levels.svg".to_string());



}

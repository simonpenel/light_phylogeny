/// name = "light_phylogeny"
/// version = "0.10.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"
use log::{info};
use std::fs;
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::Config;
use crate::arena::{newick2tree,xml2tree};
use crate::arena::{knuth_layout,cladogramme,check_contour_postorder,shift_mod_xy,
    set_middle_postorder,real_length};
use crate::arena::{map_species_trees,set_species_width,check_vertical_contour_postorder,
    map_gene_trees,bilan_mappings,center_gene_nodes,move_dupli_mappings};
use crate::arena::{find_sptree,find_sptrees,find_rgtrees,check_for_obsolete,scale_heigth,scale_width};
use crate::thirdlevel::{get_gtransfer,optimisation,check_optimisation,classify_transfer,reorder_transfers};
use crate::drawing::{draw_tree,draw_sptree_gntrees};

/// Read a newick file and store the tree into ArenaTree structure
//  ===============================================================
pub fn read_newick(filename:String, tree: &mut ArenaTree<String>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error! Something went wrong reading the newick file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let root = tree.new_node("Root".to_string());
    newick2tree(contents, tree, root, &mut 0);
}

/// Read a phyloxml file and store the tree into a ArenaTree structure
//  ===============================================================
pub fn read_phyloxml(filename:String, tree: &mut ArenaTree<String>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error! Something went wrong reading the phyloxml file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let doc = roxmltree::Document::parse(&contents).unwrap();
    let descendants = doc.root().descendants();
    // Search for the first occurence of the "clade" tag
    for node in descendants {
        if node.has_tag_name("clade"){
            // The first clade is the root
            // Initialize the index used for defining the value
            let mut index  = &mut 0;
            // Val of the root
            let root = "N".to_owned()+&index.to_string();
            // Create the node, get its associated index and store it in root again
            let root = tree.new_node(root.to_string());
            // Call xlm2tree on the root
            xml2tree(node, root, &mut index,  tree);
            break;
        }
    }
    info!("Tree : {:?}",tree);
}
/// Read a recphyloxml file and store the species and gene trees into several ArenaTree structures
//  ==============================================================================================
pub fn read_recphyloxml(filename:String, sp_tree: &mut ArenaTree<String>,
    gene_trees: &mut std::vec::Vec<ArenaTree<String>>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error! Something went wrong reading the recPhyloXML file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let doc = &mut roxmltree::Document::parse(&contents).unwrap();
    // Get the  species tree:
    // Get the  NodeId associated to the first element with the "spTree" tag
    let spnode = find_sptree(doc);
    let spnode = match spnode {
        Ok(index) => index,
        Err(_err) => {
            eprintln!("\nError: No clade spTree has been found in xml.\
            \nIt seems that the input file is not a recPhyloXML file.\
            \nUse option -F to force to use phyloXML or newick format.");
             std::process::exit(1);
        },
    };
    // Get the  Node associated  to the NodeId
    let spnode = doc.get_node(spnode).expect("Unable to get the Node associated to this nodeId");
    info!("spTree Id: {:?}",spnode);
    let descendants = spnode.descendants();
    // Search for the first occurence of the "clade" tag
    for node in descendants {
        if node.has_tag_name("clade"){
            // The first clade is the root
            // Initialize the index used for defining the value
            let mut index  = &mut 0;
            // Val of the root
            let root = "N".to_owned()+&index.to_string();
            // Create the node, get its associated index and store it in root again
            let root = sp_tree.new_node(root.to_string());
            // Call xlm2tree on the root
            xml2tree(node, root, &mut index,  sp_tree);
            break;
        }
    }
    info!("Species tree : {:?}",sp_tree);
    // Get the gene trees:
    // Get the list of nodes associated to  the "recGeneTree" tag
     let rgnodes = find_rgtrees(doc);
     let rgnodes = match rgnodes {
         Ok(indexes) => indexes,
         Err(_err) => {
             eprintln!("\nError: No clade recGeneTree has been found in xml.\
             \nIt seems that the input file is not a recPhyloXML file.\
             \nUse option -F to force to use phyloXML or newick format.");
              std::process::exit(1);
         },
     };
    for rgnode in rgnodes {
        let mut gene_tree: ArenaTree<String> = ArenaTree::default();
        info!("Search recGeneTree node {:?}",rgnode);
        let rgnode = doc.get_node(rgnode).expect("Unable to get the Node associated to this nodeId");
        info!("Associated recGeneTree  : {:?}",rgnode);
        // Analyse the gene tree
        let descendants = rgnode.descendants();
        // Search for the first occurence of the "clade" tag
        for node in descendants {
            if node.has_tag_name("clade"){
                // The first clade is the root
                // Initialize the index used for defining the value
                let mut index  = &mut 0;
                // Val of the root
                let root = "N".to_owned()+&index.to_string();
                // Create the node, get its associated index and store it in root again
                let root = gene_tree.new_node(root.to_string());
                // Call xlm2tree on the root
                xml2tree(node, root, &mut index, &mut gene_tree);
                break;
            }
        }
        // Traitement des balises obsoletes potentielles (ancien format recPhyloXML)
        check_for_obsolete(&mut gene_tree,sp_tree);
        // Ajoute l'arbre de gene
        gene_trees.push(gene_tree);
    }
    let  nb_gntree =  gene_trees.len().clone();
    println!("Number of gene trees : {}",nb_gntree);
    info!("List of gene trees : {:?}",gene_trees);
}

/// Read a recphyloxml file and store the species and gene trees into several ArenaTree structures
//  ==============================================================================================
pub fn read_recphyloxml_multi(filename:String, global_pipe: &mut ArenaTree<String>,
    gene_trees: &mut std::vec::Vec<ArenaTree<String>>,
    global_roots:  &mut std::vec::Vec<usize>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error! Something went wrong reading the recPhyloXML file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let doc = &mut roxmltree::Document::parse(&contents).unwrap();
    // Get the species trees:
    // Get the list of nodes associated to  the "spTree" tag
    // let spnodes = find_sptrees(doc).expect("No clade spTree has been found in xml");
    let spnodes = find_sptrees(doc);
    let spnodes = match spnodes {
        Ok(indexes) => indexes,
        Err(_err) => {
            eprintln!("\nError: No clade spTree has been found in xml.\
            \nIt seems that the input file is not a recPhyloXML file.\
            \nUse option -F to force to use phyloXML or newick format.");
             std::process::exit(1);
        },
    };

    let  mut index  =  &mut 0;
    // let mut groots: std::vec::Vec<usize> = Vec::new();

    for spnode in spnodes {
        info!("Search spTree node {:?}",spnode);
        let spnode = doc.get_node(spnode).expect("Unable to get the Node associated to this nodeId");
        info!("Associated spTree  : {:?}",spnode);
        // Analyse the gene tree
        let descendants = spnode.descendants();
        // Search for the first occurence of the "clade" tag
        // index = &(index + 1) ;
        *index +=1;
        for node in descendants {
            if node.has_tag_name("clade"){
                let globalroot = "G".to_owned()+&index.to_string();
                // Create the node, get its associated index and store it in root again
                println!("Create {}","G".to_owned()+&index.to_string());
                let globalroot =global_pipe.new_node(globalroot.to_string());
                global_roots.push(globalroot);
                // Call xlm2tree on the root
                xml2tree(node, globalroot, &mut index, global_pipe);
                break;
            }
        }
    }
    if global_roots.len() > 1 {
    println!("Racine gauche = {}",global_roots[0]);
    println!("Racine droite = {}",global_roots[1]);

    let super_root = global_pipe.new_node("SUPER_ROOT".to_string());
    global_pipe.arena[super_root].name="SUPER ROOT".to_string();
    global_pipe.arena[super_root].visible = false;
    global_pipe.arena[super_root].children.push(global_roots[0]);
    global_pipe.arena[super_root].children.push(global_roots[1]);
    global_pipe.arena[global_roots[0]].parent = Some(super_root);
    global_pipe.arena[global_roots[1]].parent = Some(super_root);
    }

    let  nb_sptree =  global_roots.len().clone();
    println!("Number of species trees : {}",nb_sptree);
    info!("List of species roots : {:?}",global_roots);
    // Get the gene trees:
    // Get the list of nodes associated to  the "recGeneTree" tag
    // let rgnodes = find_rgtrees(doc).expect("No clade recGeneTree has been found in xml");
    let rgnodes = find_rgtrees(doc);
    let rgnodes = match rgnodes {
        Ok(indexes) => indexes,
        Err(_err) => {
            eprintln!("\nError: No clade recGeneTree has been found in xml.\
            \nIt seems that the input file is not a recPhyloXML file.\
            \nUse option -F to force to use phyloXML or newick format.");
             std::process::exit(1);
        },
    };
    for rgnode in rgnodes {
        let mut gene_tree: ArenaTree<String> = ArenaTree::default();
        info!("Search recGeneTree node {:?}",rgnode);
        let rgnode = doc.get_node(rgnode).expect("Unable to get the Node associated to this nodeId");


        info!("Associated recGeneTree  : {:?}",rgnode);
        // Analyse the gene tree
        let descendants = rgnode.descendants();
        // Search for the first occurence of the "clade" tag
        for node in descendants {
            if node.has_tag_name("clade"){
                // The first clade is the root
                // Initialize the index used for defining the value
                let mut index  = &mut 0;
                // Val of the root
                let root = "N".to_owned()+&index.to_string();
                // Create the node, get its associated index and store it in root again
                let root = gene_tree.new_node(root.to_string());
                // Call xlm2tree on the root
                xml2tree(node, root, &mut index, &mut gene_tree);
                break;
            }
        }
        // Traitement des balises obsoletes potentielles (ancien format recPhyloXML)
        check_for_obsolete(&mut gene_tree, global_pipe);
        // Ajoute l'arbre de gene
        gene_trees.push(gene_tree);
    }
    let  nb_gntree =  gene_trees.len().clone();
    println!("Number of gene trees : {}",nb_gntree);
    info!("List of gene trees : {:?}",gene_trees);
}

/// Create a svg of the tree in phyloxml context
// =============================================
pub fn phyloxml_processing(
    mut tree: &mut ArenaTree<String>, // tree
    options: &Options,                // display options
    config: &Config,                  // svg configuration
    outfile: String                   // output file
    ) {
    info!("Tree : {:?}",tree);
    // -----------------------
    // Traitement en 4 étapes
    // -----------------------
    // Au départ l'arbre est orienté du haut vers le bas (i.e. selon Y)
    // Le svg sera tourné de -90 a la fin.
    //
    //----------------------------------------------------------
    // 1ère étape :initialisation des x,y de l'arbre :
    // profondeur => Y, left => X= 0, right X=1
    // ---------------------------------------------------------
    let  root = tree.get_root();
    knuth_layout(&mut tree,root, &mut 1);
    // ---------------------------------------------------------
    // Option : Cladogramme
    // ---------------------------------------------------------
    if options.clado_flag {
        cladogramme(&mut tree);
    }
    // ---------------------------------------------------------
    // 2ème étape : Vérifie les contours
    // ---------------------------------------------------------
     check_contour_postorder(&mut tree, root);
    // ---------------------------------------------------------
    // 3eme etape : Decale toutes les valeurs de x en fonction
    // de xmod
    // ---------------------------------------------------------
    shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
    // ---------------------------------------------------------
    // 4ème étape : Place le parent entre les enfants
    // ---------------------------------------------------------
    set_middle_postorder(&mut tree, root);
    // ---------------------------------------------------------
    // OPTIONAL Scale the heigt if needed
    // ---------------------------------------------------------
    if options.height != 1.0 { scale_heigth(&mut tree,options.height)};
    // ---------------------------------------------------------
    // OPTIONAL Scale the width if needed
    // ---------------------------------------------------------
    if options.width != 1.0 { scale_width(&mut tree,options.width)};
    // ---------------------------------------------------------
    // Option : real_length
    // ---------------------------------------------------------
    if options.real_length_flag {
        real_length(&mut tree, root, &mut 0.0, & options);
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);
    draw_tree(&mut tree, outfile, & options, & config);
}

/// Create a svg of the tree in recphyloxml context
// =============================================
pub fn recphyloxml_processing(
    mut sp_tree: &mut ArenaTree<String>,                    // species tree
    mut gene_trees: &mut std::vec::Vec<ArenaTree<String>>,  // gene trees
    mut options: &mut Options,                              // display options
    config: &Config,                                        // svg configuration
    mapping:bool,                                           // map gene and species
    transfers: & std::vec::Vec<(String,String)>,            // optional additional transfers
    outfile: String                                         // output file
) {
// -----------------------
// Traitement en 12 etapes
// -----------------------
// Au depart l'arbre est orienté du haut vers le bas (i.e. selon Y)
// Le svg sera tourné de -90 a la fin.
//
//----------------------------------------------------------
// 1ere étape :initialisation des x,y de l'arbre d'espèces :
// profondeur => Y, left => X= 0, right X=1
// ---------------------------------------------------------
let  root = sp_tree.get_root();
knuth_layout(&mut sp_tree,root, &mut 1);
// --------------------
// OPTIONAL  Cladogramme
// --------------------
if options.clado_flag {
    cladogramme(&mut sp_tree);
}
// ---------------------------------------------------------
// 2eme étape :  mapping des genes sur l'espèce pour
// connaître le nombre de noeuds d'arbre de gènes associés à
// chaque noeud de l'arbre d'espèces
// ---------------------------------------------------------
if mapping {
    map_species_trees(&mut sp_tree,&mut gene_trees);
    info!("Species tree after mapping : {:?}",sp_tree);
}
// ---------------------------------------------------------
// OPTIONAL Optimisation if needed
// ---------------------------------------------------------
if options.optimisation {
    println!("Optimisation of orientation according to transfers");
    if gene_trees.len() > 1 {
        eprintln!("Optimisation is working only with single gene tree reconciliation.\nExit");
        std::process::exit(1);
    }
    // Analyse des transfers
    // ---------------------
    let gene_transfers = get_gtransfer(&mut gene_trees[0]);
    // let selected_transfers = select_transfer(&gene_transfers, &mut sp_tree);

    let selected_transfers =gene_transfers;
    println!("Optimisation: Gene transfers = {:?}",selected_transfers);
    let nbt = selected_transfers.len();
    println!("Optimisation: Number of transfers = {:?}",nbt);
    let mut numt = 0 ;
    while numt < nbt {
        classify_transfer(&selected_transfers[numt], &mut sp_tree, & numt);
        numt = numt + 1;
    }
    // println!("Optimisation: Species tree after : {:?}",sp_tree);
    let mut ordered = Vec::<usize>::new();
    reorder_transfers(&mut sp_tree, root, &mut ordered);
    ordered.reverse();
    println!("Optimisation: Reordered transfers : {:?}",ordered);
    for numt in ordered {
        let tr_root = optimisation(&selected_transfers[numt], &mut sp_tree);
        check_optimisation(&selected_transfers[numt], &mut sp_tree, tr_root);
    }
}
// ---------------------------------------------------------
// 3eme étape : Vérifie les conflits dans l'arbre d'espèces
// au niveau horizontal -> valeurs xmod
// ---------------------------------------------------------
 check_contour_postorder(&mut sp_tree, root);
// ---------------------------------------------------------
// 4eme étape : Décale toutes les valeurs de x en fonction
// de xmod dans l'abre d'espèces
// ---------------------------------------------------------
shift_mod_xy(&mut sp_tree, root, &mut 0.0, &mut 0.0);
// ---------------------------------------------------------
// 5eme étape : Place le parent entre les enfants dans
// l'arbre d'espèces
// ---------------------------------------------------------
set_middle_postorder(&mut sp_tree, root);
// ---------------------------------------------------------
// 6ème etape : Fixe l'épaisseur de l'arbre d'espèces
// ---------------------------------------------------------
set_species_width(&mut sp_tree);
// ---------------------------------------------------------
// 7ème étape :  Vérifie les conflits verticaux dans
// l'arbre d'espèces
// ---------------------------------------------------------
check_vertical_contour_postorder(&mut sp_tree, root, 0.0);
// ---------------------------------------------------------
// OPTIONAL Scale the heigt if needed
// ---------------------------------------------------------
if options.height != 1.0 { scale_heigth(&mut sp_tree,options.height)};
// ---------------------------------------------------------
// OPTIONAL Scale the width if needed
// ---------------------------------------------------------
if options.width != 1.0 { scale_width(&mut sp_tree,options.width)};
// ---------------------------------------------------------
// ---------------------------------------------------------
// 8ème étape :  mapping des noeuds de genes sur les noeuds
// d'espèce pour initialiser les coordonées des noeuds des
// arbres de gènes
// ---------------------------------------------------------
if mapping {
    map_gene_trees(&mut sp_tree,&mut gene_trees);
}
// ---------------------------------------------------------
// 9ème etape : décale les noeuds de gene associés à un
// noeud d'especes pour éviter qu'ils soit superposés
// ---------------------------------------------------------
bilan_mappings(&mut sp_tree, &mut gene_trees,root, & options);
// ---------------------------------------------------------
// 10ème étape : recalcule les coordonnées svg de tous les
// arbres de gènes
// ---------------------------------------------------------
let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
info!("map_species_trees: {} gene trees to be processed",nb_gntree);
let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
loop {
    let  groot = gene_trees[idx_rcgen].get_root();
    shift_mod_xy(&mut gene_trees[idx_rcgen], groot, &mut 0.0, &mut 0.0);
    idx_rcgen += 1;
    if idx_rcgen == nb_gntree {
        break;
    }
}
// ---------------------------------------------------------
// 11eme etape : centre les noeuds de genes dans le noeud de l'espece
// ---------------------------------------------------------
center_gene_nodes(&mut sp_tree,&mut gene_trees,root);
// ---------------------------------------------------------
// 12eme etape traite spécifiquement les duplications et les feuilles
// ---------------------------------------------------------
move_dupli_mappings(&mut sp_tree, &mut gene_trees,root);
// // ---------------------------------------------------------
// // Fin: Ecriture du fichier svg
// // ---------------------------------------------------------
println!("Output filename is {}",outfile);
match options.species_only_flag {
    true => {
        if options.species_internal {
             options.gene_internal = true;}
             draw_tree(&mut sp_tree, outfile, &options, &config);

    },
    false => draw_sptree_gntrees(&mut sp_tree, &mut gene_trees, outfile,
        &options, &config, &transfers),
};
}

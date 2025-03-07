/// name = "light_phylogeny"
/// version = "1.4.1"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"

// Convention: "pipe" trees are equivalent to "upper" trees, "path" trees are equivalenet to "lower" trees

use log::{info};
use std::fs;
use std::collections::HashMap;
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::Config;
use crate::arena::Event;
use crate::arena::PIPEBLOCK;
use crate::arena::{newick2tree,xml2tree,get_descendant};
use crate::arena::{knuth_layout,cladogramme,check_contour_postorder,
    check_contour_postorder_tidy_tree,shift_mod_xy,fusion_mod_xy,set_middle_postorder,real_length,
    set_leaves_y_values,shift_nodes_y_values};
use crate::arena::{map_species_trees,set_species_width,check_vertical_contour_postorder,
    bilan_mappings,bilan_mappings_reti,center_gene_nodes,move_dupli_mappings,move_species_mappings,
    species_uniformisation,process_fl,uniformise_gene_leaves_y_values};
use crate::arena::{find_sptrees,find_rgtrees,check_for_obsolete,scale_heigth,scale_width,make_invisible};
use crate::thirdlevel::{get_gtransfer,optimisation,check_optimisation,classify_transfer,reorder_transfers};
use crate::drawing::{draw_tree,draw_sptree_gntrees};

// Functions
// =========

/// Read a newick file and store the tree into ArenaTree structure.
pub fn read_newick(filename:String, tree: &mut ArenaTree<String>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("\nERROR:\nSomething went wrong when reading the newick file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let root = tree.new_node("Root".to_string());
    newick2tree(contents, tree, root, &mut 0);
}
/// Read a phyloxml file and store the tree into a ArenaTree structure.
pub fn read_phyloxml(filename: String, tree: &mut ArenaTree<String>) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("\nERROR:\nSomething went wrong when reading the phyloxml file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    let doc = roxmltree::Document::parse(&contents);
    let doc = match doc {
        Ok(xml) => xml,
        Err(_err) => {
            eprintln!("\nERROR:\nSomething went wrong when parsing the input file.");
            eprintln!("It seems this is not a correct xml file:");
            eprintln!("{}",_err);
            std::process::exit(1);
        },
    };
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
            xml2tree(node, root, &mut index, tree);
            break;
        }
    }
    info!("Tree : {:?}",tree);
}
/// Get the hybridation events in the species tree
pub fn get_hybridation( tree: & ArenaTree<String>) -> HashMap::<String,Vec<(usize,usize)>>
    {
    let mut dico = HashMap::<String,Vec<(usize,usize)>>::new();
    for index in  & tree.arena {
        let buf : Vec<_> = index.name.split(" ").collect();
        let tag = buf[0];
        if tag == "HYBRID" && buf.len() == 3 {
            let name_hybrid = buf[1].to_string();
            let parent = match index.parent {
                Some(p) => p,
                None => panic!("No root"),
            };
            let children = &tree.arena[parent].children;
            let idx = index.idx;
            let hybrid_from = match  tree.is_left(idx) {
                true => children[1],
                false => children[0],
            };

            if !dico.contains_key(&name_hybrid) {
                dico.insert(name_hybrid.clone(),vec![]);
            }
            if let Some(x) = dico.get_mut(&name_hybrid) {
                x.push((hybrid_from,idx));
            };
        }
    }
    dico
}
/// Adding invisible parent nodes in order to display multiple species trees.
pub fn add_virtual_roots(
    global_roots: &mut std::vec::Vec<String>,
    pere: &mut usize,
    level: &mut usize,
    global_pipe: &mut ArenaTree<String>,
    ) {
    if global_roots.len() == 1 {
        let nom_droite = "VIRTUAL_".to_string().to_owned() + &level.to_string() + "_"+&(*pere-1).to_string();
        let nom_pere = "ROOT_".to_string().to_owned() + &level.to_string() + "_"+&(*pere).to_string();
        let  node_pere = global_pipe.new_node(nom_pere.clone());
        global_pipe.arena[node_pere].name = nom_pere;
        global_pipe.arena[node_pere].visible = false;
        let  node_droite = global_pipe.node(nom_droite.clone());
        global_pipe.arena[node_droite].name = nom_droite;
        global_pipe.arena[node_droite].visible = false;
        let node_left = global_pipe.node(global_roots[0].to_string());
        global_pipe.arena[node_pere].children.push(node_left);
        global_pipe.arena[node_pere].children.push(node_droite);
        global_pipe.arena[node_left].parent = Some(node_pere);
        global_pipe.arena[node_droite].parent = Some(node_pere);
        if *pere >  0 {
            let mut my_vect : Vec<String> = (0..(*pere+1)).map(|n| "ROOT_".to_string().to_owned() + &level.to_string() + "_" + &n.to_string()).collect();
            *level = *level + 1;
            *pere = 0;
            add_virtual_roots(&mut my_vect, pere, level, global_pipe);
        }
    }
    if global_roots.len() == 2 {
        let nom_pere = "ROOT_".to_string().to_owned() + &level.to_string() + "_"+&(*pere).to_string();
        let  node_pere = global_pipe.node(nom_pere.clone());
        global_pipe.arena[node_pere].name = nom_pere;
        global_pipe.arena[node_pere].visible = false;
        let  node_droite = global_pipe.node(global_roots[1].to_string());
        let node_left = global_pipe.node(global_roots[0].to_string());
        global_pipe.arena[node_pere].children.push(node_left);
        global_pipe.arena[node_pere].children.push(node_droite);
        global_pipe.arena[node_left].parent = Some(node_pere);
        global_pipe.arena[node_droite].parent = Some(node_pere);
        if *pere > 0 {
            let mut my_vect : Vec<String> = (0..(*pere+1)).map(|n| "ROOT_".to_string().to_owned() + &level.to_string() + "_" + &n.to_string()).collect();
            *level = *level + 1;
            *pere = 0;
            add_virtual_roots(&mut my_vect, pere, level, global_pipe);
        }
    }
    if global_roots.len() > 2 {
        let  (tete, queue) = global_roots.split_at(2);
        let nom_pere = "ROOT_".to_string().to_owned() + &level.to_string() + "_"+&(*pere).to_string();
        let  node_pere = global_pipe.node(nom_pere.clone());
        global_pipe.arena[node_pere].name = nom_pere;
        global_pipe.arena[node_pere].visible = false;
        let node_droite = global_pipe.node(tete[1].to_string());
        let node_left = global_pipe.node(tete[0].to_string());
        global_pipe.arena[node_pere].children.push(node_left);
        global_pipe.arena[node_pere].children.push(node_droite);
        global_pipe.arena[node_left].parent = Some(node_pere);
        global_pipe.arena[node_droite].parent = Some(node_pere);
        let mut queue_clone = queue.to_vec();
        *pere = *pere + 1;
        add_virtual_roots(&mut queue_clone, pere,level, global_pipe);
    }
}
/// Read a recphyloxml file and store the species and gene trees into several ArenaTree structures.
pub fn read_recphyloxml_multi(
    filename: String,
    global_pipe: &mut ArenaTree<String>,
    gene_trees: &mut std::vec::Vec<ArenaTree<String>>,
    global_roots: &mut std::vec::Vec<usize>
    ) {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("\nERROR:\nSomething went wrong when reading the recPhyloXML file.");
            eprintln!("{}",err);
            eprintln!("Please check file name and path.");
            std::process::exit(1);
        },
    };
    // let doc = &mut roxmltree::Document::parse(&contents).unwrap();
    let doc = roxmltree::Document::parse(&contents);
    let doc = &mut match doc {
        Ok(contents) => contents,
        Err(_err) => {
            eprintln!("\nERROR:\nSomething went wrong when parsing the input file.");
            eprintln!("It seems this is not a correct xml file:");
            eprintln!("{}",_err);
            std::process::exit(1);
        },
    };
    // Get the species trees:
    // Get the list of nodes associated to  the "spTree" tag
    // let spnodes = find_sptrees(doc).expect("No clade spTree has been found in xml");
    let spnodes = find_sptrees(doc);
    let spnodes = match spnodes {
        Ok(indexes) => indexes,
        Err(_err) => {
            eprintln!("\nERROR:\nNo clade spTree has been found in xml.\
            \nIt seems that the input file is not a recPhyloXML file.\
            \nUse option -F to force to use phyloXML or newick format.");
             std::process::exit(1);
        },
    };
    let mut index = &mut 0;
    let mut global_root_names: std::vec::Vec<String> = Vec::new();
    for spnode in spnodes {
        info!("Search spTree node {:?}",spnode);
        let spnode = doc.get_node(spnode).expect("Unable to get the Node associated to this nodeId");
        info!("Associated spTree  : {:?}",spnode);
        // Analyse the gene tree
        let descendants = spnode.descendants();
        // Search for the first occurence of the "clade" tag
        // index = &(index + 1) ;
        *index += 1;
        for node in descendants {
            if node.has_tag_name("clade"){
                let globalroot = "G".to_owned() + &index.to_string();
                global_root_names.push(globalroot.clone());
                // Create the node, get its associated index and store it in root again
                info!("Create {}","G".to_owned() + &index.to_string());
                let globalroot = global_pipe.new_node(globalroot.to_string());
                global_roots.push(globalroot);
                // Call xlm2tree on the root
                xml2tree(node, globalroot, &mut index, global_pipe);
                break;
            }
        }
    }
    if global_roots.len() > 1 {
        add_virtual_roots(&mut global_root_names, &mut 0, &mut 0, global_pipe);
    }
    let  nb_sptree =  global_roots.len().clone();
    println!("Number of species trees : {}",nb_sptree);
    info!("List of species roots : {:?}",global_roots);
    info!("Species tree(s) : {:?}",global_pipe);
    // Get the gene trees:
    // Get the list of nodes associated to  the "recGeneTree" tag
    // let rgnodes = find_rgtrees(doc).expect("No clade recGeneTree has been found in xml");
    let rgnodes = find_rgtrees(doc);
    let rgnodes = match rgnodes {
        Ok(indexes) => indexes,
        Err(_err) => {
            eprintln!("\nERROR:\nNo clade recGeneTree has been found in xml.\
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
                let mut index = &mut 0;
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
    let  nb_gntree = gene_trees.len().clone();
    println!("Number of gene trees : {}",nb_gntree);
    info!("List of gene trees : {:?}",gene_trees);
}
/// Create a svg of the tree in phyloxml context.
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
    knuth_layout(&mut tree, root, &mut 1);
    // ---------------------------------------------------------
    // Option : Cladogramme
    // ---------------------------------------------------------
    if options.clado_flag {
        cladogramme(&mut tree);
    }

    // ------------------------------------
    // Option collapse tree node
    // ------------------------------------
    for collapsed_node in &options.collapsed_nodes {
        let sw = match tree.get_index(collapsed_node.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[phyloxml_processing] ERROR Unable to find node {:?}",collapsed_node.to_string());
                std::process::exit(1);
            },
        };
        let mut descendants:Vec<usize> = Vec::new();
        get_descendant(tree,sw,&mut descendants);
        for index in descendants {
            tree.arena[index].visible = false;
            tree.arena[index].parent = None;
        }
        tree.arena[sw].children = Vec::new();
        tree.arena[sw].collapsed = true;
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
    if options.height != 1.0 {scale_heigth(&mut tree,options.height)};
    // ---------------------------------------------------------
    // OPTIONAL Scale the width if needed
    // ---------------------------------------------------------
    if options.width != 1.0 {scale_width(&mut tree,options.width)};
    // ---------------------------------------------------------
    // Option : real_length
    // ---------------------------------------------------------
    if options.real_length_flag {
        real_length(&mut tree, root, &mut 0.0, & options);
        if options.tidy{
            check_contour_postorder_tidy_tree(&mut tree, root, & options, & config);
            set_middle_postorder(&mut tree, root);
        }
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);
    draw_tree(&mut tree, outfile, & options, & config);
}
/// Create a svg of the tree in recphyloxml context.
pub fn recphyloxml_processing(
    mut sp_tree: &mut ArenaTree<String>,                    // species tree
    mut gene_trees: &mut std::vec::Vec<ArenaTree<String>>,  // gene trees
    options: &mut Options,                                  // display options
    config: & Config,                                        // svg configuration
    mapping: bool,                                           // map gene and species
    transfers: & std::vec::Vec<(String,String)>,            // optional additional transfers
    outfile: String,                                         // output file
) {
    // -----------------------
    // Traitement en 12 etapes
    // -----------------------
    // Au depart l'arbre est orienté du haut vers le bas (i.e. selon Y)
    // Le svg sera tourné de -90 a la fin.
    //
    //  Option : ajout d'une branche free_living
    let initial_root = sp_tree.get_root();
    let mut free_root = 0;
    if options.free_living {
        // Check if a free_living is already in the data
        let test_root = sp_tree.get_index("FREE_LIVING_ROOT".to_string());
        match test_root {
            Ok(root) => {
                info!("[recphyloxml_processing] FREE_LIVING_ROOT already in the tree.");
                let free_children = & sp_tree.arena[root].children;
                free_root = free_children[1];
                info!("FREE_LIVING_ROOT : {}",free_root);
            },
            Err(_err) => {
                info!("[recphyloxml_processing] No FREE_LIVING_ROOT  in the tree, I create one.");
                let left = sp_tree.get_root();
                let right = sp_tree.new_node("free_living".to_string());
                sp_tree.arena[right].name = "FREE_LIVING".to_string();
                free_root = right;
                let fl_root = sp_tree.new_node("free_living_root".to_string());
                sp_tree.arena[fl_root].name = "FREE_LIVING_ROOT".to_string();
                sp_tree.arena[fl_root].visible = false;
                sp_tree.arena[fl_root].children.push(left);
                sp_tree.arena[fl_root].children.push(right);
                sp_tree.arena[right].parent = Some(fl_root);
                sp_tree.arena[left].parent = Some(fl_root);
                info!("[recphyloxml_processing] FREE_LIVING_ROOT : {}",free_root);
            },
        };
    }
    // ------------------------------------
    // Option switch species tree node
    // ------------------------------------
    for switch in &options.switches {
        let sw = match sp_tree.get_index(switch.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",switch.to_string());
                std::process::exit(1);
            },
        };
        let mut children : Vec<usize> = Vec::new();
        children.push(sp_tree.arena[sw].children[1]);
        children.push(sp_tree.arena[sw].children[0]);
        sp_tree.arena[sw].children = children;
    }
    //----------------------------------------------------------
    // 1ere étape :initialisation des x,y de l'arbre d'espèces :
    // profondeur => Y, left => X= 0, right X=1
    // ---------------------------------------------------------
    let root = sp_tree.get_root();
    knuth_layout(&mut sp_tree,root, &mut 1);
    // --------------------
    // Option  Cladogramme
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
        map_species_trees(&mut sp_tree, &mut gene_trees);
        info!("Species tree after mapping : {:?}",sp_tree);
    }

    // ------------------------------------
    // Option collapse species tree node
    // ------------------------------------
    for collapsed_node in &options.collapsed_nodes {
        let sw = match sp_tree.get_index(collapsed_node.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",collapsed_node.to_string());
                std::process::exit(1);
            },
        };
        make_invisible(sp_tree,gene_trees,sw);
        for (idx_tree,idx_node) in  &sp_tree.arena[sw].nodes {
            gene_trees[*idx_tree].arena[*idx_node].collapsed = true;
        }
        sp_tree.arena[sw].children = Vec::new();
        sp_tree.arena[sw].collapsed = true;
    }

    // --------------------------------------------------
    //  Option  uniformise les noeuds de l'arbre d'espece
    // --------------------------------------------------
    if options.uniform {
        species_uniformisation(&mut sp_tree);
        // Il fait decaler le tout (mais je ne comprend pas pourquoi?)
        let smallest_y = sp_tree.get_smallest_y();
        let root_width = sp_tree.arena[root].nbg as f32 * PIPEBLOCK ;
        shift_nodes_y_values(sp_tree, root, -smallest_y + root_width);
    }
    // ---------------------------------------------------------
    // Option  utilise les longueurs de branches
    // ---------------------------------------------------------
    if options.real_length_flag {
        //  Tout d'abord il faut uniformiser
        species_uniformisation(&mut sp_tree);
        let min_dist = sp_tree.get_smallest_l();
        if min_dist == 0.0 {
            eprintln!("\nERROR:\nFound a branch with a distance equal to 0.0 in the species (or 'upper') tree.");
            eprintln!("It is not possible create an 'upper' tree presenting branches of zero length.");
            std::process::exit(1);
        }
        // On veut que la longueur minimum soit un peu superieure a la moitie de l'epaisser des noods
        // On utilise le nombre de gene , qui a etet uniformise precedemment
        // options.scale = (sp_tree.arena[root].nbg as f32 / 2.0  + 0.25)  / min_dist ;
        let optimised_factor = (sp_tree.arena[root].nbg as f32 / 2.0  + 0.25)  / min_dist ;
        options.scale = optimised_factor * options.scale ;
        real_length(&mut sp_tree, root, &mut 0.0, & options);
        // Il faut decaler le tout (mais je ne comprend pas pourquoi?)
        let smallest_y = sp_tree.get_smallest_y();
        let root_width = sp_tree.arena[root].nbg as f32 * PIPEBLOCK ;
        shift_nodes_y_values(sp_tree, root, -smallest_y + root_width);
    }
    // ---------------------------------------------------------
    // Oprion Optimisation if needed
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
        let selected_transfers = gene_transfers;
        println!("Optimisation: Gene transfers = {:?}",selected_transfers);
        let nbt = selected_transfers.len();
        println!("Optimisation: Number of transfers = {:?}",nbt);
        let mut numt = 0 ;
        while numt < nbt {
            classify_transfer(&selected_transfers[numt], &mut sp_tree, & numt);
            numt = numt + 1;
        }
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
    // de xmod dans l'arbre d'espèces
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
    set_species_width(&mut sp_tree, &mut gene_trees);
    // ---------------------------------------------------------
    // 7ème étape :  Vérifie les conflits verticaux dans
    // l'arbre d'espèces
    // ---------------------------------------------------------
    if ! options.real_length_flag {
        check_vertical_contour_postorder(&mut sp_tree, root, 0.0, &options.species_compression);
    }
    // Processing  recphyloxml hybrids if present
    let hybrids = get_hybridation(& sp_tree);
    let mut fusion_orders_recphylo:Vec<bool> = Vec::new();
    for (hybrid_name, hybrid) in &hybrids {
        if hybrid.len() != 2 {
            println!("ERROR in the hybridation {} {:?}",hybrid_name,hybrid);
            println!("      Please check the recPhyloXML file.");
            std::process::exit(1);
        }
        let hybrid1 = hybrid[0]; // host1 , host1 hybrided with host2
        let hybrid2 = hybrid[1]; // host2 , host2 hybrided with host1
        let h1 = hybrid1.1; //host1 hybrided with host2
        let h2 = hybrid2.1; //host2 hybrided with host1
        let o1 = hybrid1.0; // host1
        let o2 = hybrid2.0; // host2
        fusion_orders_recphylo.push(sp_tree.arena[h1].x < sp_tree.arena[h2].x);
        // Fusionne les branche hybrides
        fusion_mod_xy(&mut sp_tree, h1, h2);
        let root1 = match sp_tree.arena[h1].parent{
            Some(p) => p,
            None => panic!("No root"),
        };
        // Aligne le fils avec la racine
        let root2 = match sp_tree.arena[h2].parent{
            Some(p) => p,
            None => panic!("No root"),
        };
        sp_tree.arena[o1].x = sp_tree.arena[root1].x;
        sp_tree.arena[o2].x = sp_tree.arena[root2].x;
        // Donne la largeur du fils a la racine
        let max = match sp_tree.arena[o1].height > sp_tree.arena[o2].height {
            true => sp_tree.arena[o1].height,
            false => sp_tree.arena[o2].height,
        };
        if sp_tree.arena[root1].height != sp_tree.arena[root2].height {
        // Donne la longeur la plus grande des 2 hybrdies a la racine
        sp_tree.arena[root1].height = max ;
        sp_tree.arena[root2].height = max ;
        }
        // Donne la largeur du fils a la racine
        sp_tree.arena[root1].width = sp_tree.arena[o1].width ;
        sp_tree.arena[root2].width = sp_tree.arena[o2].width ;

        // change l'evenement lie au gene speciation-> hybridation
        let nodes = &sp_tree.arena[root1].nodes;
        for (index_node, node) in nodes {
            if gene_trees[*index_node].arena[*node].e == Event::Speciation {
                 gene_trees[*index_node].arena[*node].e = Event::Hybridation;
            }
            let children = &gene_trees[*index_node].arena[*node].children;
            // gere le cas ou la racine est a gauche du fils gauche ( ou a droite du fils droit)
            if gene_trees[*index_node].arena[*node].x < gene_trees[*index_node].arena[children[0]].x {
                gene_trees[*index_node].arena[*node].x = gene_trees[*index_node].arena[children[0]].x;
            }
        }
        let nodes = &sp_tree.arena[root2].nodes;
        for (index_node, node) in nodes {
            if gene_trees[*index_node].arena[*node].e == Event::Speciation {
                 gene_trees[*index_node].arena[*node].e = Event::Hybridation;
            }
        }
    }

    // -------------------------------
    // Option gestion des hybridations
    // -------------------------------
    // Sens dans lequel decaler les noeuds de genes apres la fusion
    let mut fusion_orders:Vec<bool> = Vec::new();
    for (name1, name2) in &options.hybrid {
        let h1 = match sp_tree.get_index(name1.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name1.to_string());
                std::process::exit(1);
            },
        };
        let h2 = match sp_tree.get_index(name2.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name2.to_string());
                std::process::exit(1);
            },
        };

        println!("Merging nodes {} [{}] and {}  [{}]",h1,sp_tree.arena[h1].name,h2,sp_tree.arena[h2].name);
        fusion_orders.push(sp_tree.arena[h1].x < sp_tree.arena[h2].x);
        fusion_mod_xy(&mut sp_tree, h1, h2);
    }
    // ---------------------------------------------------------
    // Egalise les feuilles
    // ---------------------------------------------------------
    if ! options.real_length_flag {
        // Egalise les feuilles
        let largest_y  = sp_tree.get_largest_y();
        set_leaves_y_values(sp_tree, root, largest_y);
    } else {
        if options.tidy{
            check_contour_postorder_tidy_tree(&mut sp_tree, root, & options, & config);
        }
    }
    // -------------------------------------------------
    // Option collapse species tree node; shifting nodes
    // -------------------------------------------------
    for collapsed_node in &options.collapsed_nodes {
        let sw = match sp_tree.get_index(collapsed_node.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",collapsed_node.to_string());
                std::process::exit(1);
            },
        };
        match sp_tree.arena[sw].parent {
            None => {},
            Some(p) => {
                let shift_y = ( sp_tree.arena[sw].y - sp_tree.arena[p].y) / 2.0;
                sp_tree.arena[sw].y = sp_tree.arena[sw].y - shift_y;

            }
        }

    }
    // ---------------------------------------------------------
    // Option Scale the heigt if needed
    // ---------------------------------------------------------
    if options.height != 1.0 {scale_heigth(&mut sp_tree, options.height)};
    // ---------------------------------------------------------
    // Option Scale the width if needed
    // ---------------------------------------------------------
    if options.width != 1.0 { scale_width(&mut sp_tree, options.width)};
    // ---------------------------------------------------------
    // 8ème etape : décale les noeuds de gene associés à un
    // noeud d'especes pour éviter qu'ils soit superposés
    // ---------------------------------------------------------
    bilan_mappings(&mut sp_tree, &mut gene_trees, initial_root, & options);
    // ---------------------------------------------------------
    // 9eme etape : centre les noeuds de genes dans le noeud de l'espece
    // ---------------------------------------------------------
    center_gene_nodes(&mut sp_tree,&mut gene_trees, initial_root);

    // ----------------------------------------------------------
    // Option gestion des hybridations definies dans le recPhyloXML
    // ----------------------------------------------------------
    // Processing recphylo hybrids if present: change gene positions
    // to avoid superposition
    let mut idx_fusion = 0;
    for (hybrid_name, hybrid) in &hybrids {
        let hybrid1 = hybrid[0];
        let hybrid2 = hybrid[1];
        let h1 = hybrid1.1;
        let h2 = hybrid2.1;
        println!("Hybrid {}: {:?} between {} and {}",hybrid_name,hybrid,sp_tree.arena[hybrid1.0].name,sp_tree.arena[hybrid2.0].name);
        let fusion_order = fusion_orders_recphylo[idx_fusion];
        let fusion_order_inv = ! fusion_order;
        bilan_mappings_reti(&mut sp_tree, &mut gene_trees, h1, fusion_order);
        bilan_mappings_reti(&mut sp_tree, &mut gene_trees, h2, fusion_order_inv);
        idx_fusion += 1;
    }
    if idx_fusion > 0 {
        options.fill_species = true ;
    }
    // Processing recphylo hybrids if present: adapt root position
    // to the new gene positions
    for (_hybrid_name, hybrid) in &hybrids {
        let hybrid1 = hybrid[0];
        let hybrid2 = hybrid[1];
        let h1 = hybrid1.1;
        let h2 = hybrid2.1;
        let root1 = match sp_tree.arena[h1].parent{
            Some(p) => p,
            None => panic!("No root"),
        };
        // Aligne le fils avec la racine
        let root2 = match sp_tree.arena[h2].parent{
            Some(p) => p,
            None => panic!("No root"),
        };
        for root in vec![root1,root2].iter() {
            let nodes = &sp_tree.arena[*root].nodes;
            for (index_node, node) in nodes {
                let children =  & gene_trees[*index_node].arena[*node].children;
                // gere le cas ou la racine est a gauche du fils gauche ( ou a droite du fils droit)
                if gene_trees[*index_node].arena[*node].x < gene_trees[*index_node].arena[children[1]].x {
                    gene_trees[*index_node].arena[*node].x = gene_trees[*index_node].arena[children[1]].x;
                }
            }
            // 2nd round (not possible to modify twice in a single round)
            for (index_node, node) in nodes {
                let children =  & gene_trees[*index_node].arena[*node].children;
                // gere le cas ou la racine est a gauche du fils gauche ( ou a droite du fils droit)
                if gene_trees[*index_node].arena[*node].x > gene_trees[*index_node].arena[children[1]].x {
                    gene_trees[*index_node].arena[*node].x = gene_trees[*index_node].arena[children[1]].x;
                }
            }
        }
    }
    // Change the species node name
    for (hybrid_name, hybrid) in &hybrids {
        let hybrid1 = hybrid[0];
        let hybrid2 = hybrid[1];
        let h1 = hybrid1.1;
        let h2 = hybrid2.1;
        sp_tree.arena[h1].name = hybrid_name.to_string();
        sp_tree.arena[h2].name = "".to_string();
    }
    // ----------------------------------------------------------
    // Option gestion des hybridations definies par l'utilisateur
    // ----------------------------------------------------------
    let mut idx_fusion = 0;
    for (name1, name2) in &options.hybrid {
        let h1 = match sp_tree.get_index(name1.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name1.to_string());
                std::process::exit(1);
            },
        };
        let h2 = match sp_tree.get_index(name2.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name2.to_string());
                std::process::exit(1);
            },
        };
        let fusion_order = fusion_orders[idx_fusion];
        let fusion_order_inv = ! fusion_order;
        bilan_mappings_reti(&mut sp_tree, &mut gene_trees, h1, fusion_order);
        bilan_mappings_reti(&mut sp_tree, &mut gene_trees, h2, fusion_order_inv);
        idx_fusion += 1;
    }
    if idx_fusion > 0 {
        options.fill_species = true ;
    }
    for (name1, name2) in &options.hybrid {
        let fusion_name  = &(name1.to_owned() + " hybrid. " + name2) ;
        let h1 = match sp_tree.get_index(name1.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name1.to_string());
                std::process::exit(1);
            },
        };
        let h2 = match sp_tree.get_index(name2.to_string()){
            Ok(index) => index,
            Err(_err) => {
                eprintln!("[recphyloxml_processing] ERROR Unable to find node {:?}",name2.to_string());
                std::process::exit(1);
            },
        };
        sp_tree.arena[h1].name = fusion_name.to_string();
        sp_tree.arena[h2].name = "".to_string();
    }
    // ---------------------------------------------------------
    // 10eme etape traite spécifiquement les duplications et les feuilles
    // ---------------------------------------------------------
    move_dupli_mappings(&mut sp_tree, &mut gene_trees, initial_root, & options);
    // 11eme etape : Ca des speciations dont les fils sont
    // dans le meme noeud "tuyeau" que le pere
    // Cela n'arrice que quand on mappe des genes sur des hotes
    // via les parasites (thirdlevel)
    // ---------------------------------------------------------
    move_species_mappings(&mut sp_tree, &mut gene_trees, initial_root);
    // Option : traitement specifique des gene "free living"
    if options.free_living {
        process_fl(&mut sp_tree, &mut gene_trees, free_root, &options);
    }
    if ! options.real_length_flag {
        // Egalise les feuilles 2
        uniformise_gene_leaves_y_values(sp_tree, gene_trees);
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);
    match options.species_only_flag {
        true => {
            if options.species_internal {
                options.gene_internal = true;
                }
            draw_tree(&mut sp_tree, outfile, &options, &config);
        },
        false => draw_sptree_gntrees(&mut sp_tree, &mut gene_trees, outfile,
            &options, &config, &transfers),
    };
}

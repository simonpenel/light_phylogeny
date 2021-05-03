/// name = "light_phylogeny"
/// version = "0.10.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"
use log::{info};
use crate::arena::ArenaTree;
use crate::arena::Event;
use crate::arena::{lca};
// map a transfert in gene  tree to the species tree
pub fn map_transfer(transfers: Vec<(String,String)> , parasite_tree: &mut ArenaTree<String>) -> Vec<(String,String)> {
        let mut  map_transfers = vec![];
        for (start, end) in transfers {
            let map_start = parasite_tree.get_index(start.to_string())
                .expect("[map_transfer] unable to find start node");
            let map_start = parasite_tree.arena[map_start].location.to_string();
            let map_end = parasite_tree.get_index(end.to_string())
                .expect("[map_transfer] unable to find end node");
            let map_end = parasite_tree.arena[map_end].location.to_string();
            map_transfers.push((map_start,map_end));
        }
        map_transfers
}

//  get the trasnsfers in a gene tree
pub fn get_gtransfer(gene_tree: &mut ArenaTree<String>) -> Vec<(String,String)> {
    let mut  transfers = vec![];
    for index in  &gene_tree.arena {
        if index.is_a_transfert {
            let parent = index.parent;
            let parent = match parent {
                Some(p) => p,
                None => panic!("[get_gtransfert] the node has no parent"),
            };
            let end = index.location.to_string();
            let start = gene_tree.arena[parent].location.to_string();
            transfers.push((start,end));
        };
    };
    transfers
}

// Map a reconciled species tree to a pipe species tree
pub fn map_parasite_g2s(para_as_species: &mut ArenaTree<String>,para_as_gene: &mut ArenaTree<String>,) {
    // Explore pipe species tree
    for index in  &mut para_as_species.arena {
        let name = &index.name;
        let i = para_as_gene.get_index(name.to_string());
        // Search the pipe species node into the reconciled species tree
        match i {
            // If mapping is successful attribute the event and is_a_transfert to the node
            Ok(i) => {
                let e = &para_as_gene.arena[i].e;
                info!("[map_parasite_g2s] Mapping of {} OK, event is {:?} (transfert: {})",name,e,&para_as_gene.arena[i].is_a_transfert);
                index.is_a_transfert = para_as_gene.arena[i].is_a_transfert;
                index.e = match  e {
                    &Event::Duplication => Event::Duplication,
                    &Event::BranchingOut => Event::BranchingOut,
                    &Event::Speciation => Event::Speciation,
                    &Event::BifurcationOut => Event::BifurcationOut,
                    &Event::Loss => Event::Loss,
                    &Event::Leaf => Event::Leaf,
                    _ => {println!("Event {:?} not selected",e);
                         Event::Undef},
                };
            },
            Err(_err) => {
                info!("map_parasite_g2s] Unable to map {}",name);
            },
        }
    }
}

// Map pipe species tree to reconciled species tree
pub fn map_parasite_s2g(para_as_species: &mut ArenaTree<String>,
    para_as_gene: &mut ArenaTree<String>,
    gene_trees : &mut std::vec::Vec<ArenaTree<String>>) {
    let mut virt_svg = 0; //Increment for virtual svg nodes
    // Explore reconciled species tree
    for index in &para_as_gene.arena {
        let name = &index.name;
        //  Search the node in the pipe species tree
        let i = para_as_species.get_index(name.to_string());
        match i {
            Ok(_i) => {
                info!("[map_parasite_s2g] Mapping of {} OK",name);
            },
            Err(_err) => {
                // We need to define a new node in the pipe species tree
                info!("[map_parasite_s2g] Unable to map {} {:?}, new node needed",name,&index.e);
                // To do that we nee the parent of the missing node
                let p = &index.parent;
                let p = match p {
                    Some(p) => p,
                    None => panic!("[map_parasite_s2g] Error: node as not parent"),
                };
                // p is the parent of the node in the rec species which is not found
                // in the pipe species
                let parent_name = para_as_gene.arena[*p].name.clone();
                info!("[map_parasite_s2g]  => parent of the node in the reconciled species tree is {}({})",
                    p,parent_name);
                // let's find this node in the pipe species tree
                let j = para_as_species.get_index(parent_name.to_string());
                let j = match j {
                    Ok(j) => j,
                    Err(_e) => panic!("Unable to find parent in 'species' tree"),
                };
                // j is the mapping of p  in the spec tree
                info!("[map_parasite_s2g]  => Mapping of parent {} OK",parent_name);
                match index.e {
                    // the missing node is a loss, I add it to the parent j
                    // (which have 3 children now)
                    Event::Loss => {
                        info!("[map_parasite_s2g]  => missing node is a Loss, I add it to parent");
                        //  New pipe species node with the name find in the rec species tree
                        let new_loss = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_loss].name = name.to_string();
                        // Add the new pipe species node to j
                        para_as_species.arena[new_loss].parent = Some(j);
                        para_as_species.arena[j].children.push(new_loss);
                        info!("[map_parasite_s2g]  => parent is now : {:?}",para_as_species.arena[j]);
                    },
                    _ => {
                        info!("[map_parasite_s2g]  => missing node is a not Loss, I insert it between  parent and chidren");
                        //  New pipe species node with the name find in the rec species tree
                        let new_node = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_node].name = name.to_string();
                        // Children of j
                        let children = &para_as_species.arena[j].children;
                        // Insert the new node between j and its children
                        // I suppose there is 2 nodes, even it mayx exist a third one which is
                        // the loss node previously added
                        let left = children[0];
                        let right = children[1];
                        // left and right will be the children of new_node and new_node
                        // will be their parent
                        para_as_species.arena[left].parent = Some(new_node);
                        para_as_species.arena[right].parent = Some(new_node);
                        para_as_species.arena[new_node].children.push(left);
                        para_as_species.arena[new_node].children.push(right);
                        //  left and rigth are not the children of j anymore
                        para_as_species.arena[j].children.retain(|&x| x !=  left);
                        para_as_species.arena[j].children.retain(|&x| x !=  right);
                        // new_node is a child of j and j is its father
                        para_as_species.arena[j].children.push(new_node);
                        para_as_species.arena[new_node].parent = Some(j);
                        //  Transfering the gene nodes associated to j in  the pipe species
                        //  to the new pipe species node
                        let gnodes = &para_as_species.arena[j].nodes;
                        let &nbg = &para_as_species.arena[j].nbg;
                        para_as_species.arena[new_node].nodes = gnodes.to_vec();
                        para_as_species.arena[new_node].nbg = nbg;
                        para_as_species.arena[j].nbg = 0;
                        para_as_species.arena[j].nodes = [].to_vec();
                        // Now we modify the location of  the gene nodes (gene species maping)
                        // We may have to add  a node in  the gene tree for j
                        // gene nodes associated to the new rec species node:
                        let new_gnodes = &para_as_species.arena[new_node].nodes;
                        // Array of gene nodes we will have to deal with  afterrwawrds
                        let mut add_gnodes = [].to_vec();
                        for (ng,nn) in new_gnodes { // ng is the tree number, nn the index of node
                            info!("[map_parasite_s2g] Modify gene tree number {}",*ng);
                            info!("[map_parasite_s2g] Redefine location of the node.");
                            info!("[map_parasite_s2g] Previous {:?}:", &gene_trees[*ng].arena[*nn]);
                            gene_trees[*ng].arena[*nn].location = para_as_species.arena[new_node].name.to_string();
                            info!("[map_parasite_s2g] New {:?}:", &gene_trees[*ng].arena[*nn]);
                            // In case the node is not a root (I expect index 0 is root, maybe this
                            // is not perfect) we need to add a gene node to map exactly with the
                            // pipe species, except if the node is transfer
                            // NOTE :  IN FACT INDEX 0 IS NOT A CORRECT INDICATION IF FORMAT IS NEWICK
                            if (nn > &0) && (!index.is_a_transfert) {
                                info!("[map_parasite_s2g] Node is not a gene root nor a transfer: adding 2 new genes.");
                                // We need to add 2 virtual svg nodes, 1 to be displayed in the
                                // pipe node j, and 1 because we want a binary tree.
                                // parent du noeud traité
                                let p = gene_trees[*ng].arena[*nn].parent;
                                let parent =  match p {
                                    Some(p) => p,
                                    None => panic!("[map_parasite_s2g] Unable to find parent"),
                                };
                                // nouveau noeud 1
                                let new_svgnode = gene_trees[*ng].new_node("virtualsvg_".to_string()
                                    + &virt_svg.to_string());
                                virt_svg = virt_svg + 1 ;
                                // A verifier
                                gene_trees[*ng].arena[new_svgnode].location = index.name.clone();
                                // nouveau noeud 2
                                let new_svgnode_bis = gene_trees[*ng].new_node("virtualsvg_".to_string()
                                    + &virt_svg.to_string());
                                virt_svg = virt_svg + 1 ;
                                // A verifier
                                gene_trees[*ng].arena[new_svgnode_bis].location = index.name.clone();
                                add_gnodes.push((j,*ng,(new_svgnode_bis,new_svgnode)));
                                // para_as_species.arena[j].nodes.push = [].to_vec();
                                //  le nouveau noeud a comme parent le parent du noeud traité
                                gene_trees[*ng].arena[new_svgnode].parent=p;
                                //  le noeud traite a comme parent le nouveau noeud
                                gene_trees[*ng].arena[*nn].parent=Some(new_svgnode);
                                //  le noeud bis a comme parent le nouveau noeud
                                gene_trees[*ng].arena[new_svgnode_bis].parent=Some(new_svgnode);
                                gene_trees[*ng].arena[new_svgnode].children.push(new_svgnode_bis);
                                gene_trees[*ng].arena[new_svgnode].children.push(*nn);
                                gene_trees[*ng].arena[parent].children.push(new_svgnode);
                                gene_trees[*ng].arena[parent].children.retain(|&x| x !=  *nn);
                                info!("[map_parasite_s2g] new gene added.");
                            }
                        }
                        for (node,ng,(node1,node2))  in add_gnodes {
                            info!("[map_parasite_s2g] Adding gene nodes of tree number {} to species node {} ({},{})",ng,node,node1,node2);
                            para_as_species.arena[node].nbg = para_as_species.arena[node].nbg + 1;
                            para_as_species.arena[node].nodes.push((ng,node1)) ;
                            para_as_species.arena[node].nbg = para_as_species.arena[node].nbg + 1;
                            para_as_species.arena[node].nodes.push((ng,node2)) ;
                        }
                    },
                };


            },
        }
    }
}

//  get the trasnsfers in a gene tree
pub fn select_transfer(transfers: & Vec<(String,String)>,species_tree: &mut ArenaTree<String>) -> Vec<(String,String)> {
let mut  selected = vec![];
for (start,end) in transfers {
    let s = species_tree.get_index(start.to_string()).expect("[select_transfer] Unable fo find start");
    let e = species_tree.get_index(end.to_string()).expect("[select_transfer] Unable fo find start");
    let ancestor = lca(species_tree,s,e);
    if (ancestor != s) && (ancestor != e) {
        selected.push((start.clone(),end.clone()));
    }
}
selected

}

//  get the trasnsfers in a gene tree
pub fn optimisation(transfer: & (String,String),species_tree: &mut ArenaTree<String>) -> usize {
    println!("");
        let  (start,end) = transfer;
    println!("=========================================");
    println!("===> TRANSFER {}->{}",start,end);
    println!("=========================================");
    // est ce le start est a droite du end?
    let s = species_tree.get_index(start.to_string()).expect("[select_transfer] Unable fo find start");
    let e = species_tree.get_index(end.to_string()).expect("[select_transfer] Unable fo find start");
    let ancestor = lca(species_tree,s,e);
    println!("Ancestor of {}->{} is {}",start,end, &species_tree.arena[ancestor].name);
    //  le noeu de droite de l'ancetre
    let droite = species_tree.arena[ancestor].children[1];
    let gauche = species_tree.arena[ancestor].children[0];
    println!("Rigth son of ancestor is {}",&species_tree.arena[droite].name);
    let mut _right_left_start = false;
    if droite == s {
        _right_left_start = true;
    }
    else {
        let mut parent = species_tree.arena[s].parent;
        while parent != None {
            let p = match parent {
                Some(p) => p,
                None => panic!("[select_transfer] unexpected None"),
            };
            if p == droite {
                _right_left_start = true;
            }
            parent = species_tree.arena[p].parent;
        }
    }
    if _right_left_start {
        println!("The start is on the right");
// Si le start est a droit j'ajour un go_left a tous les noeuds entre le star et la racine
// et un go right  a tous les noeuds entr ele end et la racine

// SAIF QUNAD ON EST SOI MEME KE PERE
    println!(" Start: Check if {} is not {} an not root",species_tree.arena[s].name,species_tree.arena[droite].name);
    if (s != droite ) && ( s!= ancestor ){
        species_tree.arena[s].go_left = species_tree.arena[s].go_left +1;
        println!("increment go left at {}",species_tree.arena[s].name);
    // }
    let mut parent = species_tree.arena[s].parent;
    println!("Start: Add a go left from parent of {} to {}",species_tree.arena[s].name,species_tree.arena[droite].name);
    while (parent != Some(droite) && parent != Some(ancestor)  && parent != None) {
                let p = match parent {
                Some(p) => p,
                None => panic!("[select_transfer] unexpected None"),
            };
            println!("increment go left at {}",species_tree.arena[p].name);
            species_tree.arena[p].go_left = species_tree.arena[p].go_left +1;
            parent = species_tree.arena[p].parent;

    }
}
    println!("End: Check if {} is not {} and not root",species_tree.arena[e].name,species_tree.arena[gauche].name);

    if (e != gauche ) && ( e!= ancestor) {
        species_tree.arena[e].go_right = species_tree.arena[e].go_right +1;
        println!("increment go right at {}",species_tree.arena[e].name);
    // }
    // else {
    let mut parent = species_tree.arena[e].parent;
    println!("End: Add a go right from parent of {} to {}",species_tree.arena[e].name,species_tree.arena[gauche].name);

    while (parent != Some(gauche) && parent != Some(ancestor)) {
                let p = match parent {
                Some(p) => p,
                None => panic!("[select_transfer] unexpected None"),
            };
            println!("increment go right at {}",species_tree.arena[p].name);
            species_tree.arena[p].go_right = species_tree.arena[p].go_right +1;
            parent = species_tree.arena[p].parent;
    // }
}
}


} else {
    println!("The start is on the left");

    // SAIF QUNAD ON EST SOI MEME KE PERE
        println!(" Start: Check if {} is not {} and not root",species_tree.arena[s].name,species_tree.arena[gauche].name);
        if (s != gauche ) && ( s!= ancestor){
            species_tree.arena[s].go_right = species_tree.arena[s].go_right +1;
            println!("increment go rigth at {}",species_tree.arena[s].name);
        // }
        // else {
        let mut parent = species_tree.arena[s].parent;
        println!("Start: Add a go right from parent of {} to {}",species_tree.arena[s].name,species_tree.arena[gauche].name);
        while (parent != Some(gauche) && parent != Some(ancestor)  && parent != None) {
                    let p = match parent {
                    Some(p) => p,
                    None => panic!("[select_transfer] unexpected None"),
                };
                println!("increment go right at {}",species_tree.arena[p].name);
                species_tree.arena[p].go_right = species_tree.arena[p].go_right +1;
                parent = species_tree.arena[p].parent;

        }
}
        println!("End: Check if {} is not {} and not root",species_tree.arena[e].name,species_tree.arena[droite].name);

        if (e != droite ) && ( e != ancestor ){
            species_tree.arena[e].go_left = species_tree.arena[e].go_left +1;
            println!("increment go left at {}",species_tree.arena[e].name);
        // }
        let mut parent = species_tree.arena[e].parent;
        println!("End: Add a go left from parent of {} to {}",species_tree.arena[e].name,species_tree.arena[droite].name);

        while (parent != Some(droite) && parent != Some(ancestor)) {
                    let p = match parent {
                    Some(p) => p,
                    None => panic!("[select_transfer] unexpected None"),
                };
                println!("increment go left at {}",species_tree.arena[p].name);
                species_tree.arena[p].go_left = species_tree.arena[p].go_left +1;
                parent = species_tree.arena[p].parent;
        }
}


}
ancestor
}

pub fn check_optimisation(transfer: & (String,String), species_tree: &mut ArenaTree<String>,node: usize, inversion:  i32) {
    let inversion = 1;
    let  (start,end) = transfer;
println!("=========================================");
println!("===> CHECK TRANSFER {}->{}",start,end);
println!("=========================================");
// est ce le start est a droite du end?
let s = species_tree.get_index(start.to_string()).expect("[select_transfer] Unable fo find start");
let e = species_tree.get_index(end.to_string()).expect("[select_transfer] Unable fo find start");


    let children = &mut species_tree.arena[node].children;
    if children.len() > 0 {
    let left = children[0];
    let right = children[1];


    let goleft_left =      species_tree.arena[left].go_left as i32;
    let goright_left =     species_tree.arena[left].go_right as i32;

    let score_goleft_left = goleft_left - goright_left;


    let goleft_right =      species_tree.arena[right].go_left as i32;
    let goright_right =     species_tree.arena[right].go_right as i32;

    let score_goleft_right = goleft_right - goright_right;

    let switch  = match inversion {
             1 =>  score_goleft_right > score_goleft_left,
            -1 => score_goleft_left > score_goleft_right,
            _ => panic!("akalzkla"),

    };
    let mut _inversion = inversion;
    // println!("check fixed {}",species_tree.arena[node].name);
    // if species_tree.arena[node].fixed == true {panic!("zzz")};
    // if score_goleft_right > score_goleft_left {
    if switch {
        print!("Switch node {} : ",species_tree.arena[node].name);
        if species_tree.arena[node].fixed  {
            println!(" impossible node is fixed");
        }
        else {
            println!("OK");
        }
    }

    if switch && ( species_tree.arena[node].fixed == false ){
        println!("INterversion at node {} {} ({}>{})",node,species_tree.arena[node].name,score_goleft_right,score_goleft_left);
        species_tree.arena[node].children[0] = right;
        species_tree.arena[node].children[1] = left;
        // species_tree.arena[node].fixed = true;
        // let mut parent = species_tree.arena[node].parent;
        // while parent != None {
        //     let p = match parent {
        //         Some(p) => p ,
        //         None => panic!("unexpected"),
        //     };
        //     species_tree.arena[p].fixed = true;
        //     parent = species_tree.arena[p].parent;
        //     if p == node {
        //          break;
        //      }
        //
        // };


        _inversion = match inversion {
            1 => -1,
            -1 => 1,
            _ => panic!("akalzkla"),
        };


    }
    if (node != e ) && (node != s) {
    // if (0 == 0) || ( species_tree.arena[right].go_left > 0 ) || ( species_tree.arena[left].go_left > 0 ) ||  ( species_tree.arena[right].go_right > 0 ) || ( species_tree.arena[left].go_right  > 0 ){
    println!("FIXING NODE ORIENTATON {}",species_tree.arena[node].name);
    species_tree.arena[node].fixed = true;}

// species_tree.arena[node].fixed = true;
    // println!("FIXING NODE ORIENTATON {}",species_tree.arena[node].name);
    // species_tree.arena[node].fixed = true;
    // let mut parent = species_tree.arena[node].parent;
    // while parent != None {
    //     let p = match parent {
    //         Some(p) => p ,
    //         None => panic!("unexpected"),
    //     };
    //     species_tree.arena[p].fixed = true;
    //     parent = species_tree.arena[p].parent;
    //     if p == node {
    //          break;
    //      }
    //
    // };
    species_tree.arena[node].go_right = 0;
    species_tree.arena[node].go_left = 0;
    let inversion = _inversion;
    if (left != e ) && (left != s) {
    check_optimisation(transfer,species_tree,left,inversion);
    }
    if (right != e ) && (right != s) {
    check_optimisation(transfer,species_tree,right,inversion);
    }
}
}
pub fn classify_transfer(transfer: & (String,String),species_tree: &mut ArenaTree<String>, index: & usize)  {
    let  (start,end) = transfer;
println!("=========================================");
println!("===> CLASSIGY TRANSFER {}->{}",start,end);
println!("=========================================");
// est ce le start est a droite du end?
let s = species_tree.get_index(start.to_string()).expect("[select_transfer] Unable fo find start");
let e = species_tree.get_index(end.to_string()).expect("[select_transfer] Unable fo find start");
let ancestor = lca(species_tree,s,e);
let mut parent = species_tree.arena[s].parent;
while (parent != Some(ancestor)) && (parent != None) {
    let p = match parent {
        Some(p) => p,
        None => panic!("unexpected"),
    };
    species_tree.arena[p].transfers.push(*index);
    parent = species_tree.arena[p].parent;
};

let mut parent = species_tree.arena[e].parent;
while (parent != Some(ancestor)) && (parent != None) {
    let p = match parent {
        Some(p) => p,
        None => panic!("unexpected"),
    };
    species_tree.arena[p].transfers.push(*index);
    parent = species_tree.arena[p].parent;
};
}

pub fn reorder_transfers(species_tree: &mut ArenaTree<String>, node:  usize, ordered: &mut Vec<usize> ) {
    let tr = &species_tree.arena[node].transfers;
    println!("REORDER_TRANSFERS => {:?}",tr);
    for t in tr {
        if !ordered.contains(&t) {
            ordered.push(*t);
        }


    }
    // if tr.len() == 1 {
    //     println!("REORDER_TRANSFERS => One trabsfer");
    //     ordered.push(tr[0]);
    // }
    let children = &mut species_tree.arena[node].children;
    if children.len() > 0 {
    let left = children[0];
    let right = children[1];
    reorder_transfers(species_tree,left,ordered);
    reorder_transfers(species_tree,right,ordered);

}
    // let t = species_tree.arena[p].transfers
}

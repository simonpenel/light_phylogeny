use crate::arena::ArenaTree;
use crate::arena::Event;

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
            // println!("[map_transfer] {:?} -> {:?} ==> {:?} -> {:?}",start,end,map_start,map_end);
        }
        map_transfers
}

pub fn get_gtransfer(gene_tree: &mut ArenaTree<String>) -> Vec<(String,String)> {
    let mut  transfers = vec![];
    for index in  &gene_tree.arena {
        if index.is_a_transfert {
            let parent = index.parent;
            let parent = match parent {
                Some(p) => p,
                None => panic!("[get_gtransfert] the node has no parent"),
            };
            // let transfer = (&index,parent);
            let end = index.location.to_string();
            let start = gene_tree.arena[parent].location.to_string();
            transfers.push((start,end));
        };
    };
    transfers
}

pub fn map_parasite_g2s(para_as_species: &mut ArenaTree<String>,para_as_gene: &mut ArenaTree<String>,) {
    for index in  &mut para_as_species.arena {
        // println!("Mapping {:?}",index.name);
        let name = &index.name;
        let i = para_as_gene.get_index(name.to_string());
        match i {
            Ok(i) => {
                let e = &para_as_gene.arena[i].e;
                println!("Mapping of {} OK, event is {:?} (transfert: {})",name,e,&para_as_gene.arena[i].is_a_transfert);
                // index.e=Event::Duplication;
                // index.e = match &para_as_gene.arena[i].e{
                index.is_a_transfert = para_as_gene.arena[i].is_a_transfert;
                index.e = match  e {
                    &Event::Duplication => Event::Duplication,
                    &Event::BranchingOut => Event::BranchingOut,
                    &Event::Speciation => Event::Speciation,
                    &Event::Loss => Event::Loss,
                    &Event::Leaf => Event::Leaf,
                    _ => {println!("Event {:?} not selected",e);
                         Event::Undef},
                };

            },
            Err(_err) => {
                println!("Unable to map {}",name);
            },
        }
    }
}

pub fn map_parasite_s2g(para_as_species: &mut ArenaTree<String>,
    para_as_gene: &mut ArenaTree<String>,
    gene_trees : &mut std::vec::Vec<ArenaTree<String>>) {
    for index in &para_as_gene.arena {
        let name = &index.name;
        let i = para_as_species.get_index(name.to_string());
        match i {
            Ok(_i) => {
                println!("[map_parasite_s2g] Mapping of {} OK",name);
            },
            Err(_err) => {
                println!("[map_parasite_s2g] Unable to map {} {:?}",name,&index.e);
                let p = &index.parent;
                let p = match p {
                    Some(p) => p,
                    None => panic!("[map_parasite_s2g] Error: node as not parent"),
                };
                // p is the parent of the node in the rec species not found in the pipe species
                let parent_name = para_as_gene.arena[*p].name.clone();
                println!("[map_parasite_s2g]  => parent of the node in the reconciled species tree is {}({})",
                    p,parent_name);
                let j = para_as_species.get_index(parent_name.to_string());
                let j = match j {
                    Ok(j) => j,
                    Err(_e) => panic!("Unable to find parent in 'species' tree"),
                };
                // j is the mapping of p  in the spec tree
                println!("[map_parasite_s2g]  => Mapping of parent {} OK",parent_name);
                match index.e {
                    // the missing node is a loss, I add
                    Event::Loss => {
                        println!("[map_parasite_s2g]  => missing node is a Loss, I add it to parent");
                        //  New pipe species node with the name find in the rec species tree
                        let new_loss = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_loss].name = name.to_string();
                        // Add the new pipe species node to j
                        para_as_species.arena[new_loss].parent = Some(j);
                        para_as_species.arena[j].children.push(new_loss);
                    },
                    _ => {
                        println!("[map_parasite_s2g]  => missing node is a not Loss, I insert it between  parent and chidren");
                        //  New pipe species node with the name find in the rec species tree
                        let new_node = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_node].name = name.to_string();
                        // Children of j
                        let children = &para_as_species.arena[j].children;
                        // Insert the new node between j and its children
                        // I suppose there is 2 nodes, even it mayx exist a third one which is
                        //  the loss node previously added
                        let left = children[0];
                        let right = children[1];
                        para_as_species.arena[left].parent = Some(new_node);
                        para_as_species.arena[right].parent = Some(new_node);
                        para_as_species.arena[new_node].children.push(left);
                        para_as_species.arena[new_node].children.push(right);
                        para_as_species.arena[j].children.retain(|&x| x !=  left);
                        para_as_species.arena[j].children.retain(|&x| x !=  right);
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
                            println!("[map_parasite_s2g] Processing gene tree number {}",*ng);
                            println!("[map_parasite_s2g] Redefine location of the node.");
                            println!("[map_parasite_s2g] Previous {:?}:", &gene_trees[*ng].arena[*nn]);
                            gene_trees[*ng].arena[*nn].location = para_as_species.arena[new_node].name.to_string();
                            println!("[map_parasite_s2g] New {:?}:", &gene_trees[*ng].arena[*nn]);
                            // In case the node is not a root (I expect index 0 is root, maybe this
                            // is not perfect) we need to add a gene node to map with the pipe species
                            if nn > &0 {
                                println!("[map_parasite_s2g] This not a gene root: adding a new gene.");
                                // parent du noeud traité
                                let p = gene_trees[*ng].arena[*nn].parent;
                                let parent =  match p {
                                    Some(p) => p,
                                    None => panic!("[map_parasite_s2g] Unable to find parent"),
                                };
                                // nouveau noeud 1
                                let new_svgnode = gene_trees[*ng].new_node("virtual1_svg".to_string());
                                gene_trees[*ng].arena[new_svgnode].location = index.name.clone();
                                // nouveau noeud 2
                                let new_svgnode_bis = gene_trees[*ng].new_node("virtual2_svg".to_string());
                                gene_trees[*ng].arena[new_svgnode_bis].location = index.name.clone();
                                // para_as_species.arena[j].nbg = 2;
                                // debug.push((index.idx,*ng,(new_svgnode_bis,*nn)));
                                add_gnodes.push((j,*ng,(new_svgnode_bis,*nn)));
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
                            }
                            // &gene_trees[ng].arena[nn].nodes = vecbug;
                            // println!("DEBUG {:?}", &gene_trees[*ng].arena[*nn]);
                        // }
                        }
                        for (node,ng,(node1,node2))  in add_gnodes {
                            println!("DEBUG ADD Gene number {}, node {} ({},{})",ng,node,node1,node2);
                            // para_as_species.arena[node].nbg = 2;
                            // para_as_species.arena[node].nodes = [(ng,node1),(ng,node2)].to_vec();
                        }
                        // let  (ng,nn)  = vecbug[2];





                    },
                };


            },
        }
    }
}

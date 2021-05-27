/// name = "light_phylogeny"
/// version = "0.20.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"
use std::process;
use log::{info};
pub const BLOCK: f32 = 60.0;
pub const PIPEBLOCK: f32 = BLOCK / 4.0;

// Structures
// ==========

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    /// index in the Arena structure
    pub idx: usize,             // index dans la structure
    val: T,                     // valeur unique dans la structure
    /// name of the node
    pub name: String,           // nom du noeud ou de la feuille
    /// support of the node
    pub support: String,           // support du noeud
    /// parent if the node
    pub parent: Option<usize>,  // index du parent
    /// children of the node
    pub children: Vec<usize>,   // indexes des enfants
    /// x coordinate
    pub x: f32,                 // coordonnee x (avant rotation 90 svg)
    /// x shift
    pub xmod: f32,              // decalage x a ajouter a x
    /// y coordinate
    pub y: f32,                 // coordonnee y (avant rotation 90 svg)
    /// y shift
    pub ymod: f32,              // decalage y a ajouter a y (pour les arbres reconcilies)
    /// real branch length
    pub l: f32,                 // longueur de branche lue dans le fichier
    /// event (duplication, speciation, loss, transfer ...)
    pub e: Event,               // evenement (dans le cas d'arbre de gene) Duplication, Loss, etc.
    /// location of the gene tree node in the species tree (recPhyloXML)
    pub location: String,       // SpeciesLocaton associe evenement (dans le cas d'arbre de gene)
    /// pipe width (recPhyloXML)
    pub width: f32,             // largeur du tuyeau (dans le cas d'arbre d'espece)
    /// pipe height (recPhyloXML)
    pub height: f32,            // hauteur du tuyeau (dans le cas d'arbre d'espece)
    /// number of gene nodes associated to the species node (recPhyloXML)
    pub nbg: usize,             // nombre de noeud  d'arbre de genes associcés à ce noeud
                                // (dans le cas d'arbre d'espece)
    /// number of gene nodes associated to the species node (recPhyloXML)
    pub nodes: Vec<(usize,usize)>,      // gene nodes associes (dans le cas d'arbre d'espece)
    /// the node come from a transfer (it is a transferBack and its parent is a BranchingOut)
    pub is_a_transfert: bool,   // the node come from a tarnsfert, i.e. he is a transferBack and
                                // his parent is a BranchingOut . Since more than 1 event is
                                // associated to the node in xml (as transferBack+leaf)
                                // and only one is  associated to the node in the structure
                                // (here leaf)
    /// optimisation: number of left-side orientation minimising transfer crossings (recPhyloXML)
    pub go_left: usize,                            // this is useful for drawing the transfer.
    /// optimisation: number of right-side orientation minimising transfer crossings (recPhyloXML)
    pub go_right: usize,
    /// optimisation: node is fixed, left/side orientation of children can not be modified (recPhyloXML)
    pub fixed: bool,
    /// optimisation: indexes of transfers associated to the node (recPhyloXML)
    pub transfers: Vec<usize>,      // transfer associes (dans le cas d'arbre d'espece)
    /// will be displayed
    pub visible: bool,

}
impl<T> Noeud<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            name: String::from("Undefined"),
            support: String::from("Undefined"),
            parent: None,
            children: vec![],
            x: 0.0,
            xmod: 0.0,
            y: 0.0,
            ymod: 0.0,
            l: 0.0,
            e: Event::Undef,
            location: String::from("Undefined"),
            width: PIPEBLOCK ,
            height: PIPEBLOCK ,
            nbg: 0,
            nodes: vec![],
            is_a_transfert:false,
            go_left: 0,
            go_right: 0,
            fixed:false,
            transfers: vec![],
            visible:true,
        }
    }
    /// Set node event
    pub fn set_event(&mut self, e: Event)
    {
        self.e = e;
    }
    /// Set node x
    pub fn set_x_noref(&mut self, x: f32)
    {
        self.x = x;
    }
    /// Set node xmod
    pub fn set_xmod_noref(&mut self, xmod: f32)
    {
        self.xmod = xmod;
    }
    /// Set node ymod
    pub fn set_ymod_noref(&mut self, ymod: f32)
    {
        self.ymod = ymod;
    }
    /// Set node y
    pub fn set_y_noref(&mut self, y: f32)
    {
        self.y = y;
    }
}
/// Structure ArenaTree.
///
/// Taken from
/// <https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6>
///
/// Ben Lovy <https://deciduously.com/>
#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq
{
    pub arena: Vec<Noeud<T>>,
}
impl<T> ArenaTree<T>
where
    T: PartialEq
{
    #[allow(dead_code)]
    /// Add a node and send its new index. If the
    /// node already exists, send its index.
    pub fn node(&mut self, val: T) -> usize {
        // first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Noeud::new(idx, val));
        idx
    }
    /// Add a node and send its new index. If the
    /// node already exists, send a panic alert.
    pub fn new_node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                    panic!("Node already exists.");
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Noeud::new(idx, val));
        // Ok(idx)
        idx
    }
    ///  Get index of a node from its name
    pub fn get_index(&mut self, name: String) -> Result <usize, usize> {
        for node in &self.arena {
            if node.name == name {
                return Ok(node.idx)
             }
            }
        println!("Note: Unable to find {} in the tree.",name);
        Err(0)
    }
    /// Get the index of the root.
    pub fn get_root(&mut self) -> usize {
        for node in &self.arena {
            if node.parent == None {
                return node.idx
             }

            }
        panic!("Unable to get root of the tree.");
    }
    /// Check if the node is a leaf.
    pub fn is_leaf(&self, idx: usize) -> bool {
        match self.arena[idx].children.len() {
        0 => true,
        _ => false,
        }
    }
    /// Check if the node is a left node.
    pub fn is_left(&self, idx: usize) -> bool {
        match self.arena[idx].parent {
        Some(p) => {
            let children = &self.arena[p].children;
            idx == children[0]
        },
        None => false,
        }
    }
    /// Check if the node is the root.
    #[allow(dead_code)]
    pub fn is_root(&self, idx: usize) -> bool {
        match self.arena[idx].parent {
        Some(_p) => false,
        None => true,
        }
    }
    /// Get the depth of the node in the tree.
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    /// Get the largest x value of a tree.
    pub fn get_largest_x(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.x + node.width / 2.0  > max {
                max = node.x + node.width / 2.0 ;
                }
            }
        max
    }
    /// Get the largest y value of a tree.
    pub fn get_largest_y(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.y  + node.height / 2.0  > max {
                max = node.y  + node.height / 2.0  ;
             }
            }
        max
    }
    /// Get the smallest x value of a tree.
    pub fn get_smallest_x(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.x - node.width / 2.0  < min {
                min = node.x  - node.width / 2.0 ;
             }
            }
        min
    }
    /// Get the smallest y value of a tree.
    pub fn get_smallest_y(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.y   - node.height / 2.0  < min {
                min = node.y  - node.height / 2.0;
             }
            }
        min
    }
}

///  Structure Options: drawing options.
#[derive(Debug)]
pub struct Options{
    /// display internal gene nodes
    pub gene_internal: bool,
    /// display internal species nodes (recPhyloXML)
    pub species_internal: bool,
    /// display a cladogramme
    pub clado_flag: bool,
    /// only draw the species tree (recPhyloXML)
    pub species_only_flag: bool,
    /// use the real branch length
    pub real_length_flag: bool,
    /// open the svg in the browser
    pub open_browser: bool,
    /// verbose mode
    pub verbose: bool,
    /// only draw gene tree number # (recPhyloXML)
    pub disp_gene: usize,
    /// scale to be applied to real branch length
    pub scale: f32,
    /// ratio between  species pipe tree width and cumulated gene trees width (recPhyloXML)
    pub ratio: f32,
    /// rotate the svg 90 counter clockwise
    pub rotate: bool,
    /// not yet implemented
    pub remove: bool,
    /// draw only one transfer to represent redundant transfers (recPhyloXML)
    pub thickness_flag: bool,
    /// abundance threshold for displaying redundant transfers (recPhyloXML)
    pub thickness_thresh: usize,
    /// number of the gene tree to display when displaying redundant transfers as one (recPhyloXML)
    pub thickness_gene: usize,
    /// display the abundance of the redundant transfers (recPhyloXML)
    pub thickness_disp_score:bool,
    /// optimise species branches left/right orientation in order to minimise transfer crossings (recPhyloXML, under development)
    pub optimisation:bool,
    /// scale to be applied to the heigth of the tree
    pub height:f32,
    /// width to be applied to the heigth of the tree
    pub width:f32,
    /// display support
    pub support:bool,
}
impl Options {
    pub fn new() -> Self {
        Self {
            gene_internal:false,
            species_internal:false,
            clado_flag:true,
            species_only_flag:false,
            real_length_flag:false,
            open_browser:false,
            verbose:false,
            disp_gene:0,
            scale:1.0,
            ratio:1.0,
            rotate:true,
            remove:false,
            thickness_flag:false,
            thickness_thresh:0,
            thickness_gene:0,
            thickness_disp_score:false,
            optimisation:false,
            height:1.0,
            width:1.0,
            support:false,

        }
    }
}

/// Structure Config: drawing config.
#[derive(Debug)]
pub struct Config{
    pub species_color: String,
    pub species_opacity: String,
    pub single_gene_color: String,
    pub gene_opacity: String,
    pub species_police_color: String,
    pub species_police_size: String,
    pub gene_police_size: String,
    pub bezier: String,
}
impl Config {
    pub fn new() -> Self {
        Self {
            species_color:"pink".to_string(),
            species_opacity:"0.9".to_string(),
            single_gene_color:"blue".to_string(),
            gene_opacity:"0.9".to_string(),
            species_police_color:"orange".to_string(),
            species_police_size:"12".to_string(),
            gene_police_size:"10".to_string(),
            bezier:"1".to_string(),
        }
    }
}

// Enums
// =====

/// enum of the possible events in a gene tree
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    BranchingOut,
    TransferBack,
    BifurcationOut,
    Leaf,
    ObsoleteSpeciationLoss,
    Undef,
}
/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}


// Fonctions
// =========

///  Fill an ArenaTree structure with the contents of a parentheses  tree.
pub fn newick2tree(arbre:String, tree : &mut ArenaTree<String>, index:usize, num: &mut usize) {
    info!("[newick2tree] Tree = {}",arbre);
    let arbre = arbre;
    check_is_rooted(&arbre);
    let (left,right,trail) = find_left_right(arbre);
    info!("[newick2tree] Left = {} Right = {} Trail = {}",left,right,trail);
    match trail.find(':') {
        Some(j) => {
            tree.arena[index].l = trail[j+1..].to_string().parse::<f32>().unwrap();
            tree.arena[index].name = trail[0..j].to_string();
        },
        None => {tree.arena[index].name = trail[..].to_string();},
    };

    match  left.find(',') {
        Some(_i)=> {
            *num = *num + 1;
            //  Nouveau noeud
            let name = "NODE_".to_string()+&num.to_string();
            let new_index = tree.new_node(name.to_string());
            // tree.arena[new_index].name = int_name.to_string();
            tree.arena[index].children.push(new_index);
            tree.arena[new_index].parent = Some(index);
            newick2tree(left, tree,new_index,num);
        },
        None => {
            info!("[newick2tree] {} is a leaf",left);
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let left_index = tree.new_node(name);
            tree.arena[index].children.push(left_index);
            tree.arena[left_index].parent = Some(index);
            // Suppression des balise NHX (tres sale)
            // TODO a ameliorer
            let left =  match left.find("[&&"){
                Some(k) =>  left[..k].to_string(),
                None => left[..].to_string(),
            };
            match  left.find(':') {
                Some(i)=> {
                    tree.arena[left_index].name = left[0..i].to_string();
                    tree.arena[left_index].l = left[i+1..].to_string().parse::<f32>().unwrap();
                },
                None => {
                    tree.arena[left_index].name = left;
                },
            }
        },
    };
    match  right.find(',') {
        Some(_i)=> {
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let new_index = tree.new_node(name.to_string());
            tree.arena[index].children.push(new_index);
            tree.arena[new_index].parent = Some(index);
            newick2tree(right, tree,new_index,num);
        },
        None => {
            info!("[newick2tree] {} is a leaf",right);
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let right_index = tree.new_node(name);
            tree.arena[index].children.push(right_index);
            tree.arena[right_index].parent = Some(index);
            // Suppression des balise NHX (tres sale)
            // TODO a ameliorer
            let right =  match right.find("[&&"){
                Some(k) =>  right[..k].to_string(),
                None => right[..].to_string(),
            };
            match  right.find(':') {
                Some(i)=> {
                    tree.arena[right_index].name = right[0..i].to_string();
                    tree.arena[right_index].l = right[i+1..].to_string().parse::<f32>().unwrap();
                },
                None => {
                    tree.arena[right_index].name = right;
                },
            }
        },
    };
}

/// Check if tree in newick format is rooted
pub fn check_is_rooted(arbre: &String) {
let p = arbre.matches('(').count();
let c = arbre.matches(',').count();
if p == c {
    info!("Tree is rooted.");
}
else
if p + 1 == c  {
    println!("Tree is unrooted.");
    println!("Please use a rooted tree.");
    process::exit(0);
}
else {
    println!("Unable to determine if tree is rooted, file is incorrect.");
    process::exit(1);
}
}
// Split a parenthesed tree into  left and right  parenthsed trees and trailing  string
pub fn find_left_right(arbre:String)-> (String,String,String){
    let mut len = arbre.len() - 1;
    if &arbre[len..] == "\n" {
        len -= 1;
    }
    assert_eq!(&arbre[..1],"(");
    let mut num_par = 0;
    let mut i = 0;
    for char in arbre.chars() {
        i += 1;
         match char {
             '(' => num_par += 1,
             ')' => num_par -= 1,
             ',' => {
                 if num_par == 1 {
                     break
                 }
             },
             _ => {}
         };
    }
    let left = (&arbre[1..i-1]).to_string();
    let right = (&arbre[i..len+1]).to_string();
    let trail =  match right.rfind(')'){
        Some(k) =>  right[k+1..].to_string(),
        None => "".to_string(),
    };
    let trail =  match trail.find(';'){
        Some(k) =>  trail[0..k].to_string(),
        None => trail,
    };
    let right =  match right.rfind(')'){
        Some(k) =>  right[..k].to_string(),
        None => right[..].to_string(),
    };
    // Suppression des balise NHX (tres sale)
    // TODO a ameliorer
    let trail =  match trail.find("[&&"){
        Some(k) =>  trail[..k].to_string(),
        None => trail[..].to_string(),
    };

    (left,right,trail)
}

/// Fill an ArenaTree structure with the contents of a roxmltre::Node structure
pub fn xml2tree(node: roxmltree::Node, parent: usize, mut numero : &mut usize,
                mut  tree: &mut ArenaTree<String>) {
    // Note : speciationLoss", "speciationOutLoss", "speciationOut" sont obsolètes
    // (vieux fomat recphyloxml)
    // On essaie de les traiter correctement
    // je cherche les fils
    let children = node.children();
    for child in children {
        if child.has_tag_name("clade"){
            // increment le numero
            *numero += 1;
            // Nouveau nom
            let name = "N".to_owned()+&numero.to_string();
            //  index de ce nouveau nom
            let name = tree.new_node(name.to_string());
            //Ajoute ce noeud au parent
            tree.arena[parent].children.push(name);
            // Attribue un parent a ce noeud
            tree.arena[name].parent = Some(parent);
            let nb_att = child.attributes().len();
            if nb_att >= 1 {
                let mut i = 0;
                while i < nb_att {
                    if child.attributes()[i].name() == "branch_length" {
                        let dist = child.attributes()[i].value().parse::<f32>();
                        match dist {
                            Ok(dist) => tree.arena[name].l = dist,
                            Err(_err)    => panic!("[xml2tree] Unable to read branch length"),
                        };

                        }
                    i = i + 1 ;
                }
            }
            // Explore le reste de l'arbre a partir de ce noeud
            xml2tree(child, name, &mut numero, &mut tree);
        }
        // Attribue le nom defini dans le tag id
        if child.has_tag_name("id"){
            let nom = child.first_child().unwrap().text();
            match nom {
                Some(text) => tree.arena[parent].name = text.to_string(),
                None    => tree.arena[parent].name = "Unkwnown".to_string(),
            };
        }
        // Attribue le nom defini dans le tag name
        if child.has_tag_name("name"){
            let nom = child.first_child().unwrap().text();
            match nom {
                Some(text) => tree.arena[parent].name = text.to_string(),
                None    => tree.arena[parent].name = "Unkwnown".to_string(),
            };
        }
        // Attribue la distance definie dans le tag branch_length
        if child.has_tag_name("branch_length"){
            let dist = child.first_child().unwrap().text();
            match dist {
                Some(text) => tree.arena[parent].l = text.to_string().parse::<f32>().unwrap(),
                None    => panic!("[xml2tree] Unable to read branch length"),
            };
        }
        // Attribue le support defini dans confidence
        if child.has_tag_name("confidence"){
            let support = child.first_child().unwrap().text();
            match support {
                Some(text) => tree.arena[parent].support = text.to_string(),
                None    => panic!("[xml2tree] Unable to read branch support"),
            };
        }
        // traitement events (phyloXML)
        if child.has_tag_name("events"){
            info!("[xml2tree]  phyloXML event detected at {:?}",tree.arena[parent]);
            for evenement in child.children() {
                if evenement.has_tag_name("speciations"){
                    tree.arena[parent].set_event(Event::Speciation);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                };
                if evenement.has_tag_name("duplications"){
                    tree.arena[parent].set_event(Event::Duplication);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                };
                if evenement.has_tag_name("losses"){
                    tree.arena[parent].set_event(Event::Loss);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                };


            }
        }
        // traitement eventsRec (recPhyloXML)
        if child.has_tag_name("eventsRec"){
            info!("[xml2tree] recPhyloXML event detected at {:?}",tree.arena[parent]);
            let mut event_num = 0; // Le nb d'evenements dans balise eventsRec
            let mut sploss_num = 0; // pour le format obsolete
            let current_sploss_name = parent;
            for evenement in child.children() {
                if evenement.has_tag_name("speciationLoss"){
                    // speciationLoss is obsolete and need a special processing: adding 2 nodes,
                    // one of them  being a loss
                    info!("[xml2tree] Find obsolete tag speciationLoss (# {})",sploss_num);
                    info!("[xml2tree] Index of current father (current_sploss_name)  {}",
                     current_sploss_name);
                    info!("[xml2tree] Initial tree {:?}",tree);
                    sploss_num += 1;
                    // Incremente le numero
                    *numero += 1;
                    // Nouveau nom
                    let sploss_name = "ADDED_SPECLOSS".to_owned()+&numero.to_string();
                    info!("[xml2tree] Create new node sploss {}",sploss_name);
                    //  index de ce nouveau nom
                    let sploss_name = tree.new_node(sploss_name.to_string());
                    info!("[xml2tree] Modified tree 1  {:?}",tree);
                    info!("[xml2tree] Index of current node {}",parent);
                    // Je veux que le noeud courant soit le fils du ouveau noeud, et que celui ci
                    // prenne la place du noeud courant.
                    // parent du noeud courant:
                    let grandparent =
                    match tree.arena[parent].parent {
                        Some(p) => p,
                        None => panic!("No parent! Unable to process obsolete format"),
                    };
                    // Attribution d'un nom et d'un event
                    tree.arena[sploss_name].set_event(Event::ObsoleteSpeciationLoss);
                    tree.arena[sploss_name].name = "SpecOutLoss".to_string();
                    // Attribution d'une location
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] Location is {}",location);
                    info!("[xml2tree] Set Location of sploss_name:");
                    tree.arena[sploss_name].location = location.to_string();
                    // Ajout du nouveau noeud comme frere du noeud courant
                    tree.arena[sploss_name].parent = Some(grandparent);
                    // Vire le noeud courant des fils de son pere
                    tree.arena[grandparent].children.retain(|&x| x !=  parent);
                    // Ajoute le nouveau noeud  au pere
                    tree.arena[grandparent].children.push(sploss_name);
                    // Ajoute le noeud  courant au nouveau noeud
                    tree.arena[sploss_name].children.push(parent);
                    //  Redfinit le parent du noeud courant
                    tree.arena[parent].parent = Some(sploss_name);
                    info!("[xml2tree] Modified tree 2  {:?}",tree);
                    // Nouveau nom Loss
                    let loss_name = "ADDED_LOSS".to_owned()+&numero.to_string();
                    info!("[xml2tree] Create new node loss {}",loss_name);
                    //  index de ce nouveau nom
                    let loss_name = tree.new_node(loss_name.to_string());
                    info!("[xml2tree] New node loss : {:?}",tree.arena[loss_name]);
                    //  Ajoute le Loss au nouveau noeud
                    tree.arena[sploss_name].children.push(loss_name);
                    tree.arena[loss_name].parent = Some(sploss_name);
                    // Attribution d'un nom et d'un event
                    tree.arena[loss_name].set_event(Event::Loss);
                    tree.arena[loss_name].name = "Added Loss".to_string();
                }
                if evenement.has_tag_name("speciationOutLoss"){
                    panic!("Warning: taxon 'speciationOutLoss' is obsolete");
                }
                if evenement.has_tag_name("speciationOut"){
                    panic!("Warning: taxon 'speciationOut' is obsolete");
                }
                if evenement.has_tag_name("loss"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::Loss);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    tree.arena[parent].location = location.to_string();
                }
                if evenement.has_tag_name("leaf"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    // TODO
                    // C'est une feuille mais pas forcement  le premier evenement:
                    // <eventsRec>
                    //   <leaf speciesLocation="5"></leaf>
                    // </eventsRec>
                    //  mais dans les autres cas
                    // <eventsRec>
                    //   <transferBack destinationSpecies="4"></transferBack>
                    //   <leaf speciesLocation="4"></leaf>
                    // </eventsRec>
                    //  On va ecraser l'info  transferBack, mais celle-ci a ete stockée dans
                    //  le champs is_a_transfert
                    tree.arena[parent].set_event(Event::Leaf);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    let nb_att = evenement.attributes().len();
                    info!("[xml2tree] Number of attributes  = {}",nb_att);
                    assert!(evenement.has_attribute("speciesLocation"));
                    if nb_att == 1 {
                        assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                        let location = evenement.attributes()[0].value();
                        tree.arena[parent].location = location.to_string();
                    }
                    else {
                    // TODO tres sale verifier : on a les deux possibilites:
                    // <leaf speciesLocation="Lentisphaerae" geneName="Lentisphaerae"></leaf>
                    // <leaf geneName="a2_a" speciesLocation="a"></leaf>
                        assert!(nb_att == 2);
                        assert!(evenement.has_attribute("geneName"));
                        assert!(evenement.has_attribute("speciesLocation"));
                        if evenement.attributes()[0].name() == "geneName" {
                            assert_eq!(evenement.attributes()[1].name(),"speciesLocation");
                            let name = evenement.attributes()[0].value();
                            tree.arena[parent].name = name.to_string();
                            info!("[xml2tree] set name {:?}",tree.arena[parent]);
                            let location = evenement.attributes()[1].value();
                            tree.arena[parent].location = location.to_string();
                        }
                        if evenement.attributes()[1].name() == "geneName" {
                            assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                            let name = evenement.attributes()[1].value();
                            tree.arena[parent].name = name.to_string();
                            info!("[xml2tree] set name {:?}",tree.arena[parent]);
                            let location = evenement.attributes()[0].value();
                            tree.arena[parent].location = location.to_string();
                        }
                    }
                }
                if evenement.has_tag_name("speciation"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::Speciation);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] set location = {}",location);
                    tree.arena[parent].location = location.to_string();
                }
                if evenement.has_tag_name("duplication"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::Duplication);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] set location = {}",location);
                    tree.arena[parent].location = location.to_string();
                }
                if evenement.has_tag_name("branchingOut"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::BranchingOut);
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] set location = {}",location);
                    tree.arena[parent].location = location.to_string();
                }
                // TODO
                // a verifier
                if evenement.has_tag_name("transferBack"){
                    // Ici on plusieurs evenements
                    // Par exemple
                    // <eventsRec>
                    // <transferBack destinationSpecies="5"></transferBack>
                    // <branchingOut speciesLocation="5"></branchingOut>
                    // </eventsRec>
                    // ou
                    // <eventsRec>
                    // <transferBack destinationSpecies="10"></transferBack>
                    // <speciation speciesLocation="10"></speciation>
                    // </eventsRec>
                    // Le destinationSpecies est donc l'emplacement ou doit etre
                    // le noeud représentant l'arivee du transfert
                    // le point de depart du transfer etant le pere de ce noeud
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::TransferBack);
                    // a priori cet event  ne sera pas conserve
                    info!("[xml2tree] setting event of {:?} : {:?}",tree.arena[parent].name,
                     tree.arena[parent].e);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    assert!(evenement.has_attribute("destinationSpecies"));
                    assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] set destinationSpecies = {}",location);
                    tree.arena[parent].location = location.to_string();
                    tree.arena[parent].is_a_transfert = true;
                }
                // TODO
                if evenement.has_tag_name("bifurcationOut"){
                    event_num += 1;
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    tree.arena[parent].set_event(Event::BifurcationOut);
                    info!("[xml2tree] Attributes of {:?} are {:?}",evenement,evenement.attributes());
                    let grandparent =  tree.arena[parent].parent;
                    match grandparent {
                        Some(p)     => {
                            let location =  &tree.arena[p].location;
                            info!("[xml2tree] set location according to its father = {}",
                                  location);
                            tree.arena[parent].location = location.to_string();},
                        None        => panic!("BifurcationOut node as no parent : {:?}",
                                              tree.arena[parent]),
                    };
                    tree.arena[parent].is_a_transfert = true;
                    //  A verifier
                    // Meme espece que son pere
                    // assert!(evenement.has_attribute("destinationSpecies"));
                    // assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                    // let location = evenement.attributes()[0].value();
                    // tree.arena[parent].location = location.to_string();
                }
            }
            info!("[xml2tree]Event closed");
        }
    }
}


// Processig potential obsolete tag from the old recPhyloXML format
pub fn check_for_obsolete( gene_tree:&mut ArenaTree<String>, species_tree:&mut ArenaTree<String>) {
    info!("[check_for_obsolete] Initial gene tree {:?}",&gene_tree);
    let mut osls = Vec::new();
    for  index in &mut gene_tree.arena {
        if index.e == Event::ObsoleteSpeciationLoss {
            info!("[check_for_obsolete] Find ObsoleteSpeciationLoss (OSL): {:?}",index);
            osls.push(index.idx);
        }
    }
    info!("[check_for_obsolete] Find {} OSL in the tree",osls.len());
    if osls.len() > 0 {
        println!("Warning! There was {} obsolete SpeciationLoss tag in the recPhyloXML",osls.len());
    }
    for osl in osls {
        info!("[check_for_obsolete] Processing: {:?}",osl);
        info!("[check_for_obsolete] OSL = {} at species {:?}",gene_tree.arena[osl].name,
         gene_tree.arena[osl].location);
        let children = &gene_tree.arena[osl].children;
        info!("[check_for_obsolete] Number of children: {:?}",children.len());
        assert_eq!(children.len(),2);
        let left = children[0];
        let right = children[1];
        info!("[check_for_obsolete] Left child = {} {:?}",gene_tree.arena[left].name,
         gene_tree.arena[left].e);
        info!("[check_for_obsolete] Right child = {} {:?}",gene_tree.arena[right].name,
         gene_tree.arena[right].e);
        // Le noeud loss dont on veut connaitre l'espece associee
        let loss = match gene_tree.arena[left].e {
            Event::Loss => left,
            _ => right,
        };
        // L'espece du noeud qui n'est pas loss
        let species_not_loss = match gene_tree.arena[left].e {
            Event::Loss => &gene_tree.arena[right].location,
            _ =>  &gene_tree.arena[left].location,
        };
        info!("[check_for_obsolete] Loss node is {:?}", gene_tree.arena[loss]);
        info!("[check_for_obsolete] Species of the other child is {:?}", species_not_loss);
        // Espece de l'OSL
        let species = &gene_tree.arena[osl].location;
        //  Index de l'espece dans l'arbre d'espece
        let species_node = match species_tree.get_index(species.to_string()){
            Ok(index) => index,
            Err(_err) => panic!("[check_for_obsolete] Unable to find node {:?}",species.to_string()),
            };
        let species_node_left =  &species_tree.arena[species_node].children[0];
        let species_node_right =  &species_tree.arena[species_node].children[1];
        let species_left =  &species_tree.arena[*species_node_left].name;
        let species_right =  &species_tree.arena[*species_node_right].name;
        info!("[check_for_obsolete] Species under the OSL in species tree = {}, {}",species_left,
         species_right);
        let loss_species = match species_not_loss ==  species_left {
            true => species_right,
            false => species_left,
        };
        info!("[check_for_obsolete] Thus species of loss node is {}",loss_species);
        gene_tree.arena[loss].location = loss_species.to_string();
    }
    info!("[check_for_obsolete] Final gene tree {:?}",&gene_tree);
}

#[allow(dead_code)]
// Set the coordinates of the gene tree according to species tree coordinates
pub fn map_gene_trees(sp_tree: &mut ArenaTree<String>,
                      gene_trees:&mut std::vec::Vec<ArenaTree<String>>) {
    let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
    info!("[map_gene_trees] {} gene trees to be processed",nb_gntree);
    let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
    loop {
        info!("[map_gene_trees] => Processing Gene Tree {}",idx_rcgen);
        for  index in &mut gene_trees[idx_rcgen].arena {
            let mut mapped = false;
            // println!("MAP node {:?} event {:?} location {:?}",index.idx, index.e,index.location);
            for spindex in  &mut sp_tree.arena {
                if  index.location == spindex.name {
                    mapped = true;
                    let x = spindex.x;
                    index.x = x;
                    let y = spindex.y;
                    index.y = y;
                    info!("[map_gene_trees] [{}] Gene node {:?} mapped to  species node {:?}",idx_rcgen,
                     index,spindex);
                }
            }
            if !mapped {
                panic!("Unable to map Node {:?}",index);
            }
        }
        // Passe à l'arbre de gènes suivant
        idx_rcgen += 1;
        if idx_rcgen == nb_gntree {
            break;
        }
    } //Fin de la boucle sur les arbres de gènes
}

// Determine the number of gene nodes associated to a species node
pub fn map_species_trees(sp_tree: &mut ArenaTree<String>,
                         gene_trees: &mut std::vec::Vec<ArenaTree<String>>) {
    let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
    info!("[map_species_trees] {} gene trees to be processed",nb_gntree);
    let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
    loop {
        info!("[map_species_trees] => Processing Gene Tree {}",idx_rcgen);
        // Boucle sur les noeuds de l'arbre de gene idx_rcgen
        for  index in &mut gene_trees[idx_rcgen].arena {
            let mut mapped = false;
            // Boucle sur les noeuds de l'arbre d'espèce
            for spindex in  &mut sp_tree.arena {
                if  index.location == spindex.name {
                    mapped = true;
                    // Incremente le nb de noeuds de gene associé au noeud d'espece
                    let mut nbg = spindex.nbg;
                    nbg = nbg + 1 ;
                    spindex.nbg = nbg;
                    // Ajoute le tuple (index de l'arbre de  gene, index du noeud de gene) associé
                    spindex.nodes.push((idx_rcgen,index.idx));
                    // spindex.nodes.insert(0,(idx_rcgen,index.idx));
                    info!("[map_species_trees] Gene node {:?} mapped to  species node {:?}",index,spindex);
                }
            }
            if !mapped {
                panic!("Unable to map Node {:?}",index);
            }
        }
        // Passe à l'arbre de gènes suivant
        idx_rcgen += 1;
        if idx_rcgen == nb_gntree {
            break;
        }
    } //Fin de la boucle sur les arbres de gènes
}

// Shift the gene nodes in a given species node to avoid superposition.
pub fn bilan_mappings(sp_tree: &mut ArenaTree<String>,
                      gene_trees: &mut std::vec::Vec<ArenaTree<String>>,
                      index: usize, options: &Options) {
    info!("[bilan_mappings] Species Node {}",sp_tree.arena[index].name);
    let ratio = options.ratio ;   // permet de regler l'ecartement entre les noeuds de genes dans
                                  // l'arbre d'espece
    // shift_left_x is a shift for x for a left node
    // shift_right_x is a shift for x for a right node
    // shift_y is a shift for y for a any node
    let  mut shift_left_x = 0.0;
    let  mut shift_y = 0.0;
    let  mut shift_right_x = sp_tree.arena[index].nbg as f32 -1.0 ;
    let incr = 1.0;
    // TODO classer selon le Y du pere pour eviter les croisement
    // boucle sur m'espeve
    for (index_node, node)  in &sp_tree.arena[index].nodes {
        info!("[bilan_mappings] >>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,
         gene_trees[*index_node].arena[*node].e);
        let bool_left = sp_tree.is_left(index);
        match  gene_trees[*index_node].arena[*node].e {
            Event::Duplication => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                // Do not shift on x  duplicated nodes
                // shift = shift + incr;
                shift_y = shift_y + incr;
                // Do not shift on x  duplicated nodes
                // shift_x = shift_x - incr;
            },
            Event::Speciation => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::TransferBack => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::BranchingOut => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::Leaf => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::Loss => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::BifurcationOut => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            Event::ObsoleteSpeciationLoss => {
                let x = gene_trees[*index_node].arena[*node].x;
                let x = match bool_left {
                    true   => x + PIPEBLOCK*shift_left_x / ratio,
                    false  => x + PIPEBLOCK*shift_right_x / ratio,
                };
                gene_trees[*index_node].arena[*node].set_x_noref(x);
                let y = gene_trees[*index_node].arena[*node].y;
                let y = y + PIPEBLOCK*shift_y / ratio;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                shift_left_x = shift_left_x + incr;
                shift_y = shift_y + incr;
                shift_right_x = shift_right_x - incr;
            },
            _=> {},
        }
    }
    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
         bilan_mappings( sp_tree, gene_trees,son_left, options);
         bilan_mappings( sp_tree, gene_trees,son_right, options);
    }
}

// Shift again the previously shifted gene nodes in case of a duplication node or a leaf
pub fn move_dupli_mappings(sp_tree: &mut ArenaTree<String>,
                           gene_trees: &mut std::vec::Vec<ArenaTree<String>>, index: usize) {
    for (index_node, node) in &sp_tree.arena[index].nodes {
        info!("[move_dupli_mappings] >>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,
         gene_trees[*index_node].arena[*node].e);
        match  gene_trees[*index_node].arena[*node].e {
            Event::Duplication => {
                let bool_left = sp_tree.is_left(index);
                // on aligne sur le fils de gauch ou droite selon si on est dans une branche g ou d
                let dupli_children =  &mut  gene_trees[*index_node].arena[*node].children;
                let dupli_son = match bool_left {
                    true => dupli_children[0],
                    false => dupli_children[1],
                };
                let x = gene_trees[*index_node].arena[dupli_son].x;
                gene_trees[*index_node].arena[*node].set_x_noref(x);
            },
            // Il faut deplacer aussi les feuilles pour compenser: on les mets au meme niveau
            Event::Leaf => {
                let y = sp_tree.arena[index].y + sp_tree.arena[index].height / 2.0 ;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                gene_trees[*index_node].arena[*node].set_ymod_noref(0.0);
            }
            _=> {},
        }
    }
    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        move_dupli_mappings( sp_tree, gene_trees,son_left);
        move_dupli_mappings( sp_tree, gene_trees,son_right);
    }
}

// Center the gene nodes into a  specie snode
pub fn center_gene_nodes(sp_tree: &mut ArenaTree<String>,
                         gene_trees: &mut std::vec::Vec<ArenaTree<String>>, index: usize) {
    let left_sp = sp_tree.arena[index].x - sp_tree.arena[index].width / 2.0  ;
    let right_sp = sp_tree.arena[index].x + sp_tree.arena[index].width / 2.0  ;
    let up_sp = sp_tree.arena[index].y + sp_tree.arena[index].ymod
                - sp_tree.arena[index].height / 2.0  ;
    let down_sp = sp_tree.arena[index].y + sp_tree.arena[index].ymod
                  + sp_tree.arena[index].height / 2.0  ;
    let mut left_gene = -100000000.0;
    let mut right_gene = 100000000.0;
    let mut down_gene = -100000000.0;
    let mut up_gene = 100000000.0;
    for (index_node, node) in &sp_tree.arena[index].nodes {
        info!("[center_gene_nodes] >>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,
         gene_trees[*index_node].arena[*node].e);
        if  gene_trees[*index_node].arena[*node].x    > left_gene {
            left_gene =  gene_trees[*index_node].arena[*node].x  ;
        }
        if gene_trees[*index_node].arena[*node].x    < right_gene {
            right_gene =  gene_trees[*index_node].arena[*node].x  ;
        }

        if  gene_trees[*index_node].arena[*node].ymod > 0.0 {
            panic!("Unexpected ymod value");
        }
        if gene_trees[*index_node].arena[*node].y    > down_gene {
            down_gene =  gene_trees[*index_node].arena[*node].y  ;
        }
        if  gene_trees[*index_node].arena[*node].y    < up_gene {
            up_gene =  gene_trees[*index_node].arena[*node].y  ;
        }
    }
    let middle_sp = (left_sp + right_sp) / 2.0;
    let middle_gn = (left_gene  + right_gene)  / 2.0;
    let shift = middle_gn  - middle_sp;
    let y_middle_sp = (up_sp + down_sp) / 2.0;
    let y_middle_gn = (up_gene  + down_gene)  / 2.0;
    let y_shift = y_middle_gn  - y_middle_sp;
    for (index_node, node) in &sp_tree.arena[index].nodes {
        let x = gene_trees[*index_node].arena[*node].x;
        let x = x - shift ;
        gene_trees[*index_node].arena[*node].set_x_noref(x);
        let y = gene_trees[*index_node].arena[*node].y;
        let y = y - y_shift ;
        gene_trees[*index_node].arena[*node].set_y_noref(y);
    }
    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        center_gene_nodes( sp_tree, gene_trees,son_left);
        center_gene_nodes( sp_tree, gene_trees,son_right);
    }
}

// Set the width of the species tree pipe.
pub fn set_species_width(sp_tree: &mut ArenaTree<String>,
                         gene_trees: &mut std::vec::Vec<ArenaTree<String>>) {
    for spindex in  &mut sp_tree.arena {
        let  nbg = spindex.nbg;
        let nodes = &spindex.nodes;
        let mut nb_duplication_node = 0;
        for (index_node,node) in nodes {
            if gene_trees[*index_node].arena[*node].e == Event::Duplication {
                nb_duplication_node = nb_duplication_node + 1;
            }
        }
        // nb_duplication_node = 0;
        if nbg > 0 {
            spindex.width = ( nbg - nb_duplication_node ) as f32 * PIPEBLOCK;
            spindex.height = ( nbg - 0 ) as f32 * PIPEBLOCK;
        }
        else {
            spindex.width =   PIPEBLOCK;
            spindex.height =  PIPEBLOCK;
        }
    }
}


/// Get the id of the first "spTree" tag.
pub fn find_sptree( doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("spTree"){
            return Ok(node.id())
        }
    }
    Err(0)
}
/// Get the list of ids of all the "spTree" tag in a xml document.
pub fn find_sptrees( doc: &mut roxmltree::Document) -> Result < Vec<roxmltree::NodeId>, usize> {
    let descendants = doc.root().descendants();
    let mut gene_nodes:std::vec::Vec<roxmltree::NodeId> = Vec::new();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("spTree"){
            gene_nodes.push(node.id());
        }
    }
    info!("[find_sptrees] Number of species trees in xml = {}",gene_nodes.len());
    match gene_nodes.len() > 0 {
        true => return Ok(gene_nodes),
        false => Err(0),
    }
}
/// Get the list of ids of all the "regGeneTree" tag in a xml document.
pub fn find_rgtrees( doc: &mut roxmltree::Document) -> Result < Vec<roxmltree::NodeId>, usize> {
    let descendants = doc.root().descendants();
    let mut gene_nodes:std::vec::Vec<roxmltree::NodeId> = Vec::new();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("recGeneTree"){
            gene_nodes.push(node.id());
        }
    }
    info!("[find_rgtrees] Number of gene trees in xml = {}",gene_nodes.len());
    match gene_nodes.len() > 0 {
        true => return Ok(gene_nodes),
        false => Err(0),
    }
}

/// Initial set x and y of nodes :  left son x is 0;  right son x is 1; y is depth
pub fn  knuth_layout(tree: &mut ArenaTree<String>,index: usize,depth: &mut usize){
    tree.arena[index].set_y_noref(BLOCK* (*depth as f32));
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("The tree must be binary.")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        tree.arena[son_left].set_x_noref(0.0);
        tree.arena[son_right].set_x_noref(BLOCK);
        knuth_layout(tree,son_left,&mut(*depth+1));
        knuth_layout(tree,son_right,&mut(*depth+1));
    }
}

/// Transforms the tree into cladogram
pub fn cladogramme( tree: &mut ArenaTree<String>) {
    let root = tree.get_root();
    let mut  max_depth = get_maxdepth(tree,root,&mut 0);
    set_leaves_to_bottom(tree,root,&mut max_depth);
}

///  Transforms the tree into real branch  length representation
pub fn real_length(tree: &mut ArenaTree<String>, index: usize, dist: &mut f32, options: &Options) {
    let  dist_father = tree.arena[index].l;
    let mut dist = *dist + dist_father;
    tree.arena[index].set_y_noref(dist * BLOCK * options.scale + BLOCK);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        real_length( tree, son_left, &mut dist, options);
        real_length( tree, son_right, &mut dist, options);
    }

}

/// Get the depth of the tree
pub fn get_maxdepth( tree: &mut ArenaTree<String>, index:usize, max :&mut usize) -> usize {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        if  tree.depth(son_left) > *max {
            *max =  tree.depth(son_left);
        }
        if  tree.depth(son_right) > *max {
            *max =  tree.depth(son_right);
        }
         get_maxdepth(tree,son_left,max);
         get_maxdepth(tree,son_right,max);
    }
    *max
}

// Set the y values of the leaves of the node index to  max value
pub fn set_leaves_to_bottom( tree: &mut ArenaTree<String>, index: usize, max:&mut  usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        set_leaves_to_bottom(tree,son_left,max);
        set_leaves_to_bottom(tree,son_right,max);
    }
    else {
        match tree.arena[index].e {
            Event::Loss => tree.arena[index].set_y_noref(BLOCK* (*max as f32 )),
            _ => tree.arena[index].set_y_noref(BLOCK* (*max as f32 + 1.0)),
        };
    }
}

// Set the y values of the leaves
pub fn set_leaves_y_values( tree: &mut ArenaTree<String>, index: usize, y:  f32) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        set_leaves_y_values(tree,son_left,y);
        set_leaves_y_values(tree,son_right,y);
    }
    else {
        match tree.arena[index].e {
            Event::Loss => {},
            _ => tree.arena[index].set_y_noref(y),
        };
    }
}


// Shift the  x values  of a node and its children according to the cumulated xmod values
pub fn shift_mod_xy( tree: &mut ArenaTree<String>, index: usize, xmod: &mut f32, ymod: &mut f32) {
    info!("[shift_mod_xy] shifting {:?} xmod={} ymod={}",tree.arena[index],xmod,ymod);
    let x_father = tree.arena[index].x;
    let  xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    let y_father = tree.arena[index].y;
    let  ymod_father = tree.arena[index].ymod;
    let mut ymod = *ymod + ymod_father;
    tree.arena[index].set_y_noref(y_father+ymod);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("The tree must be binary")
    }
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_xy( tree, son_left, &mut xmod, &mut ymod);
        shift_mod_xy( tree, son_right, &mut xmod, &mut ymod);
    }

}

/// Scaling tree height
pub fn scale_heigth( tree: &mut ArenaTree<String>, scale: f32) {
    for spindex in  &mut tree.arena {
        let y = spindex.y;
        spindex.y = y * scale;

    };
}
/// Scaling width height
pub fn scale_width( tree: &mut ArenaTree<String>, scale: f32) {
    for spindex in  &mut tree.arena {
        let x = spindex.x;
        spindex.x = x * scale;

    };
}

#[allow(dead_code)]
// Traverse the tree using post-order traversal starting from a given node
pub fn  explore_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        explore_postorder(tree,left);
        explore_postorder(tree,right);
        println!("POST-ORDER TRAVERSAL : INTERNAL NODE  {:?} / DEPTH = {}",tree.arena[index],
         tree.depth(index));
    }
    else{
        println!("POST-ORDER TRAVERSAL : LEAF           {:?} / DEPTH = {}",tree.arena[index],
         tree.depth(index));
    }
}

// Solve the conflicts between a parent and its children
pub fn  check_vertical_contour_postorder(tree: &mut ArenaTree<String>,index:usize, ymod: f32) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        info!("[check_vertical_contour_postorder] Father = {} (ymod = {} ) , Left = {}, Right = {}",
         tree.arena[index].name,tree.arena[index].ymod,
         tree.arena[left].name,tree.arena[right].name);
        push_down(tree,index, left,right);
        check_vertical_contour_postorder(tree,left,tree.arena[left].ymod + 0.0 *  ymod);
        check_vertical_contour_postorder(tree,right,tree.arena[right].ymod + 0.0 * ymod);
    }
}
// Check for conficts between parents and children and shift down nodes in order to solve
// detected conflicts ( for species pipe tree only).
pub fn push_down (tree: &mut ArenaTree<String>, parent: usize, left: usize, right: usize) {
    let node_parent_down_pos = node_ypos(tree,parent,1);
    let node_left_up_pos = node_ypos(tree,left,-1);
    let node_right_up_pos = node_ypos(tree,right,-1);
    if (node_left_up_pos <=  node_parent_down_pos) ||
       (node_right_up_pos <=  node_parent_down_pos) {
        let shift_left = node_parent_down_pos - node_left_up_pos ;
        let shift_right = node_parent_down_pos - node_right_up_pos ;
        let mut shift_down = match shift_left > shift_right {
            true => shift_left,
            false => shift_right,
        };
        if shift_down <= PIPEBLOCK {
            shift_down = PIPEBLOCK;
        }
        // TODO configurable
        let shift_down = shift_down + 4.0 * PIPEBLOCK;
        info!("[push_down] CONFLIT AT SPEC NODE {}: parent y = {} ymod = {} down = {} left up = {} right up = {} => shift = {}",tree.arena[parent].name,tree.arena[parent].y,tree.arena[parent].ymod,node_parent_down_pos,node_left_up_pos,node_right_up_pos,shift_down);
        info!("[push_down] SHIFTING Y {} + 1xPIPEBLOCK = {}",shift_down,shift_down + 1.0 * PIPEBLOCK);
        info!("Initial left : y = {}, ymod = {}",tree.arena[left].y,tree.arena[left].ymod);
        let y = tree.arena[left].y;
        let y = y + shift_down ;
        tree.arena[left].set_y_noref(y);
        let y = tree.arena[right].y;
        let y = y +shift_down ;
        tree.arena[right].set_y_noref(y);
    }
}

// Solve the conflicts between the left subtree and the right subtree
pub fn  check_contour_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        check_contour_postorder(tree,left);
        check_contour_postorder(tree,right);
        push_right(tree,left,right);
    }
    else{
    }
}
// Get the leftest  or rightest x value of a node
pub fn node_xpos(tree: &mut ArenaTree<String>, index: usize, xmod: f32, operator : i32) -> f32 {
    tree.arena[index].x + tree.arena[index].xmod
    + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK + xmod
}
// Get the upper or lower y value of a node
pub fn node_ypos(tree: &mut ArenaTree<String>, index: usize,  operator : i32) -> f32 {
    tree.arena[index].y + tree.arena[index].ymod
    + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK
}
// Get the left 'contour' of a sub tree
pub fn  get_contour_left(tree: &mut ArenaTree<String>,index:usize,depth:usize,
                         contour_left: &mut Vec<f32>,parent_xmod: f32)  {
    info!("[get_contour_left] >>> {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_left_pos = node_xpos(tree,index,parent_xmod,-1);
    if contour_left.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("Error: negative xmod.");
        }
        contour_left.push(node_left_pos);
        info!("[get_contour_left] increment contour is now {:?}",contour_left);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("Error: negative  xmod.");
    }
    if node_left_pos <= contour_left[local_depth] {
        contour_left[local_depth] = node_left_pos;
        info!("[get_contour_left]: contour is now {:?}",contour_left);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        get_contour_left(tree,left,depth,contour_left,tree.arena[index].xmod + parent_xmod );
    }
}

// Get the right 'contour' of a sub tree
pub fn  get_contour_right(tree: &mut ArenaTree<String>,index:usize,depth:usize,
                          contour_right: &mut Vec<f32>,parent_xmod: f32)  {
    info!("[get_contour_right] process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_right_pos = node_xpos(tree,index,parent_xmod,1);
    if contour_right.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("Error: negative xmod");
        }
        contour_right.push(node_right_pos);
            info!("[get_contour_right] increment contour is now {:?}",contour_right);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("Error: negative xmod");
    }
    if node_right_pos >= contour_right[local_depth] {
        contour_right[local_depth] = node_right_pos ;
            info!("[get_contour_right] contour is now {:?}",contour_right);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let right = children[1];
        get_contour_right(tree,right,depth,contour_right,tree.arena[index].xmod + parent_xmod );
    }
}

// Check for conficts between subtrees and shift conflicting right-hand subtrees to the right
// in order to solve detected  conflicts.
pub fn  push_right(tree: &mut ArenaTree<String>,left_tree:usize,right_tree:usize) -> f32 {
    info!("[push_right] compare right contour of {} and left contour of {}",left_tree, right_tree);
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x
        + tree.arena[left_tree].xmod + tree.arena[left_tree].nbg as f32 *PIPEBLOCK];
    let depth_left_tr  = tree.depth(left_tree);
    get_contour_right(tree,left_tree,depth_left_tr,&mut right_co_of_left_tr,0.0);
    info!("[push_right] right contour of {} = {:?}",left_tree,right_co_of_left_tr);
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x
        + tree.arena[right_tree].xmod - tree.arena[right_tree].nbg as f32 *PIPEBLOCK];
    let depth_right_tr  = tree.depth(right_tree);
    get_contour_left(tree,right_tree,depth_right_tr,&mut left_co_of_right_tr,0.0);
    info!("[push_right] left contour of {} = {:?}",right_tree,left_co_of_right_tr);
    // Si on   a pas le meme longeur de contour on complete le plus petit
    // en remplissant ce qui manque avec la derniere valeur, pour eviter
    // qu'un sous arbre vosin se place sous une feuille
    let right_len = right_co_of_left_tr.len();
    let left_len = left_co_of_right_tr.len();
    if left_len > right_len {
        let last_val =  right_co_of_left_tr[right_len-1];
        let last_vals =  vec![last_val;left_len-right_len];
        right_co_of_left_tr.extend(last_vals.iter().copied());
        info!("[push_right] complete right contour with last value {}", last_val);
    }
    if left_len < right_len {
        let last_val =  left_co_of_right_tr[left_len-1];
        let last_vals =  vec![last_val;right_len - left_len];
        left_co_of_right_tr.extend(last_vals.iter().copied());
        info!("[push_right] complete left contour with last value {}", last_val);
    }
    info!("[push_right] comparing  right cont. of left tree: {:?}",right_co_of_left_tr);
    info!("[push_right] with left cont. of right tree:       {:?} ",left_co_of_right_tr);

    let iter = left_co_of_right_tr.iter().zip(right_co_of_left_tr).map(|(x, y )| (x-y));
    let shift = iter.min_by(|x, y| (*x as i64) .cmp(&(*y as i64 )));
    info!("[push_right] distance max  = {:?}",shift);
    match shift {
        Some(val) => {
            info!("[push_right] distance max  = {:?}",shift);
            if val <= 0.0 {// bidouilel
                info!("[push_right] ================CONFLIT==========");
                info!("[push_right] Modify node {:?}",tree.arena[right_tree]);
                let x_mod =  tree.arena[right_tree].xmod;
                info!("[push_right] initial x_mod = {}",x_mod);
                let x_mod =  x_mod -1.0 *val + BLOCK ;//bidouille
                info!("[push_right] new x_mod = {}",x_mod);
                tree.arena[right_tree].set_xmod_noref(x_mod);
                info!("[push_right] updated node {:?}",tree.arena[right_tree]);
                info!("[push_right] ================CONFLIT==========");
            }
        },
        None => {}
    }
    0.0
}

// Set the x of the father between its children
pub fn  set_middle_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        set_middle_postorder(tree,left);
        set_middle_postorder(tree,right);
        info!("[set_middle_postorder] node {:?}",index);
        let x_left = tree.arena[left].x;
        let x_right = tree.arena[right].x;
        let x = tree.arena[index].x;
        let x_middle = ( x_right + x_left ) / 2.0 ;
        info!("[set_middle_postorder] x father set from {} to {}",x,x_middle);
        tree.arena[index].set_x_noref(x_middle);
        let x_mod =  tree.arena[right].xmod;
        let x_mod =  x_mod + x_middle - x;
        tree.arena[index].set_xmod_noref(x_mod);
    }
}

/// Send the index of the last common ancestor of 2 nodes
#[allow(dead_code)]
pub fn lca(tree : &mut ArenaTree<String>, index1:usize, index2: usize)  -> usize {
    let p1 = tree.arena[index1].parent;
    let p2 = tree.arena[index2].parent;
    let p1 = match p1 {
        Some(p) => p,
        None => return index1,
    };
    let p2 = match p2 {
        Some(p) => p,
        None =>  return index2,
    };
    if p1 == index2 {
        return index2
    }
    if p2 == index1 {
        return index1
    }
    let d1 = tree.depth(index1);
    let d2 = tree.depth(index2);
    info!("[lca] Distance {}: {}",index1,d1);
    info!("[lca] Distance {}: {}",index2,d2);
    if d1 == d2 {
        // lca(tree,p1,p2);
        info!("[lca] Comparison of {:?} and {:?}",tree.arena[index1],tree.arena[index2]);
        match p1 == p2  {
            true => {
                info!("[lca] LCA is {}",p1);
                 return p1
            },
            false => lca(tree,p1,p2),
        }
    }
    else if d1 > d2 {
        lca(tree,p1,index2)
    }
    else {
         lca(tree,index1,p2)
    }
}

#[allow(dead_code)]
pub fn summary_root(tree : &mut ArenaTree<String>, index:usize)  {
    let children  = &tree.arena[index].children;
    match children.len() > 0  {
        true => {
            let left = children[0];
            let right = children[1];
            println!("Node {} ({}) has 2 children: {} ({}) and {} ({})",
            index, &tree.arena[index].name,
            left, &tree.arena[left].name,
            right, &tree.arena[right].name);
            summary_root(tree,left);
            summary_root(tree,right);
        },
        false => {
            println!("Node {} ({}) has no children ",index,&tree.arena[index].name)
        },
    }

}

#[allow(dead_code)]
/// Reset all positions x y xmod ymod of the tree
pub fn reset_pos(tree : &mut ArenaTree<String>)  {
for index in &mut tree.arena {
    index.x = 0.0;
    index.y = 0.0;
    index.xmod = 0.0;
    index.ymod = 0.0;
    };
}

#[allow(dead_code)]
/// Display a short summary of each node of the tree
pub fn summary(tree : &mut ArenaTree<String>)  {
for index in &tree.arena {
    let children  = &index.children;
    let parent = match index.parent {
        Some(p) =>  &tree.arena[p].name,
        None => "None",
    };
    match children.len() > 0  {
        true => {
            print!("Node {} ({}) has {} children:",index.idx,index.name,children.len());
            for child in children {
                print!(" {} ({})", child, &tree.arena[*child].name);
            };
            print!(" and its parent is {}",parent);
            },
        false => {
            print!("Node {} ({}) has no children and its parent is {}",index.idx,index.name,parent)
            },
        }
        println!(" [{:?}]",index.e);
    }
}

#[allow(dead_code)]
/// Add a node to another node
pub fn add_child(tree : &mut ArenaTree<String>, parent:usize, child:usize)  {
    info!("[add_child] adding {} to {}",child,parent);
    tree.arena[child].parent = Some(parent);
    tree.arena[parent].children.push(child);
    info!("[add_child] parent node : {:?}",tree.arena[parent]);

}

#[allow(dead_code)]
/// Move a node from a parent node to another node
pub fn move_child(tree : &mut ArenaTree<String>, child:usize, new_parent:usize)  {
    let parent =  match tree.arena[child].parent {
        Some(p) => p,
        None => panic!("Node {:?} has no parent.",child),
    };
    info!("[move_child] moving {} from {} to {}",child,parent,new_parent);
    tree.arena[child].parent = Some(new_parent);
    tree.arena[new_parent].children.push(child);
    tree.arena[parent].children.retain(|&x| x !=  child);
    info!("[move_child] parent node : {:?}",tree.arena[parent]);
    info!("[move_child] new parent node : {:?}",tree.arena[new_parent]);

}

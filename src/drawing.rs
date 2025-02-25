/// name = "light_phylogeny"
/// version = "1.4.1"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"

// Convention: "pipe" trees are equivalent to "upper" trees, "path" trees are equivalenet to "lower" trees

use log::{info};
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::Config;
use crate::arena::Event;
use crate::arena::BLOCK;
use crate::arena::PIPEBLOCK;
use crate::arena::{get_x_span,get_root_distance};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Circle;
use svg::node::element::Style;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;
use random_color::{Color,RandomColor,Luminosity};


// const STHICKNESS: usize = 12; // Epaisseur trait species
// const SQUARESIZE: f32 = 6.0; // taille carre dupli

// Functions
// =========

/// Draw the svg of a simple tree.
pub fn draw_tree (
    tree: &mut ArenaTree<String>, // tree
    name: String,                 // output file
    options: &Options,            // display options
    config: &Config,              // svg config
    ) {
    info!("[draw_tree] Drawing tree...");
    let width_timeline = 10.0;
    let mut gene_color = config.single_gene_color.clone();
    let largest_x = tree.get_largest_x() * 1.0 + 0.0 ;
    let largest_y = tree.get_largest_y() * 1.0 + 0.0 ;
    let smallest_x = tree.get_smallest_x() * 1.0 + 0.0 ;
    let smallest_y = tree.get_smallest_y() * 1.0 + 0.0 ;
    let width_svg = largest_x - smallest_x + 0.0;
    let width_svg = width_svg * 1.0;
    let longest_name = get_longest_name(tree) as f32;
    let height_svg = largest_y - smallest_y + longest_name *
        config.gene_police_size.parse::<f32>().unwrap();
    let height_svg = height_svg * 1.0;
    let x_viewbox = smallest_x - 0.0 ;
    let y_viewbox = smallest_y - 0.0;
    let  mut document = match options.rotate {
        true => Document::new()
                .set("width", height_svg)
                .set("height", width_svg)
                .set("viewBox", (
                    x_viewbox,
                    y_viewbox,
                    height_svg + 2.0 *BLOCK,
                    width_svg + 2.0 *BLOCK,
                ))
                .set("style","background-color:".to_owned() + &options.bckg_color),
        false => Document::new()
                .set("width", width_svg)
                .set("height", height_svg)
                .set("viewBox", (
                    x_viewbox,
                    y_viewbox,
                    width_svg + 2.0 *BLOCK,
                    height_svg + 2.0 *BLOCK,
                ))
                .set("style","background-color:".to_owned() + &options.bckg_color),
    };
    // Gestion des styles
    // ------------------
    let style = Style::new(".gene { font-size: ".to_owned()
        + &config.gene_police_size.to_string() + "px; fill:"
        + &gene_color.to_string() + "; } .support { font-size: "
        + &config.gene_police_size.to_string() + "px; fill: red; } .branch { font-size: "
        + &config.gene_police_size.to_string() + "px; fill: black; }" );
    document.append(style);
    let mut phylostyle:String = Default::default() ;
    // Cas ou il y a des couleurs definies par l'utilsateur: ajout de styles pour la police
    if options.gene_colors.len() > 0 {
        // On definit le style gene_O qu'on associe a la premiere couleur
        let font_color = options.gene_colors[0].clone();
        let added_style = " .gene_0".to_owned()
            + " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
            + &font_color.to_string() + "; } ";
            // Je passe en str pour l'ajouter
            let add_style_str :&str = &added_style;
            phylostyle.push_str(add_style_str);
    }
    // Cas ou il y a des noeuds a colorer: ajout de styles pour la police
    if options.node_colors.len() > 0 {
        // On definit la couleur des font pour le style node_0
        // Style de la font pour la partie de l'arbre non selectionnee
        let mut font_color  = get_default_color(0);
        // Cas ou on utilise les couleurs definies par l'utilisateur
        if options.gene_colors.len() > 0 {
            font_color = options.gene_colors[0].clone();
        }
        let added_style = " .node_0".to_owned()
            + " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
            + &font_color.to_string() + "; } ";
        // Je passe en str pour l'ajouter
        let add_style_str :&str = &added_style;
        phylostyle.push_str(add_style_str);
        let mut node_color_idx = 1; // Les autres restent a 0
        // Traite tous les noeuds a colorer
        for node_color in  &options.node_colors {
            // Recupere le noeud dans l'arbre
            let node_colored = tree.get_index(node_color.to_string());
            match node_colored {
                Ok(n) => {
                    // On definit la couleur des font pour le style
                    // Style de la font pour la partie de l'arbre
                    let mut font_color  = get_default_color(node_color_idx);
                    // Cas ou on utilise les couleurs definies par l'utilisateur
                    if options.gene_colors.len() > 0 {
                        let _idx_user_color = node_color_idx % options.gene_colors.len();
                        font_color = options.gene_colors[_idx_user_color].clone();
                    }
                    let added_style = " .node_".to_owned() + &node_color_idx.to_string()
                        + " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
                        + &font_color.to_string() + "; } ";
                        // Je passe en str pour l'ajouter
                        let add_style_str :&str = &added_style;
                        phylostyle.push_str(add_style_str);
                        // Modifie la valeur de l'index de couleur dans tous les fils du noeud
                        set_color_index(tree,n,node_color_idx);
                },
                Err(_e) =>{
                        println!("Warning Node {} not found in the gene tree",node_color)
                },
            }
            node_color_idx += 1;
        }
    }
    // Ajout du style
    let style = Style::new(phylostyle);
    document.append(style);

    // Construction du svg
    // -------------------
    // Cas ou l'utilisateur a defini des couleur. Le gene unique a la premiere couleur
    if options.gene_colors.len() > 0 {
        gene_color = options.gene_colors[0].clone();
    }
    // Creation d'un element
    let mut g = Element::new("g");
    // Affiche les timelines (au debut pour eviter un eventuel recouvrement
    // des nom de noeuds internes)
    display_timelines(
        tree,   // species tree
        options,   // drawing options
        &mut g,
        width_timeline
    );
    // Exploration de l'arbre
    for  index in &tree.arena {
        // Cas ou il y a une coloration par noeud
        if options.node_colors.len() > 0 {
            // Utilisation des couleurs definies par l'utilsateur
            if options.gene_colors.len() > 0 {
                let _idx_user_color =  index.color_node_idx % options.gene_colors.len();
                gene_color = options.gene_colors[_idx_user_color].clone();
            }
            else {
                // Utilisation des couleurs par défaut (sans modulation aleatoire)
                gene_color  = get_default_color(index.color_node_idx);
            }
        }
        let _parent =  match index.parent {
            Some(p) => {
                let n = &tree.arena[p];
                let chemin = match index.is_a_transfert {
                    true => {
                        get_chemin_carre(
                            index.x,
                            index.y,
                            n.x,
                            n.y,
                            gene_color.to_string(),
                            config.gene_opacity.to_string(),
                            true,
                            options.gthickness,
                        )
                    },
                    false => {
                        get_chemin_carre(
                            index.x,
                            index.y,
                            n.x,
                            n.y,
                            gene_color.to_string(),
                            config.gene_opacity.to_string(),
                            false,
                            options.gthickness,
                        )
                    },
                };
                if tree.arena[p].visible {
                    g.append(chemin);
                }
                0
            },
            None => { -1 },
        };
        let  event = &index.e;
        match event {
            Event::Leaf => {
                g.append(
                    get_half_circle(
                        index.x,
                        index.y,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                )
            },
            Event::Duplication => {
                g.append(
                    get_carre(
                        index.x,
                        index.y,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                )
            },
            Event::Loss => {
                let mut cross = get_cross(
                    index.x,
                    index.y,
                    options.squaresize * 0.5,
                    gene_color.to_string(),
                    config.gene_opacity.to_string(),
                );
                cross.assign(
                    "transform",
                    "rotate(45 ".to_owned() + &index.x.to_string() + " " + &index.y.to_string() + ")",
                );
                g.append(cross);
            },
            Event::BranchingOut => {
                let mut diamond = get_carre(
                    index.x,
                    index.y,
                    options.squaresize ,
                    "orange".to_string(),
                    config.gene_opacity.to_string(),
                );
                diamond.assign(
                    "transform",
                    "rotate(45 ".to_owned() + &index.x.to_string() + " " + &index.y.to_string() + ")",
                );
                g.append(diamond);
            },
            Event::Hybridation  => {
                g.append(
                    get_empty_circle(
                        index.x,
                        index.y,
                        options.squaresize * 0.75,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                )
            },
            _  => {
                if index.visible {
                    g.append(
                        get_circle(
                            index.x,
                            index.y,
                            options.squaresize * 0.75,
                            gene_color.to_string(),
                            config.gene_opacity.to_string(),
                        )
                    )
                }
            },
        };
        match index.is_a_transfert {
            true => {
                g.append(
                    get_triangle(
                        index.x,
                        index.y - 6.0,
                        options.squaresize ,
                        "yellow".to_string(),
                        config.gene_opacity.to_string(),
                    )
                )
            },
            false => {},
        };
        // Display name
        if index.visible {
            let mut element = Element::new("text");
            match tree.is_leaf(index.idx) {
                false => {
                    element.assign("x", index.x - 5.0 )
                    },
                true => {
                    // Cas d'une feuille: on decale pour prendre en compte les timelines
                    element.assign("x", index.x  +  options.time_lines.len() as f32  * width_timeline);
                },
            }
            element.assign("y", index.y + 10.0);
            // Mode coloration par noeud
            if options.node_colors.len() > 0 {
                element.assign("class", "node_".to_owned() + &index.color_node_idx.to_string());
            }
            // Mode coloration par gene
            else {
                // plusieurs couleurs
                if options.gene_colors.len() > 0 {
                    element.assign("class", "gene_0");
                    }
                // une seule couleur (usage par defaut)
                else {
                    element.assign("class", "gene");
                }
            }
            let txt  = Text::new(&index.name);
            element.append(txt);
            element.assign(
                "transform",
                "rotate(90 ".to_owned() + &(index.x - 5.0).to_string() + ","
                + &(index.y + 10.0).to_string() + ")"
            );
            match tree.is_leaf(index.idx) {
                true => g.append(element),
                _  =>  match options.gene_internal {
                            true =>  g.append(element),
                            false => {},
                        },
            };
            // Display support
            let mut element = Element::new("text");
            element.assign("x", index.x - 15.0);
            element.assign("y", index.y + 10.0);
            element.assign("class", "support");
            let txt  = Text::new(&index.support);
            element.append(txt);
            element.assign(
                "transform",
                "rotate(90 ".to_owned() + &(index.x - 15.0).to_string()
                + "," + &(index.y + 10.0).to_string() + ")",
            );
            match tree.is_leaf(index.idx) {
                true => {},
                _  => match options.support {
                    true =>  g.append(element),
                    false => {},
                    },
            };
            // Display branch length
            match options.branch {
                true => {
                    // only if  the node has a parent
                    match index.parent {
                        Some(p)=> {
                            let mut element = Element::new("text");
                            let n = &tree.arena[p];
                            let x_dist = index.x + 5.0;
                            // Display in the furst quarter of the branch lenght
                            let y_dist = n.y + (index.y - n.y) / 4.0 ;
                            element.assign("x", x_dist);
                            element.assign("y", y_dist);
                            element.assign("class", "branch");
                            let txt  = Text::new(&index.l.to_string());
                            element.append(txt);
                            element.assign(
                                "transform",
                                "rotate(90 ".to_owned() + &(x_dist).to_string() + ","
                                + &(y_dist).to_string() + ")",
                            );
                            g.append(element);
                        },
                        None => {},
                    }
                },
                false => {},
            }
        }
    }
    // Note : simple tree
    let mut transfo: String = "translate(  ".to_owned();
    transfo.push_str(&( x_viewbox).to_string());
    transfo.push_str(" ");
    let psize = &config.gene_police_size.parse::<f32>().unwrap();
    transfo.push_str(&((width_svg  + y_viewbox + x_viewbox + psize)).to_string());
    transfo.push_str(") rotate(-90 0 0 ) ");
    match options.rotate {
        true => g.assign("transform",transfo),
        false => {}
    };
    document.append(g);
    svg::save(name, &document).unwrap();
}
/// Get the largest x in set of trees.
pub fn get_longest_x_mul(gene_trees: &mut std::vec::Vec<ArenaTree<String>>) -> f32 {
    let mut max = 0.0;
    for tree in gene_trees {
        let gene_max = tree.get_largest_x();
        if gene_max > max {
            max = gene_max;
        }
    }
    max
}
/// Get the longest node name in a set of trees.
pub fn get_longest_name_mul(gene_trees: &mut std::vec::Vec<ArenaTree<String>>) -> usize {
    let mut max = 0;
    for tree in gene_trees {
        let gene_max = get_longest_name(tree);
        if gene_max > max {
            max = gene_max;
        }
    }
    max
}
/// Get the longest node name in a tree.
pub fn get_longest_name(gene_tree: &ArenaTree<String>) -> usize {
    let mut max = 0;
    for node in &gene_tree.arena {
        if node.name.len() > max {
            max = node.name.len();
        }
    }
    max
}
/// Draw a svg pipe species tree and  several gene trees inside it.
pub fn draw_sptree_gntrees (
    sp_tree: &mut ArenaTree<String>,                    // species tree
    gene_trees: &mut std::vec::Vec<ArenaTree<String>>,   // gene trees
    name: String,                                       // output file
    options: &Options,                                  // drawing options
    config: &Config,                                    // svg configuration
    transfers: &std::vec::Vec<(String,String)>          // additional transfers
    ) {
    let mut largest_x = sp_tree.get_largest_x() * 1.0 + 0.0 ;
    let largest_x_genes = get_longest_x_mul(gene_trees); //Gestion des genes free living qio peuvent depasser de l'arbre d'espece
    if  largest_x_genes > largest_x {
        largest_x = largest_x_genes;
    }
    let largest_y = sp_tree.get_largest_y() * 1.0 + 0.0 ;
    let largest_y_nofl = sp_tree.get_largest_y_nofl() * 1.0 + 0.0 ; // y le plus large sans compte le free living
    let smallest_x = sp_tree.get_smallest_x() * 1.0 + 0.0 ;
    let smallest_y = sp_tree.get_smallest_y() * 1.0 + 0.0 ;
    let width_svg = largest_x - smallest_x + 1.0 * BLOCK;
    let width_svg = width_svg * 1.0;
    let mut longest_name = get_longest_name_mul(gene_trees) as f32;
    let species_longest_name = get_longest_name(sp_tree) as f32;
    if species_longest_name > longest_name {
        longest_name = species_longest_name;
    }
    let height_svg = largest_y - smallest_y + longest_name *
        config.gene_police_size.parse::<f32>().unwrap();
    let height_svg = height_svg * 1.0;
    let x_viewbox = smallest_x - 0.0 ;
    let y_viewbox = smallest_y - 0.0;
    let width_timeline = 20.0;
    let  mut document = match options.rotate {
        true => Document::new()
                .set("width", height_svg)
                .set("height", width_svg)
                .set("viewBox", (
                    x_viewbox,
                    y_viewbox,
                    height_svg * 1.1 + 2.0 * BLOCK,
                    width_svg + 2.0 * BLOCK,
                    )
                )
                .set("style","background-color:".to_owned() + &options.bckg_color),
        false => Document::new()
                .set("width",width_svg)
                .set("height",height_svg)
                .set("viewBox", (
                    x_viewbox,
                    y_viewbox - 0.5 * BLOCK,
                    width_svg + 0.5 *BLOCK,
                    height_svg *1.1 + 0.5 *BLOCK,
                    )
                )
               .set("style","background-color:".to_owned() + &options.bckg_color),
    };
    // Initialise la chaine de carctere dediee aux styles des fonts : font pour l'espece
    let mut recphylostyle:String = ".species { font: italic ".to_owned() + "; font-size: "
        + &config.species_police_size.to_string() + "px; fill: "
        + &config.species_police_color.to_string() + "; }";
    let mut g = Element::new("g");
    // Dessine l'arbre d'espece
    // ------------------------
    for index in &sp_tree.arena {
        // Dessine le tuyeau
        match index.parent {
            Some(p) => {
                let n = &sp_tree.arena[p];
                let color_branch_species = match index.e {
                    Event::Loss => {
                        "black".to_string()
                    },
                    _ => {
                        match index.is_a_transfert {
                            true => "green".to_string(),
                            false => config.species_color.to_string(),
                        }
                    },
                };
                let chemin = get_chemin_sp(
                    index.x,
                    index.y,
                    index.width / 2.0,
                    index.height / 2.0,
                    n.x,
                    n.y,
                    n.width / 2.0,
                    n.height / 2.0,
                    color_branch_species.clone(),
                    config.species_opacity.to_string(),
                    options.sthickness,
                );
                if sp_tree.arena[p].visible {
                    g.append(chemin);
                }
                if config.fill_species || options.fill_species {
                    let (chemin2,chemin3) = get_chemin_sp_filled(
                        index.x,
                        index.y,
                        index.width / 2.0,
                        index.height / 2.0,
                        n.x,
                        n.y,
                        n.width / 2.0,
                        n.height / 2.0,
                        color_branch_species.clone(),
                        config.species_opacity.to_string(),
                    );
                    if sp_tree.arena[p].visible {
                        g.append(chemin2);
                        g.append(chemin3);
                    };
                }
                if sp_tree.is_leaf(index.idx) {
                    // Set the y value of the pipe leaf ro the highest value of the y gene leaves
                    let mut max_gene_y = index.y;
                    for (gene_index, gene_node ) in &index.nodes {
                        let gene_y = gene_trees[*gene_index].arena[*gene_node].y;
                        if  gene_trees[*gene_index].arena[*gene_node].e != Event::Loss {
                            if gene_y >  max_gene_y {
                                max_gene_y = gene_y;
                            }
                        }
                    };
                    if max_gene_y == index.y {
                        // Dans le cas ou la feuille espece ne contient pas de gene ou seulement
                        // des loss (un peu tordu et surement inutile vu que l'on modifie
                        // cette valeur ensuite.)
                         max_gene_y = max_gene_y + index.height / 2.0;
                        // max_gene_y = max_gene_y + largest_height / 2.0;
                    }
                    if ! options.real_length_flag && index.e != Event::Loss {
                        max_gene_y = largest_y_nofl ;
                    }
                    let chemin = close_chemin_sp(
                        index.x,
                        index.y,
                        index.width / 2.0,
                        index.height / 2.0,
                        max_gene_y - index.y,
                        color_branch_species.clone(),
                        match index.collapsed {
                            true => "0.0".to_string(),
                            false => config.species_opacity.to_string(),
                        },
                        //config.species_opacity.to_string(),
                        options.sthickness,
                    );
                    if sp_tree.arena[p].visible {
                        g.append(chemin)
                    };
                    if config.fill_species || options.fill_species {
                        let chemin2 = close_chemin_sp_filled(
                            index.x,
                            index.y,
                            index.width / 2.0,
                            index.height / 2.0,
                            max_gene_y - index.y,
                            color_branch_species,
                            config.species_opacity.to_string(),
                        );
                        if sp_tree.arena[p].visible {
                            g.append(chemin2);
                        };
                    }
                }
                match index.e {
                    Event::Duplication => {
                        let carre = get_carre(
                            index.x,
                            index.y,
                            1.5 * index.width,
                            config.species_color.to_string(),
                            config.species_opacity.to_string(),
                        );
                        g.append(carre);
                    },
                    _=> {},
                }
            },
            None => {},
        };
        // Display branch length
        match options.branch {
            true => {
                // only if  the node has a parent
                match index.parent {
                    Some(p)=> {
                        let mut element = Element::new("text");
                        let n = &sp_tree.arena[p];
                        let x_dist = index.x + index.width / 2.0 + 5.0;
                        // Display in the furst quarter of the branch lenght
                        let y_dist = n.y + (index.y - n.y) / 4.0  + index.height / 2.0 ;
                        element.assign("x", x_dist);
                        element.assign("y", y_dist);
                        element.assign("class", "branch");
                        let txt  = Text::new(&index.l.to_string());
                        element.append(txt);
                        element.assign(
                            "transform",
                            "rotate(90 ".to_owned() + &(x_dist).to_string()
                            + "," + &(y_dist).to_string()+")",
                        );
                        g.append(element);
                    },
                    None => {},
                }
            },
            false => {},
        }
     }
    // Boucle sur les arbres de genes
    // ------------------------------
    let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
    let mut idx_rcgen = 0;
    loop {
        /* sort de la bcoule si il n'y a pas de genes*/
        if nb_gntree == 0 {
            break;
        }
        // Cas de la coloration par noeuds: on modifie l'index de couleur des descendants de chaque
        // noeud dans options.node_colors
        if options.node_colors.len() > 0 {
            // Gestion des styles
            // On definit la couleur des font pour le style node_0
            // Style de la font pour la partie de l'arbre
            let mut font_color  = get_default_color(0);
            // Cas ou on utilise les couleurs definies par l'utilisateur
            if options.gene_colors.len() > 0 {
                font_color = options.gene_colors[0].clone();
            }
            let added_style = " .node_0".to_owned()
            + " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
            + &font_color.to_string() + "; } ";
            // Je passe en str pour l'ajouter
            let add_style_str :&str = &added_style;
            recphylostyle.push_str(add_style_str);
        	let mut node_color_idx = 1; // Les autres restent a 0
        	// Traite tous les noeuds a colorer
        	for node_color in  &options.node_colors {
        		// Recupere le noeud dans l'arbre
        		let node_colored = gene_trees[idx_rcgen].get_index(node_color.to_string());
        		match node_colored {
        			Ok(n) => {
        				// On definit la couleur des font pour le style
        				// Style de la font pour la partie de l'arbre
        				let mut font_color  = get_default_color(node_color_idx);
        				// Cas ou on utilise les couleurs definies par l'utilisateur
        				if options.gene_colors.len() > 0 {
            				let _idx_user_color = node_color_idx % options.gene_colors.len();
            				font_color = options.gene_colors[_idx_user_color].clone();
            			}
        				let added_style = " .node_".to_owned() + &node_color_idx.to_string()
             			+ " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
             			+ &font_color.to_string() + "; } ";
        				// Je passe en str pour l'ajouter
        				let add_style_str :&str = &added_style;
        				recphylostyle.push_str(add_style_str);
        				// Modifie la valeur de l'index de couleur dans tous les fils du noeud
        				set_color_index(&mut gene_trees[idx_rcgen],n,node_color_idx);
        			},
        			Err(_e) =>{
        				println!("Warning Node {} not found in the gene tree number {}",node_color, idx_rcgen)
        			},
        		}
        		node_color_idx += 1;
        	}
        }
        // Choix de la couleur dans le cas d'une coloration par arbre de gene par defaut
        // Boucle sur une base de 6 couleurs avec une modif aleatoire (utile qaund on a pmlus de 6 genes)
        let base_couleur = match &idx_rcgen % 6 {
             5 => Color::Orange,
             0 => Color::Blue,
             1 => Color::Purple,
             2 => Color::Green,
             3 => Color::Red,
             4 => Color::Yellow,
             _ => Color::Monochrome, // Jamais
        };
        let mut gene_color = RandomColor::new()
            .hue(base_couleur)
            .luminosity(Luminosity::Bright) // Optional
            .alpha(1.0) // Optional
            .to_rgb_string(); //
        // Modification de la couleur dans le cas d'une coloration par arbre de gene défini par l'utilisateur
        if options.gene_colors.len() > 0 {
            let _idx_user_color = &idx_rcgen % options.gene_colors.len();
            gene_color = options.gene_colors[_idx_user_color].clone();
        }
        // Style de la font pour le gene
        let added_style = " .gene_".to_owned() + &idx_rcgen.to_string()
             + " { font-size: " + &config.gene_police_size.to_string() + "px; fill:"
             + &gene_color.to_string() + "; } ";
        // Je passe en str pour l'ajouter
        let add_style_str :&str = &added_style;
        recphylostyle.push_str(add_style_str);
        // Exploration de l'arbre de gene
        // ------------------------------
        for  index in &gene_trees[idx_rcgen].arena {
 			// Cas ou la coloration varie dans l'arbre des genes
            if options.node_colors.len() > 0 {
            	// Utilisation des couleurs definies par l'utilsateur
            	if options.gene_colors.len() > 0 {
            		let _idx_user_color =  index.color_node_idx % options.gene_colors.len();
            		gene_color = options.gene_colors[_idx_user_color].clone();
            		}
            	else {
            	    // Utilisation des couleurs par défaut (sans modulation aleatoire)
            	    gene_color  = get_default_color(index.color_node_idx);
            	}
            }
            // Dessine le chemin du noeud a son pere
            match index.parent {
                 Some(p) => {
                    let n = &gene_trees[idx_rcgen].arena[p];
                     // La forme du chemin depend de l'evenement
                     let chemin = match index.is_a_transfert {
                        true => {
                            // Si  flag thickness, les transfers sont affiches plus tard,
                            // selon leur redondance
                            let transfer_opacity = match options.thickness_flag {
                                    true => "0.0".to_string(),
                                    false => config.gene_opacity.to_string(),
                            };
                            // Verifie que le parent est bien un branchingout
                            match n.e {
                                Event::BranchingOut => get_chemin_transfer(
                                    index.x,
                                    index.y,
                                    n.x,
                                    n.y,
                                    gene_color.to_string(),
                                    transfer_opacity,
                                    config.bezier.to_string().parse::<f32>().unwrap(),
                                    1,
                                    options.gthickness,
                                ),
                                Event::BifurcationOut => get_chemin_transfer(
                                    index.x,
                                    index.y,
                                    n.x,
                                    n.y,
                                    gene_color.to_string(),
                                    transfer_opacity,
                                    config.bezier.to_string().parse::<f32>().unwrap(),
                                    2,
                                    options.gthickness,
                                ),
                                _ => {
                                    // Si le noaud courant est vrituel,je peux tracer quand  meme
                                    // ( a verfier)
                                    if index.virtualsvg {
                                        get_chemin_transfer(
                                            index.x,
                                            index.y,
                                            n.x,
                                            n.y,
                                            gene_color.to_string(),
                                            transfer_opacity,
                                            config.bezier.to_string().parse::<f32>().unwrap(),
                                            1,
                                            options.gthickness,
                                        )
                                    }
                                    else {
                                        // Si c'est le pere qui est virtuel on va voir le grand PERE
                                        if n.virtualsvg {
                                            let ppp = n.parent.expect("[draw_sptree_gntrees] ERROR: Unable to get the father of the node");
                                            let nnn = &gene_trees[idx_rcgen].arena[ppp];
                                            if nnn.e != Event::BranchingOut && nnn.e != Event::BifurcationOut {
                                                panic!("Wrong recPhyloXML feature in tree # {}.The (grand)father node should be BranchingOut or BifurcationOut, but I found a {:?} Father node: {:?}\nCurrent node: {:?}",
                                                idx_rcgen,nnn.e,nnn,index);
                                            }
                                            get_chemin_transfer(
                                                index.x,
                                                index.y,
                                                nnn.x,
                                                nnn.y,
                                                gene_color.to_string(),
                                                1.0.to_string(),
                                                config.bezier.to_string().parse::<f32>().unwrap(),
                                                1,
                                                options.gthickness,
                                            )
                                        }
                                        else {
                                            panic!("Wrong recPhyloXML feature in tree # {}.The father node should be BranchingOut or BifurcationOut, but I found a {:?} Father node: {:?}\nCurrent node: {:?}",
                                            idx_rcgen,n.e,n,index);
                                        }
                                    }
                                },
                            }
                        },
                        false => {
                            // Vérifie que le noeud n'est pas associé à FREE_LIVING
                            if index.location != "FREE_LIVING".to_string() {
                                get_chemin_carre(
                                    index.x,
                                    index.y,
                                    n.x,
                                    n.y,
                                    gene_color.to_string(),
                                    config.gene_opacity.to_string(),
                                    index.collapsed,
                                    options.gthickness,
                                )
                            }
                            else {
                                // Le noeud est  associé à FREE_LIVING
                                // Calcule l'opacite de l'arbre free living
                                let free_opacity = config.gene_opacity.parse::<f32>().unwrap() / 2.0 ;
                                if n.location == "FREE_LIVING".to_string() {
                                    // La branche est dans l'arbre free living
                                    get_chemin_carre(
                                        index.x,
                                        index.y,
                                        n.x,
                                        n.y,
                                        gene_color.to_string(),
                                        free_opacity.to_string(),
                                        false,
                                        options.gthickness,
                                    )
                                }
                                else {
                                    // La branche  est entre la racine de l'arbre free living
                                    // et le reste
                                    get_chemin_transfer(
                                        index.x,
                                        index.y,
                                        n.x,
                                        n.y,
                                        gene_color.to_string(),
                                        free_opacity.to_string(),
                                        config.bezier.to_string().parse::<f32>().unwrap(),
                                        3,
                                        options.gthickness,
                                    )
                                }
                            }
                        },
                     };
                     if index.visible {
                         g.append(chemin);
                    }
                 },
                 None => {
                    // C'est la racine
                    let triangle = get_triangle(
                        index.x,
                        index.y - options.squaresize,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    );
                    g.append(triangle);
                 },
             };
            // Dessine le symbole associe au noeud
            let  event = &index.e;
            if index.visible {
             match event {
                Event::Leaf => g.append(
                    get_half_circle(
                        index.x,
                        index.y,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                ),
                Event::Duplication => g.append(
                    get_carre(
                        index.x,
                        index.y,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                ),
                Event::Loss => {
                    let mut cross = get_cross(
                        index.x,
                        index.y,
                        options.squaresize * 0.5,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    );
                    cross.assign(
                        "transform",
                        "rotate(45 ".to_owned() + &index.x.to_string() + " "
                        + &index.y.to_string() + ")",
                    );
                    g.append(cross);
                },
                // Normalement il ny' a pas d event transferBack : comme il est toujour associé
                // a un autre event,c'est ce dernier qui est stocké dans le champs "e"
                // Par contre le champs "is_a_transfert" indique si le noeud prvient d'un transfer
                Event::BranchingOut  =>  {
                    let mut diamond = get_carre(
                        index.x,
                        index.y,
                        options.squaresize,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    );
                    diamond.assign(
                        "transform",
                        "rotate(45 ".to_owned() + &index.x.to_string() + " "
                        + &index.y.to_string() + ")",
                    );
                    g.append(diamond);
                },
                // Est ce que BifurcationOut existe encore ???
                Event::BifurcationOut => g.append(
                    get_carre(
                        index.x,
                        index.y,
                        options.squaresize,
                        "yellow".to_string(),
                        config.gene_opacity.to_string(),
                    )
                ),
                Event::Speciation => g.append(
                    get_circle(
                        index.x,
                        index.y,
                        options.squaresize * 0.75,
                        gene_color.to_string(),
                        // Pas d'affichage de speciation dans les noeuds collapses
                        match index.collapsed {
                            true => "0".to_string(),
                            false =>  config.gene_opacity.to_string(),
                        },
                        
                    )
                ),
                Event::ObsoleteSpeciationLoss => g.append(
                    get_circle(
                        index.x,
                        index.y,
                        options.squaresize * 0.75,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                ),
                Event::Hybridation => g.append(
                    get_empty_circle(
                        index.x,
                        index.y,
                        options.squaresize * 0.75,
                        gene_color.to_string(),
                        config.gene_opacity.to_string(),
                    )
                ),
                _ =>  {},
            };
        };
            // Affiche le texte associe au noeud
            if index.visible {
            match event {
                Event::Leaf => {
                    let mut element = Element::new("text");
                    element.assign("x", index.x - 5.0  +  options.time_lines.len() as f32  * width_timeline);
                    element.assign("y", index.y + 15.0 );
                    if options.node_colors.len() > 0 {
 					    // Mode coloration par noeud
                    	element.assign("class", "node_".to_owned() + &index.color_node_idx.to_string());
                    }
                    else {
                        // Mode coloration par gene
                    	element.assign("class", "gene_".to_owned() + &idx_rcgen.to_string());
                    }
                    let txt = Text::new(&index.name);
                    element.append(txt);
                    element.assign(
                        "transform",
                        "rotate(90 ".to_owned() + &(index.x - 5.0).to_string() + ","
                        + &(index.y + 15.0).to_string() + ")",
                    );
                    g.append(element);
                },
                _ => {
                    match options.gene_internal {
                        true =>  {
                            let mut element = Element::new("text");
                            element.assign("x", index.x+10.0);
                            element.assign("y", index.y+0.0);
                            if options.node_colors.len() > 0 {
                                // Mode coloration par noeud
                                element.assign("class", "node_".to_owned() + &index.color_node_idx.to_string());
                            }
                            else {
                                // Mode coloration par gene
                                element.assign("class", "gene_".to_owned() + &idx_rcgen.to_string());
                            }
                            let txt  = Text::new(&index.name);
                            element.append(txt);
                            element.assign("transform","rotate(90 ".to_owned()
                            + &index.x.to_string() + "," + &index.y.to_string()+")");
                            g.append(element);
                        },
                        false => {},
                    }
                },
            }
        }
        }
        idx_rcgen += 1;
        if idx_rcgen == nb_gntree {
            break;
        }
    }

    // Analyse l'abondance des transferts
    // ----------------------------------
    let mut unique_transfers: std::vec::Vec<(String,String)> = vec![];
    let mut scores: std::vec::Vec<usize> = vec![];
    let mut score_max = 1;
    if options.thickness_disp_score {
        // Style de la font pour affichage du nb de transcr
        let added_style = " .rouge { font-size: ".to_owned()+ &config.gene_police_size.to_string()
        + "px; fill:red ; } ";
        // Je passe en str pour l'ajouter
        let add_style_str :&str = &added_style;
        recphylostyle.push_str(add_style_str);
    }
    if options.branch {
        // Style de la font pour affichage du nb de transcr
        let added_style = " .branch { font-size: ".to_owned()
        + &config.species_police_size.to_string() +"px; fill:black ; } ";
        // Je passe en str pour l'ajouter
        let add_style_str :&str = &added_style;
        recphylostyle.push_str(add_style_str);
    }


    // Affiche les timelines.
    display_timelines(
        sp_tree,   // species tree
        options,   // drawing options
        &mut g,
        width_timeline
    );

    // Affiche les noms de  l'arbre d'espece ( on le fait a la fin pour que ce soit sur le dessus dans le svg)
    for index in &sp_tree.arena {
        let mut element = Element::new("text");
        // Affiche le texte associe au noeud
        match sp_tree.is_leaf(index.idx) {
            true => {
                let txt_size = index.name.len() as f32 * config.species_police_size.parse::<f32>().unwrap();
                element.assign("x", index.x  - txt_size / 2.0);
                element.assign("y", index.y - index.width /2.0 - 10.0);
                element.assign("class", "species");
                let txt  = Text::new(&index.name);
                element.append(txt);
                element.assign(
                    "transform",
                    "rotate(90 ".to_owned() + &index.x.to_string() + ","
                    + &index.y.to_string() + ")",
                );
                if index.visible {
                    g.append(element);
                }
            },
            false => {
                match options.species_internal {
                    true => {
                        element.assign("x", index.x + 15.0);
                        element.assign("y", index.y + 20.0);
                        element.assign("class", "species");
                        let txt = Text::new(&index.name);
                        element.append(txt);
                        element.assign(
                            "transform",
                            "rotate(90 ".to_owned() + &index.x.to_string()
                            + "," + &index.y.to_string() + ")",
                        );
                        if index.visible {
                            g.append(element);
                        }
                    },
                    false => {},
                };
            },
        };
    }

    // Ajout le style
    let style = Style::new(recphylostyle);
    document.append(style);

    // Ajout des transferts
    for transfer in transfers {
        let (end, start) = &transfer;
        if  ((options.trans_start.as_ref() == None) || (options.trans_start.as_ref() == Some(start))) &&
            ((options.trans_end.as_ref() == None) || (options.trans_end.as_ref() == Some(end))) {
            /*if (options.trans_start.as_ref() != None) || (options.trans_end.as_ref() != None) {
                println!("Selected transfer {:?} => {:?}",end,start);
            }*/
            let index = unique_transfers.iter().position(|r| r == transfer);
            match index {
                None => {
                    unique_transfers.push(transfer.clone());
                    scores.push(1);
                },
                Some(i) => {
                    scores[i] = scores[i]+ 1;
                    if scores[i] > score_max {
                        score_max = scores[i];
                    }
                },
            }
        }
    }
    let mut i_trans = 0;
    while i_trans < unique_transfers.len() {
        let (end, start) = &unique_transfers[i_trans];
        let score = scores[i_trans];
        info!("[draw_sptree_gntrees] Additional transfer {}: {}=>{} Score = {}",i_trans, start, end, score);
        i_trans = i_trans + 1;
        if score > options.thickness_thresh {
            info!("[draw_sptree_gntrees] Selecting additional transfer {:?} {:?}",start, end);
            let start_node = sp_tree.get_index(start.to_string()).expect("arg");
            let end_node = sp_tree.get_index(end.to_string()).expect("arg");
            let opacity = score * 100 / score_max ;
            let opacity = opacity as f32 / 100.0 ;
            let opacity = opacity.to_string();
            info!("[draw_sptree_gntrees] Adding additional transfer {:?} {:?} with opacity {}",start, end, opacity);
            let chemin = get_chemin_transfer(
                sp_tree.arena[start_node].x,
                sp_tree.arena[start_node].y,
                sp_tree.arena[end_node].x,
                sp_tree.arena[end_node].y,
                "red".to_string(),
                opacity,
                config.bezier.to_string().parse::<f32>().unwrap(),
                0,
                options.gthickness,
            );
            g.append(chemin);
            // Affichage du score
            if options.thickness_disp_score {
                let bez_y = config.bezier.to_string().parse::<f32>().unwrap() * BLOCK;
                // Point de controle de la courbe de Bezier
                let controle_x = (sp_tree.arena[start_node].x + sp_tree.arena[end_node].x) / 2.0  ;
                let controle_y = (sp_tree.arena[start_node].y + sp_tree.arena[end_node].y) / 2.0
                - bez_y - ( score.to_string().len() as f32 * &config.gene_police_size.parse::<f32>().unwrap() );
                let mut element = Element::new("text");
                element.assign("x", controle_x - 0.0);
                element.assign("y", controle_y + 0.0);
                element.assign("class", "rouge");
                let txt  = Text::new(score.to_string());
                element.append(txt);
                match options.rotate {
                    false => {}
                    true => element.assign(
                        "transform",
                        "rotate(90 ".to_owned()
                        + &controle_x.to_string() + "," + &controle_y.to_string()+")"
                    ),
                };
                g.append(element);
            }
        }
    }



    let mut transfo: String = "translate(  ".to_owned();
    transfo.push_str(&( x_viewbox).to_string());
    transfo.push_str(" ");
    transfo.push_str(&((width_svg  + x_viewbox + y_viewbox)).to_string());
    transfo.push_str(") rotate(-90 0 0 ) ");
    match options.rotate {
        true => g.assign("transform",transfo),
        false => {}
    };
    document.append(g);
    svg::save(name, &document).unwrap();
}

/// Displaying timelines
pub fn display_timelines(
        tree: &mut ArenaTree<String>,  // tree
        options: &Options,             // drawing options
        g: &mut Element,               // svg element 
        width_timeline: f32            // timeline width
){

    // affiche les timeline des noeuds internes
    let mut unknown_symbols = false;
    // Structure utilisée pour classer les noeuds internes selon leur distance à la racine.
    #[derive(Debug)]
    struct TimeLineNode {
        color: String,
        index: usize,
        root_distance: usize,
    }
    // La longeur maximum de l'arbre pour determiner  l'emplacement de la timeline
    let max_y = tree.get_largest_y_nofl();
    let mut idx_tl = 0.0;
    // Boucle sur les differentes timelines
    for time_line in &options.time_lines {
        // On remplit vecteur de  TimeLineNode
        let mut time_line_nodes: std::vec::Vec<TimeLineNode> = vec![];
        for (node_name, node_color) in time_line.iter() {
            let node = tree.get_index(node_name.to_string());
            match node {
                Err(_err) => eprintln!("There is no node named {}",node_name),
                Ok(node) => {
                        let root_dist = get_root_distance(tree,node,&mut 0);
                        time_line_nodes.push(TimeLineNode {color: node_color.to_string(), index:node, root_distance: root_dist});
                },
            }
        }
        // Classement de la structure selon la distance à la racine.
        time_line_nodes.sort_by(|a, b| a.root_distance.cmp(&b.root_distance));
        // Dessine les timelines
        for time_line_node in time_line_nodes {
            let (min,max) = get_x_span(&tree,time_line_node.index);
            match tree.is_leaf(time_line_node.index) {
                // Cas d'une feuille
                true => {
                    // get the value associated to the considered nodes
                    // (time_line_node.index = index du noeud dans l'arbre)
                    let tl_x = tree.arena[time_line_node.index].x;
                    let tl_width = tree.arena[time_line_node.index].width;
                    if time_line_node.color.starts_with('%'){
                       let v: Vec<&str> = time_line_node.color.split(":").collect();
                       let time_line_symbol = v[0];
                       let mut time_line_symbol_color = "red";
                       if v.len() > 1 {
                           time_line_symbol_color = v[1];
                       }
                       let mut find_symbol = false;
                       if time_line_symbol.to_string() == "%circle" {
                           find_symbol = true;
                           g.append(
                               get_circle(
                                   tl_x ,
                                   max_y + idx_tl * width_timeline + 5.0 + width_timeline / 2.0,
                                   width_timeline / 2.5,
                                   time_line_symbol_color.to_string(),
                                   "1.0".to_string(),
                               )
                           )
                       }
                       if time_line_symbol.to_string() == "%square" {
                           find_symbol = true;
                           g.append(
                               get_carre(
                                   tl_x ,
                                   max_y + idx_tl * width_timeline + 5.0 + width_timeline / 2.0,
                                   width_timeline / 2.0,
                                   time_line_symbol_color.to_string(),
                                   "1.0".to_string(),
                               )
                           )
                       }
                       if time_line_symbol.to_string() == "%cross" {
                           find_symbol = true;
                           g.append(
                               get_cross(
                                   tl_x ,
                                   max_y + idx_tl * width_timeline + 5.0 + width_timeline / 2.0,
                                   width_timeline / 4.0,
                                   time_line_symbol_color.to_string(),
                                   "1.0".to_string(),
                               )
                           )
                       }
                       if time_line_symbol.to_string() == "%halfcircle" {
                           find_symbol = true;
                           g.append(
                               get_half_circle(
                                   tl_x ,
                                   max_y + idx_tl * width_timeline + 5.0 + width_timeline / 2.0,
                                   width_timeline / 2.5,
                                   time_line_symbol_color.to_string(),
                                   "1.0".to_string(),
                               )
                           )
                       }
                       if time_line_symbol.to_string() == "%triangle" {
                           find_symbol = true;
                           g.append(
                               get_triangle(
                                   tl_x ,
                                   max_y + idx_tl * width_timeline + 10.0 + width_timeline / 2.0,
                                   width_timeline / 2.0,
                                   time_line_symbol_color.to_string(),
                                   "1.0".to_string(),
                               )
                           )
                       }
                       if ! find_symbol {
                           eprintln!("The symbol {} is unknown",time_line_symbol);
                           unknown_symbols = true;
                       }
                   }
                   else {
                    let chemin = get_timeline(
                        tl_x - tl_width / 2.0,
                        // lo,ng index.x - index.width ,
                        max_y + idx_tl * width_timeline + 5.0,
                        //index.y,
                        tl_width ,
                        //lo,g index.width * 4.0 ,
                        width_timeline,
                        time_line_node.color.to_string(),
                        time_line_node.color.to_string()
                    );
                     g.append(chemin);
                 };

                },
                false => {
                    g.append(
                    get_timeline(
                      min,
                      max_y + idx_tl * width_timeline + 5.0,
                      max - min,
                      width_timeline,
                      time_line_node.color.to_string(),
                      time_line_node.color.to_string()
                  ))
              },
            };
            // g.append(chemin);
        }
        idx_tl = idx_tl + 1.0;
    }
    if unknown_symbols {
        eprintln!("Allowed symbols are:");
        eprintln!("%circle");
        eprintln!("%cross");
        eprintln!("%halfcircle");
        eprintln!("%square");
        eprintln!("%triangle");
    }
}


/// Draw a frame.
pub fn get_timeline (x: f32, y:f32, w:f32, h:f32, c:String, b:String) -> Path {
    let data = Data::new()
        .move_to((x, y))
        .line_by((w, 0.0))
        .line_by((0.0, h))
        .line_by((-w, 0.0))
        .close();
    let path = Path::new()
        .set("fill", c)
        .set("stroke", b)
        .set("stroke-width", 0)
        .set("d", data);
    path
}

#[allow(dead_code)]
/// Draw a frame.
pub fn get_cadre (x: f32, y:f32, w:f32, h:f32, c:String) -> Path {
    let data = Data::new()
        .move_to((x, y))
        .line_by((w, 0.0))
        .line_by((0.0, h))
        .line_by((-w, 0.0))
        .close();
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("stroke-width", 3)
        .set("d", data);
    path
}
/// Draw a square of size s at x,y.
pub fn get_carre (x: f32, y:f32, s:f32, c:String, o:String) -> Path {
    let data = Data::new()
        .move_to((x*1.0 -s*0.5 , y*1.0 -s*0.5))
        .line_by((0, s))
        .line_by((s, 0))
        .line_by((0, -s))
        .close();
    let fill = c.clone();
    let path = Path::new()
        .set("fill", fill)
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", 3)
        .set("d", data);
    path
}
/// Draw a half circle of size s at x,y.
pub fn get_half_circle (x: f32, y:f32, s:f32, c:String, o:String) -> Path {
    let data = Data::new()
        .move_to((x*1.0 -1.0 *s , y*1.0 - 0.0*s  ))
/*        .line_by((2.0 * s, 0))*/
        .elliptical_arc_by(( 0.5 * s, 0.5 * s, 0.0, 0, 1, s * 2.0, 0   ))
        .close();
    let fill = c.clone();
    let path = Path::new()
        .set("fill", fill)
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", 1)
        .set("d", data);
    path
}
/// Draw a triangle of size s at x,y.
pub fn get_triangle (x: f32, y:f32, s:f32, c:String, o:String) -> Path {
    let data = Data::new()
        .move_to((x*1.0, y*1.0))
        .line_by((-s, -s))
        .line_by((2.0 * s, 0))
        .close();
    let fill = c.clone();
    let path = Path::new()
        .set("fill", fill)
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", 1)
        .set("d", data);
    path
}
/// Draw a circle  of size s at x,y.
pub fn get_circle (x: f32, y:f32, r:f32, c:String, o:String) -> Circle {
    let fill = c.clone();
    let circle = Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("fill", fill)
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", 1);
    circle
}
/// Draw an empty circle  of size s at x,y.
pub fn get_empty_circle (x: f32, y:f32, r:f32, c:String, o:String) -> Circle {
    let circle = Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("fill", "white")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", 1);
    circle
}
/// Draw a cross  of size s at x,y.
pub fn get_cross (x: f32, y:f32, s:f32, c:String, o:String) -> Path {
    let data = Data::new()
        .move_to((x * 1.0 , y * 1.0 -s * 2.0))
    .line_by((0, s * 4.0))
    .move_to((x*1.0 - s * 2.0, y * 1.0 ))
    .line_by((s * 4.0, 0));
    let fill = c.clone();
    let path = Path::new()
        .set("fill", fill)
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", s * 1.0)
        .set("d", data);
    path
}
/// Draw a square path between x1,y1 and x2,y2.
pub fn get_chemin_carre (x1: f32, y1:f32, x2: f32, y2:f32, c:String, o:String, stroke:bool, thickness:usize) -> Path {
    let data = Data::new()
        .move_to((x1 * 1.0, y1 * 1.0))
        .line_to((x1 * 1.0, y2 * 1.0))
        .line_to((x2 * 1.0, y2 * 1.0));
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", thickness);
    let path = match stroke {
        true => path.set("stroke-dasharray", "1, 1"),
        false => path,
    };
    let path = path.set("d", data);
    path
}
/// Draw a transfer path between x1,y1 and x2,y2.
pub fn get_chemin_transfer (
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    c: String,
    o: String,
    b:f32,
    stroke:i32,
    thickness:usize,
    ) -> Path {
    // Arrivee du point: un peu avant pour dessiner la fleche
    let initial_y1 = y1 ;
    let y1 = y1 - PIPEBLOCK;
    // Courbure de la courbe de Bezier
    let bez_y = b * BLOCK;
    // let bez_y = 20.0;
    // Point de controle de la courbe de Bezier
    let controle_x = (x1 + x2) / 2.0 ;
    let controle_y = (y1 + y2) / 2.0 - bez_y ;
    // ligne horizontale
    let mut data = "M".to_owned() + &x1.to_string() + " " + &initial_y1.to_string()
    + " L " + &x1.to_string() + " " + &y1.to_string();
    // fleche : pas de fleche si stroke = 3
    if stroke != 3 {
        data = data.to_owned() + "M" + &x1.to_string() + " "
        + &(initial_y1 - PIPEBLOCK / 4.0).to_string() + "L "
        + &(x1 - PIPEBLOCK / 4.0 ).to_string() + " "
        + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
        data = data.to_owned()+ "M" + &x1.to_string() + " "
        + &(initial_y1 - PIPEBLOCK / 4.0).to_string() + "L "
        + &(x1 + PIPEBLOCK / 4.0 ).to_string() + " "
        + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
    }
    // bezier
    data = data.to_owned() + "M" + &x1.to_string() + " " + &y1.to_string()
    + " Q " + &controle_x.to_string() + " " + &controle_y.to_string()
    + " " + &x2.to_string() + " " + &y2.to_string();
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", thickness);
    // choix du pointille
    let path = match stroke {
        3 => path,
        2 => path.set("stroke-dasharray", "4, 1"),
        1 => path.set("stroke-dasharray", "1, 1"),
        0 => path,
        _ => path.set("stroke-dasharray", "1, 1"),
    };
    let path  = path.set("d", data);
    path
}
/// Draw a square pipe path between x1,y1 ad x2,y2
pub fn get_chemin_sp (
    x1: f32,
    y1: f32,
    width1: f32,
    height1: f32,
    x2: f32,
    y2: f32,
    width2: f32,
    height2: f32,
    c: String,
    o: String,
    thickness: usize,
    ) -> Path {
    if x1 < x2 {
        let data = Data::new()
            .move_to((x1 - width1, y1 - height1 + (thickness / 2)  as f32))
            .line_to((x1 - width1, y2 - height2))
            .line_to((x2 - width2 - (thickness / 2)  as f32, y2 - height2))
            .move_to((x1 + width1, y1 - height1 + (thickness / 2)  as f32 ))
            .line_to((x1 + width1, y2 + height2))
            .line_to((x2, y2 + height2));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", c)
            .set("opacity", o)
            .set("stroke-width",thickness)
            .set("d", data);
        path
    }
    else {
        let data = Data::new()
            .move_to((x1 + width1, y1 - height1 + (thickness / 2)  as f32 ))
            .line_to((x1 + width1, y2 - height2))
            .line_to((x2 + width2 + (thickness / 2)  as f32, y2 - height2))
            .move_to((x1 - width1, y1 - height1 + (thickness / 2)  as f32))
            .line_to((x1 - width1, y2 + height2))
            .line_to((x2, y2 + height2));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", c)
            .set("opacity", o)
            .set("stroke-width", thickness)
            .set("d", data);
        path
    }
}
/// Draw a  filled square pipe path between x1,y1 ad x2,y2
pub fn get_chemin_sp_filled (
    x1: f32,
    y1: f32,
    width1: f32,
    height1: f32,
    x2: f32,
    y2: f32,
    width2: f32,
    height2: f32,
    c: String,
    o: String,
) -> (Path,Path) {
    let data1 = Data::new()
        .move_to((x1 - width1, y1 - height1 ))
        .line_to((x1 - width1, y2 - height2))
        .line_to((x1 + width1, y2 - height2))
        .line_to((x1 + width1, y1 - height1));
    let path1 = Path::new()
        .set("fill", c.clone())
        .set("stroke", c.clone())
        .set("opacity", o.clone())
        .set("stroke-width",5)
        .set("d", data1);
    let path2 = match  x1 < x2 {
        true => {
            let data2 = Data::new()
                .move_to((x1 - width1, y2 - height2 ))
                .line_to((x2 + width2, y2 - height2))
                .line_to((x2 + width2, y2 + height2))
                .line_to((x1 + width1, y2 + height2));
            let path = Path::new()
                .set("fill", c.clone())
                .set("stroke", c)
                .set("opacity", o)
                .set("stroke-width",0)
                .set("d", data2);
            path
        },
        false => {
            let data2 = Data::new()
                .move_to((x2 - width2, y2 - height2 ))
                .line_to((x1 + width1, y2 - height2))
                .line_to((x1 + width1, y2 + height2))
                .line_to((x2 + width2, y2 + height2));
            let path = Path::new()
                .set("fill", c.clone())
                .set("stroke", c)
                .set("opacity", o)
                .set("stroke-width",0)
                .set("d", data2);
            path
        },
    };
    (path1,path2)
}

/// Finish  the drawing of species tree at the leaves level.
pub fn close_chemin_sp (
    x1: f32,
    y1: f32,
    width1: f32,
    height1: f32,
    height2: f32,
    c: String,
    o: String,
    thickness: usize,
    ) -> Path {
        let data = Data::new()
            .move_to((x1 - width1, y1 - height1 + (thickness / 2)  as f32  ))
            .line_to((x1 - width1, y1 + 1.0 * height2))
            .line_to((x1 + width1, y1 + 1.0 * height2))
            .line_to((x1 + width1, y1 - height1 + (thickness / 2)  as f32  ));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", c)
            .set("opacity", o)
            .set("stroke-width", thickness)
            .set("d", data);
        path
}
/// Finish  the drawing of filled species tree at the leaves level.
pub fn close_chemin_sp_filled (
    x1: f32,
    y1: f32,
    width1: f32,
    height1: f32,
    height2: f32,
    c: String,
    o: String
    ) -> Path {
        let data = Data::new()
            .move_to((x1 - width1, y1 - height1  ))
            .line_to((x1 - width1, y1 + 1.0 * height2))
            .line_to((x1 + width1, y1 + 1.0 * height2))
            .line_to((x1 + width1, y1 - height1 ));
        let path = Path::new()
            .set("fill", c.clone())
            .set("stroke", c)
            .set("opacity", o)
            .set("stroke-width", 0)
            .set("d", data);
        path
}
/// Set to idx the color_node_idx field of the children of root in gene_tree
pub fn set_color_index(gene_tree: &mut ArenaTree<String>, root :usize, idx : usize){
	gene_tree.arena[root].color_node_idx = idx;
	let children = &gene_tree.arena[root].children;
	if children.len() > 0 {
            let left = children[0];
            let right = children[1];
            set_color_index(gene_tree, left, idx);
            set_color_index(gene_tree, right, idx);
            }
}
/// Get the default color in function of index
pub fn get_default_color (idx:usize) -> String {
	let font = match idx % 6 {
    	5 => "orange".to_string(),
        0 => "blue".to_string(),
        1 => "purple".to_string(),
        2 => "green".to_string(),
        3 => "red".to_string(),
        4 => "yellow".to_string(),
        _ => "monochrome".to_string(), // Jamais
    };
    font
}

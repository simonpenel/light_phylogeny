/// name = "light_phylogeny"
/// version = "0.19.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// license = "CECILL-2.1"
use log::{info};
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::Config;
use crate::arena::Event;
use crate::arena::BLOCK;
use crate::arena::PIPEBLOCK;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Circle;
use svg::node::element::Style;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;
use random_color::{Color,RandomColor,Luminosity};

const GTHICKNESS: usize = 3; // Epaisseur trait gene_
const STHICKNESS: usize = 6; // Epaisseur trait species
const SQUARESIZE: f32 = 6.0; // taille carre dupli

/// Draw a svg simple tree
pub fn draw_tree (
    tree: &mut ArenaTree<String>, // tree
    name: String,                 // output file
    options: &Options,            // display options
    config: &Config,              // svg config
    ) {
    info!("[draw_tree] Drawing tree...");
    let gene_color = & config.single_gene_color;
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
                .set("width",height_svg  )
                .set("height",width_svg  )
                .set("viewBox", (x_viewbox,y_viewbox,
                    height_svg + 2.0 *BLOCK ,width_svg + 2.0 *BLOCK )),
        false => Document::new()
                .set("width",width_svg  )
                .set("height",height_svg  )
                .set("viewBox", (y_viewbox,x_viewbox,width_svg + 0.5 *BLOCK ,height_svg + 2.0 *BLOCK )),
    };
    let style = Style::new(".gene { font:  ".to_owned()
        + &config.gene_police_size.to_string()+"px serif; fill:"
        + &gene_color.to_string() + "; }" );
    document.append(style);
    let style = Style::new(".support { font:  ".to_owned()
        + &config.gene_police_size.to_string()+"px serif; fill: red; }" );
    document.append(style);
    let mut g = Element::new("g");
    for  index in &tree.arena {
        let _parent =  match index.parent {
            Some(p) => {
                let n = &tree.arena[p];
                let chemin = match index.is_a_transfert {
                true => {get_chemin_carre(index.x,index.y,n.x,n.y,gene_color.to_string(),
                         config.gene_opacity.to_string(),true)},
                false => {get_chemin_carre(index.x,index.y,n.x,n.y,gene_color.to_string(),
                         config.gene_opacity.to_string(),false)},
                };
                if tree.arena[p].visible  {
                    g.append(chemin);
                }
                0
            },
            None => {-1},
        };
        let  event = &index.e;
        match event {
            Event::Leaf        =>  g.append(get_carre(index.x,index.y,2.0,"red".to_string(),
                                    config.gene_opacity.to_string())),
            Event::Duplication =>  g.append(get_carre(index.x,index.y,SQUARESIZE,
                                    gene_color.to_string(),config.gene_opacity.to_string())),
            Event::Loss =>  {
                let mut cross = get_cross(index.x,index.y,4.0,gene_color.to_string(),
                                    config.gene_opacity.to_string());
                cross.assign("transform","rotate(45 ".to_owned()+&index.x.to_string()
                + " "+&index.y.to_string()+")");
                g.append(cross);
            },
            Event::BranchingOut =>  {
                let mut diamond = get_carre(index.x,index.y,12.0,"orange".to_string(),
                    config.gene_opacity.to_string());
                diamond.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                     + " " + &index.y.to_string() + ")" );
                g.append(diamond);
            },

            _   => if index.visible { g.append(get_circle(index.x,index.y,3.0,gene_color.to_string(),
                                        config.gene_opacity.to_string()))},
        };
        match index.is_a_transfert {
            true => { g.append(get_triangle(index.x,index.y - 6.0,12.0,"yellow".to_string(),
                                            config.gene_opacity.to_string())) },
            false => {},
        };
        // Display name
        if index.visible {
            let mut element = Element::new("text");
            element.assign("x", index.x-5.0);
            element.assign("y", index.y+10.0);
            element.assign("class", "gene");
            let txt  = Text::new(&index.name);
            element.append(txt);
            element.assign("transform","rotate(90 ".to_owned()+&(index.x - 5.0).to_string()
            + ","+&(index.y + 10.0).to_string()+")");
            match tree.is_leaf(index.idx) {
                true => g.append(element),
                _   =>  match options.gene_internal {
                            true =>  g.append(element),
                            false => {},
                        },
            };

            // Display support
            let mut element = Element::new("text");
            element.assign("x", index.x-15.0);
            element.assign("y", index.y+10.0);
            element.assign("class", "support");
            let txt  = Text::new(&index.support);
            element.append(txt);
            element.assign("transform","rotate(90 ".to_owned()+&(index.x - 15.0).to_string()
            + ","+&(index.y + 10.0).to_string()+")");
            match tree.is_leaf(index.idx) {
                true => {},
                _   =>  match options.support {
                            true =>  g.append(element),
                            false => {},
                        },
            };
        }
    }
    let mut transfo: String = "translate(  ".to_owned();
    transfo.push_str(&( x_viewbox).to_string());
    transfo.push_str(" ");
    transfo.push_str(&((width_svg  + y_viewbox)).to_string());
    transfo.push_str(") rotate(-90 0 0 ) ");
    match options.rotate {
        true => g.assign("transform",transfo),
        false => {}
    };
    document.append(g);
    svg::save(name, &document).unwrap();
}

pub fn get_longest_name_mul(gene_trees:&mut std::vec::Vec<ArenaTree<String>>) -> usize {
    let mut max = 0;
    for tree in gene_trees {
        let gene_max = get_longest_name(tree);
        if gene_max > max {
            max = gene_max;
        }
    }
    max
}
pub fn get_longest_name(gene_tree:&ArenaTree<String>) -> usize {
    let mut max = 0;
    for node in &gene_tree.arena {
        if node.name.len() > max {
            max = node.name.len();
        }
    }
    max
}
/// Draw a svg pipe species tree and  several gene trees inside it
pub fn draw_sptree_gntrees (
    sp_tree: &mut ArenaTree<String>,                    // species tree
    gene_trees:&mut std::vec::Vec<ArenaTree<String>>,   // gene trees
    name: String,                                       // output file
    options: &Options,                                  // drawing options
    config: &Config,                                    // svg configuration
    transfers: &std::vec::Vec<(String,String)>          // additional transfers
    ) {
    let largest_x = sp_tree.get_largest_x() * 1.0 + 0.0 ;
    let largest_y = sp_tree.get_largest_y() * 1.0 + 0.0 ;
    let smallest_x = sp_tree.get_smallest_x() * 1.0 + 0.0 ;
    let smallest_y = sp_tree.get_smallest_y() * 1.0 + 0.0 ;
    let width_svg = largest_x - smallest_x+ 1.0 * BLOCK;
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
    let  mut document = match options.rotate {
        true => Document::new()
                .set("width",height_svg  )
                .set("height",width_svg  )
                .set("viewBox", (x_viewbox,y_viewbox,
                    height_svg + 2.0 *BLOCK ,width_svg + 2.0 *BLOCK )),
        false => Document::new()
                .set("width",width_svg  )
                .set("height",height_svg  )
                .set("viewBox", (y_viewbox,x_viewbox,width_svg + 0.5 *BLOCK ,height_svg + 2.0 *BLOCK )),
    };
    let style = Style::new(".species { font: italic ".to_owned()
        + &config.species_police_size.to_string()+"px serif; fill: "
        + &config.species_police_color.to_string()+"; }");
    document.append(style);
    let mut g = Element::new("g");
    // Dessine l'arbre d'espece
    for index in &sp_tree.arena {
        // Dessine le tuyeau
        match index.parent {
            Some(p) => {
                let n = &sp_tree.arena[p];
                let chemin = get_chemin_sp(index.x, index.y,
                                           index.width/2.0, index.height/2.0,
                                           n.x, n.y,
                                           n.width/2.0, n.height/2.0,
                                           config.species_color.to_string(),
                                           config.species_opacity.to_string());
                if sp_tree.arena[p].visible {
                    g.append(chemin)
                };
                if sp_tree.is_leaf(index.idx) {
                    // Set the y value of the pipe leaf ro the highest value of the y gene leaves
                    let mut max_gene_y =  index.y;
                    for (gene_index, gene_node ) in &index.nodes {
                        let gene_y = gene_trees[*gene_index].arena[*gene_node].y;
                        if  gene_trees[*gene_index].arena[*gene_node].e != Event::Loss {
                            if gene_y >  max_gene_y {
                                max_gene_y = gene_y;
                            }
                        }
                    };
                    if max_gene_y == index.y {
                         max_gene_y = max_gene_y +index.height / 2.0;
                    }
                    let chemin = close_chemin_sp(index.x, index.y,
                                                 index.width/2.0, max_gene_y - index.y,
                                                 config.species_color.to_string(),
                                                 config.species_opacity.to_string());
                    if sp_tree.arena[p].visible {
                        g.append(chemin)
                    };
                }
                match  index.e {
                    Event::Loss => {
                        let chemin = get_chemin_sp(index.x, index.y,
                                                   index.width/2.0, index.height/2.0,
                                                   n.x, n.y,
                                                   n.width/2.0, n.height/2.0,
                                                   "black".to_string(),
                                                   config.species_opacity.to_string());
                        g.append(chemin);
                        let chemin = close_chemin_sp(index.x, index.y,
                                                     index.width/2.0, index.height/2.0,
                                                     "black".to_string(),
                                                     config.species_opacity.to_string());
                        g.append(chemin);

                    }
                    Event::Duplication => {
                        // println!("Duplication!!");
                        let carre = get_carre(index.x,index.y,1.5 * index.width,
                            config.species_color.to_string(),config.species_opacity.to_string());
                        g.append(carre);

                    }
                    _=> {},
                }
                match  index.is_a_transfert {
                    true => {
                        let chemin = get_chemin_sp(index.x, index.y,
                                                   index.width/2.0, index.height/2.0,
                                                   n.x, n.y,
                                                   n.width/2.0, n.height/2.0,
                                                   "green".to_string(),
                                                   config.species_opacity.to_string());
                        g.append(chemin);
                        if sp_tree.is_leaf(index.idx) {
                            let chemin = close_chemin_sp(index.x, index.y,
                                                         index.width/2.0, index.height/2.0,
                                                        "green".to_string(),
                                                         config.species_opacity.to_string());
                            g.append(chemin);
                        }



                    },
                    false => {},
                }
            },
            None => {},
        };
        let mut element = Element::new("text");
        // Affiche le texte associe au noeud
        match sp_tree.is_leaf(index.idx) {
            true => {
                element.assign("x", index.x-15.0);
                element.assign("y", index.y - index.width /2.0 - 10.0);
                element.assign("class", "species");
                let txt  = Text::new(&index.name);
                element.append(txt);
                element.assign("transform","rotate(90 ".to_owned() + &index.x.to_string()
                + "," + &index.y.to_string() + ")" );
                if index.visible {
                    g.append(element);
                }
            },
            false => {
                match options.species_internal {
                    true => {
                        element.assign("x", index.x+15.0);
                        element.assign("y", index.y+20.0);
                        element.assign("class", "species");
                        let txt  = Text::new(&index.name);
                        element.append(txt);
                        element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()
                        + "," + &index.y.to_string() + ")" );
                        if index.visible {
                            g.append(element);
                        }
                    },
                    false => {},
                };
            },
        };
     }
     let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
     let mut idx_rcgen = 0;
     // Boucle sur les arbres de genes
     loop {
         let base_couleur = match &idx_rcgen % 6 {
             5 => Color::Orange,
             0 => Color::Blue,
             1 => Color::Purple,
             2 => Color::Green,
             3 => Color::Red,
             4 => Color::Yellow,
             _ => Color::Monochrome, // Jamais
         };
        let gene_color = RandomColor::new()
            .hue(base_couleur)
            .luminosity(Luminosity::Bright) // Optional
            .alpha(1.0) // Optional
            .to_rgb_string(); //
        let style = Style::new(".gene_".to_owned()+&idx_rcgen.to_string()
            + " { font: "+ &config.gene_police_size.to_string()+"px serif; fill:"
            + &gene_color.to_string() + "; }" );
        document.append(style);
        for  index in &gene_trees[idx_rcgen].arena {
            // Dessine le chemin du noeud a son pere
            match index.parent {
                 Some(p) => {
                     // Attention ici je verifie que le noeud parent n'est pas virtuel.
                     // Si c'est le cas, le vrai parent n'est pas le noeud virtuel
                     // mais le pere de celui ci

                     // let n = &gene_trees[idx_rcgen].arena[p];
                     let n = match gene_trees[idx_rcgen].arena[p].virtualsvg {
                         true =>  {
                             let real_p =  gene_trees[idx_rcgen].arena[p].parent;
                             match real_p {
                                 Some(r_p) =>  &gene_trees[idx_rcgen].arena[r_p],
                                  None => {panic!("lol")},
                             }
                             // &gene_trees[idx_rcgen].arena[p]
                         },
                         false => &gene_trees[idx_rcgen].arena[p],
                     };
                     // debug : ca ca marche
                    let n = &gene_trees[idx_rcgen].arena[p];
                     // La forme du chemin depend de l'evenement
                     let chemin = match index.is_a_transfert {
                        true => {
                            // println!("DEBUG 3 ====> TRANSFERT  {:?}",index);
                            // Si  flag thickness, les transfers sont affiches plus tard,
                            // selon leur redondance
                            let transfer_opacity = match options.thickness_flag {
                                    true => "0.0".to_string(),
                                    false => config.gene_opacity.to_string(),
                            };

                            // Verifie que le parent est bien un branchingout
                            match n.e {
                                Event::BranchingOut => get_chemin_transfer(index.x,index.y,
                                        n.x,n.y,
                                        gene_color.to_string(),
                                        transfer_opacity,
                                        config.bezier.to_string().parse::<f32>().unwrap(),
                                        1),
                                Event::BifurcationOut => get_chemin_transfer(index.x,index.y,
                                        n.x,n.y,
                                        gene_color.to_string(),
                                        transfer_opacity,
                                        config.bezier.to_string().parse::<f32>().unwrap(),
                                        2),
                                _ => {
                                    println!("DEBUG 3 ====> IS VIRTUAL  {:?}",n.virtualsvg);
                                    if n.virtualsvg {
                                        let ppp = n.parent.expect("ERROR");
                                        let nnn = &gene_trees[idx_rcgen].arena[ppp];

                                        get_chemin_transfer(index.x,index.y,
                                                nnn.x,nnn.y,
                                                gene_color.to_string(),
                                                0.0.to_string(),
                                                config.bezier.to_string().parse::<f32>().unwrap(),
                                                1)
                                        // panic!("lol");
                                    }
                                    else {
                                        panic!("Wrong recPhyloXML feature in tree # {}.The father node should be BranchingOut or BifurcationOut, but I found a {:?} Father node: {:?}\nCurrent node: {:?}",idx_rcgen,n.e,n,index);
                                    }
                                },
                            }
                        },
                        false => {
                            // Vérifie que le noeud n'est pas associé à FREE_LIVING
                            if index.location != "FREE_LIVING".to_string() {
                                get_chemin_carre(index.x,index.y,n.x,n.y ,gene_color.to_string(),
                                config.gene_opacity.to_string(),false)
                            }
                            else {
                                // Le noeud est  associé à FREE_LIVING
                                // Calcule l'opacite de l'arbre free living
                                let free_opacity =
                                    config.gene_opacity.parse::<f32>().unwrap() / 2.0 ;
                                if n.location == "FREE_LIVING".to_string() {
                                    // La branche est dans l'arbre free living
                                    get_chemin_carre(index.x,index.y,n.x,n.y,
                                        gene_color.to_string(),free_opacity.to_string(),false)
                                }
                                else {
                                    // La branche  est entre la racine de l'arbre free living
                                    // et le reste
                                    get_chemin_transfer(index.x,index.y,
                                        n.x,n.y,
                                        gene_color.to_string(),
                                        free_opacity.to_string(),
                                        config.bezier.to_string().parse::<f32>().unwrap(),
                                        0)
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
                    let triangle=get_triangle(index.x,index.y-SQUARESIZE,SQUARESIZE,
                        gene_color.to_string(),config.gene_opacity.to_string());
                    g.append(triangle);
                 },
             };
             // Dessine le symbole associe au noeud
             let  event = &index.e;
             match event {
                 Event::Leaf        =>  g.append(get_carre(index.x,index.y,1.0,"red".to_string(),
                                                            config.gene_opacity.to_string())),
                 Event::Duplication =>  g.append(get_carre(index.x,index.y,SQUARESIZE,
                                                 gene_color.to_string(),
                                                 config.gene_opacity.to_string())),
                 Event::Loss => {
                     let mut cross = get_cross(index.x,index.y,2.0,gene_color.to_string(),
                                                config.gene_opacity.to_string());
                     cross.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                     + " " + &index.y.to_string() + ")" );
                     g.append(cross);
                 },
                // Normalement il ny' a pas d event transferBack : comme il est toujour associé
                // a un autre event,c'est ce dernier qui est stocké dans le champs "e"
                // Par contre le champs "is_a_transfert" indique si le noeud prvient d'un transfer
                Event::BranchingOut  =>  {
                    let mut diamond = get_carre(index.x,index.y,4.0,gene_color.to_string(),
                                                config.gene_opacity.to_string());
                    diamond.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                    + " " + &index.y.to_string() + ")" );
                    g.append(diamond);
                    },
                // Est ce que BifurcationOut existe encore ???
                Event::BifurcationOut  =>  g.append(get_carre(index.x,index.y,5.0,
                                                    "yellow".to_string(),
                                                    config.gene_opacity.to_string())),
                Event::Speciation  =>  g.append(get_circle(index.x,index.y,3.0,
                                                            gene_color.to_string(),
                                                            config.gene_opacity.to_string())),
                _ =>  {},
            };
            // Affiche le texte associe au noeud
            match event {
                Event::Leaf        => {
                    let mut element = Element::new("text");
                    element.assign("x", index.x - 5.0);
                    element.assign("y", index.y + 15.0);
                    element.assign("class", "gene_".to_owned()+&idx_rcgen.to_string());
                    let txt  = Text::new(&index.name);
                    element.append(txt);
                    element.assign("transform","rotate(90 ".to_owned() + &(index.x - 5.0).to_string()
                    + "," + &(index.y + 15.0).to_string() + ")" );
                    g.append(element);
                    },
                _ => {
                    match options.gene_internal {
                        true =>  {
                            let mut element = Element::new("text");
                            element.assign("x", index.x+10.0);
                            element.assign("y", index.y+0.0);
                            element.assign("class", "gene_".to_owned() + &idx_rcgen.to_string());
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
      idx_rcgen += 1;
      if idx_rcgen == nb_gntree {
          break;
      }
  }
  // Ajoute les Transfers surnumeraire
  // Analyse l'abondance des transferts
  let mut unique_transfers: std::vec::Vec<(String,String)> =  vec![];
  let mut scores: std::vec::Vec<usize> =  vec![];
  let mut score_max = 1;
  if options.thickness_disp_score {
      let style = Style::new(".rouge { font: ".to_owned()+ &config.gene_police_size.to_string()+"px serif; fill:red ; }" );
      document.append(style);
  }
  for transfer in transfers {
      let index = unique_transfers.iter().position(|r| r == transfer);
      match index {
          None => {
              unique_transfers.push(transfer.clone());
              scores.push(1)},
          Some(i) => {
              scores[i] = scores[i]+ 1;
              if scores[i] > score_max {
                  score_max = scores[i];
              }
          },
      }
  }
  let mut  i_trans = 0;
  while i_trans < unique_transfers.len() {
      let (end,start) = &unique_transfers[i_trans];
      let score = scores[i_trans];
      info!("[draw_sptree_gntrees] Additional transfer {}: {}=>{} Score = {}",i_trans,start,end,score);
      i_trans = i_trans + 1;
      if score > options.thickness_thresh {
          info!("[draw_sptree_gntrees] Selecting additional transfer {:?} {:?}",start,end);
          let start_node = sp_tree.get_index(start.to_string()).expect("arg");
          let end_node = sp_tree.get_index(end.to_string()).expect("arg");
          let opacity = score * 100/ score_max ;
          let opacity = opacity as f32 / 100.0 ;
          let opacity = opacity.to_string();
          info!("[draw_sptree_gntrees] Adding additional transfer {:?} {:?} with opacity {}",start,end,opacity);
          let chemin = get_chemin_transfer(sp_tree.arena[start_node].x,
              sp_tree.arena[start_node].y,
              sp_tree.arena[end_node].x,
              sp_tree.arena[end_node].y,
              "red".to_string(),
              opacity,
              config.bezier.to_string().parse::<f32>().unwrap(),
              0
          );
          g.append(chemin);

          // Affichage du score
          if options.thickness_disp_score {
              let bez_y = config.bezier.to_string().parse::<f32>().unwrap()  * BLOCK;
              // Point de controle de la courbe de Bezier
              let controle_x = (sp_tree.arena[start_node].x + sp_tree.arena[end_node].x) / 2.0  ;
              let controle_y = (sp_tree.arena[start_node].y + sp_tree.arena[end_node].y) / 2.0
                - bez_y - ( score.to_string().len() as f32 * &config.gene_police_size.parse::<f32>().unwrap()) ;
              let mut element = Element::new("text");
              element.assign("x", controle_x - 0.0);
              element.assign("y", controle_y + 0.0);
              element.assign("class", "rouge");
              let txt  = Text::new(score.to_string());
              element.append(txt);
              match options.rotate {
                  false => {}
                  true => element.assign("transform","rotate(90 ".to_owned()+ &controle_x.to_string() + "," +
                                 &controle_y.to_string()+")"),
              };
              g.append(element);
           }
      }
  }
  let mut transfo: String = "translate(  ".to_owned();
  transfo.push_str(&( x_viewbox).to_string());
  transfo.push_str(" ");
  transfo.push_str(&((width_svg  + y_viewbox)).to_string());
  transfo.push_str(") rotate(-90 0 0 ) ");
  match options.rotate {
      true => g.assign("transform",transfo),
      false => {}
  };
  document.append(g);
  svg::save(name, &document).unwrap();
}

#[allow(dead_code)]
// Draw a frame
pub fn get_cadre (x: f32, y:f32,w:f32,h:f32, c:String) -> Path {
    let data = Data::new()
    .move_to((x , y))
    .line_by((w, 0.0 ))
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

// Draw a square  of size s at x,y
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

// Draw a triangle  of size s at x,y
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

// Draw a circle  of size s at x,y
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

// Draw a cross  of size s at x,y
pub fn get_cross (x: f32, y:f32, s:f32, c:String, o:String) -> Path {
    let data = Data::new()
    .move_to((x*1.0 , y*1.0 -s*2.0))
    .line_by((0, s*4.0))
    .move_to((x*1.0 -s*2.0, y*1.0 ))
    .line_by((s*4.0, 0));
    let fill = c.clone();
    let path = Path::new()
    .set("fill", fill)
    .set("stroke", c)
    .set("opacity", o)
    .set("stroke-width", s*1.0)
    .set("d", data);
    path
}

// Draw a square path between x1,y1 ad x2,y2
pub fn get_chemin_carre (x1: f32, y1:f32,x2: f32, y2:f32, c:String, o:String, stroke:bool)-> Path {
    let data = Data::new()
    .move_to((x1*1.0, y1*1.0))
    .line_to((x1*1.0, y2*1.0))
    .line_to((x2*1.0, y2*1.0));
    let path = Path::new()
    .set("fill", "none")
    .set("stroke", c)
    .set("opacity", o)
    .set("stroke-width", GTHICKNESS);
    let path = match stroke {
        true => path.set("stroke-dasharray","1, 1"),
        false => path,
    };
    let path  = path.set("d", data);
    path
}

// Draw a transfer path between x1,y1 ad x2,y2
pub fn get_chemin_transfer (x1: f32, y1:f32,x2: f32, y2:f32, c:String, o:String,b:f32,
    stroke:i32) -> Path {
    // Arrivee du point: un peu avant pour dessiner la fleche
    let initial_y1 = y1 ;
    let y1 = y1 - PIPEBLOCK;
    // Courbure de la courbe de Bezier
    let bez_y = b  * BLOCK;
    // let bez_y = 20.0;
    // Point de controle de la courbe de Bezier
    let controle_x = (x1 + x2) / 2.0 ;
    let controle_y = (y1 + y2) / 2.0 - bez_y ;
    // ligne horizontale
    let data = "M".to_owned() + &x1.to_string() +" " + &initial_y1.to_string()
             +" L "+ &x1.to_string() + " " + &y1.to_string();
    // fleche
    let data = data.to_owned()+ "M" + &x1.to_string() + " "
              + &(initial_y1- PIPEBLOCK / 4.0).to_string() + "L "
              + &(x1 - PIPEBLOCK / 4.0 ).to_string() + " "
              + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
    let data = data.to_owned()+ "M"+&x1.to_string() + " "
               + &(initial_y1- PIPEBLOCK / 4.0).to_string() + "L "
               + &(x1 + PIPEBLOCK / 4.0 ).to_string() + " "
               + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
    // bezier
    let data = data.to_owned() + "M" + &x1.to_string() + " " + &y1.to_string()
               + " Q " + &controle_x.to_string() + " " + &controle_y.to_string()
               + " " + &x2.to_string() + " " + &y2.to_string();
    let path = Path::new()
    .set("fill", "none")
    .set("stroke", c)
    .set("opacity", o)
    .set("stroke-width", GTHICKNESS);
    let path = match stroke {
        2 => path.set("stroke-dasharray","4, 1"),
        1 => path.set("stroke-dasharray","1, 1"),
        0 => path,
        _ => path.set("stroke-dasharray","1, 1"),
    };
    let path  = path.set("d", data);
    path
}

// Draw a square pipe path between x1,y1 ad x2,y2
pub fn get_chemin_sp (x1: f32, y1:f32, width1:f32, height1:f32, x2: f32, y2:f32,
                      width2:f32, height2:f32, c:String, o:String ) -> Path {
    if x1 < x2 {
        let data = Data::new()
        .move_to((x1 - width1, y1 - height1 + (STHICKNESS / 2)  as f32))
        .line_to((x1 - width1, y2 - height2))
        .line_to((x2 - width2 - (STHICKNESS / 2)  as f32, y2 - height2))
        .move_to((x1 + width1, y1 - height1 + (STHICKNESS / 2)  as f32 ))
        .line_to((x1 + width1, y2 + height2))
        .line_to((x2, y2 + height2));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
    }
    else {
        let data = Data::new()
        .move_to((x1 + width1, y1 - height1 + (STHICKNESS / 2)  as f32 ))
        .line_to((x1 + width1, y2 - height2))
        .line_to((x2 + width2 + (STHICKNESS / 2)  as f32, y2 - height2))
        .move_to((x1 - width1, y1 - height1 + (STHICKNESS / 2)  as f32))
        .line_to((x1 - width1, y2 + height2))
        .line_to((x2, y2 + height2));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
    }
}

// Finish  the drawing of species tree at the leaves level.
pub fn close_chemin_sp (x1: f32, y1:f32, width1:f32, height1:f32, c:String, o:String ) -> Path {
        let data = Data::new()
        .move_to((x1 - width1, y1 - height1))
        .line_to((x1 - width1, y1 + 1.0 * height1))
        .line_to((x1 + width1, y1 + 1.0 * height1))
        .line_to((x1 + width1, y1 - height1));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
}

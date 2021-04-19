//! # light_phylogeny
//!
//! `light_phylogeny` is a set of methods and functions dedicated  to phylogeny.


mod arena;
pub use self::arena::Options;
pub use self::arena::Config;
pub use self::arena::Event;
pub use self::arena::Noeud;
pub use self::arena::ArenaTree;

pub use self::arena::add_child;
// pub use self::arena::bilan_mappings;
// pub use self::arena::center_gene_nodes;
// pub use self::arena::check_contour_postorder;
// pub use self::arena::check_for_obsolete;
pub use self::arena::check_is_rooted;
// pub use self::arena::check_vertical_contour_postorder;
// pub use self::arena::cladogramme;
// pub use self::arena::find_left_right;
// pub use self::arena::find_rgtrees;
// pub use self::arena::find_sptree;
// pub use self::arena::get_contour_left;
// pub use self::arena::get_contour_right;
// pub use self::arena::get_maxdepth;
// pub use self::arena::knuth_layout;
pub use self::arena::lca;
// pub use self::arena::map_gene_trees;
// pub use self::arena::map_species_trees;
pub use self::arena::move_child;
// pub use self::arena::move_dupli_mappings;
pub use self::arena::newick2tree;
// pub use self::arena::node_xpos;
// pub use self::arena::node_ypos;
// pub use self::arena::push_down;
// pub use self::arena::push_right;
pub use self::arena::real_length;
pub use self::arena::reset_pos;
// pub use self::arena::set_leaves_to_bottom;
// pub use self::arena::set_middle_postorder;
// pub use self::arena::set_species_width;
// pub use self::arena::shift_mod_xy;
pub use self::arena::summary;
// pub use self::arena::xml2tree;
//
mod drawing;
// pub use self::drawing::close_chemin_sp;
pub use self::drawing::draw_sptree_gntrees;
pub use self::drawing::draw_tree;
// pub use self::drawing::get_carre;
// pub use self::drawing::get_circle;
// pub use self::drawing::get_chemin_carre;
// pub use self::drawing::get_chemin_sp;
// pub use self::drawing::get_chemin_transfer;
// pub use self::drawing::get_cross;
// pub use self::drawing::get_triangle;
mod building;
pub use self::building::phyloxml_processing;
pub use self::building::recphyloxml_processing;
pub use self::building::read_newick;
pub use self::building::read_phyloxml;
pub use self::building::read_recphyloxml;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  check_set_event() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let index = tree.new_node(test);
        let node = &mut tree.arena[index];
        node.set_event(Event::Duplication);
        assert_eq!(node.e,Event::Duplication);
    }
    #[test]
    fn  check_x_noref() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let index = tree.new_node(test);
        let node = &mut tree.arena[index];
        node.set_x_noref(10.0);
        let x = node.x;
        assert_eq!(x, 10.0);
    }

    #[test]
    fn  check_lca() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let root = tree.new_node("root".to_string());
        let a1 = tree.new_node("a1".to_string());
        let a2 = tree.new_node("a2".to_string());
        let a = tree.new_node("a".to_string());
        let b = tree.new_node("b".to_string());
        let c = tree.new_node("c".to_string());
        let d = tree.new_node("d".to_string());
        // Set hierarchy
        //  a1 and a2 are children of a
        tree.arena[a1].parent = Some(a);
        tree.arena[a2].parent = Some(a);
        tree.arena[a].children.push(a1);
        tree.arena[a].children.push(a2);
        // a and b are children of c
        tree.arena[a].parent = Some(c);
        tree.arena[b].parent = Some(c);
        tree.arena[c].children.push(a);
        tree.arena[c].children.push(b);
        // c and d are children of root
        tree.arena[c].parent = Some(root);
        tree.arena[d].parent = Some(root);
        tree.arena[root].children.push(c);
        tree.arena[root].children.push(d);
        let lca_test = lca(&mut tree,a1,b);
        assert_eq!(lca_test,c);
    }
    #[test]
    fn  check_read_newick() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        read_newick("examples/newick.txt".to_string(),&mut tree);
        let options: Options = Options::new();
        let config: Config = Config::new();
        phyloxml_processing(&mut tree, &options, &config, "toto.svg".to_string() ) ;
    }
    #[test]
    fn  check_read_recphylo() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let options: Options = Options::new();
        let config: Config = Config::new();
        let _root = tree.new_node("Root".to_string());
        phyloxml_processing(&mut tree, &options, &config, "toto".to_string() ) ;
        println!("Tree {:?}",tree);
    }
    #[test]
    fn  check_read_phylo() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let options: Options = Options::new();
        let config: Config = Config::new();
        let _root = tree.new_node("Root".to_string());
        phyloxml_processing(&mut tree, &options, &config, "toto".to_string() ) ;
        println!("Tree {:?}",tree);
    }
    #[test]
    fn  check_get_index() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let index = tree.new_node("test".to_string());
        tree.arena[index].name = "Test".to_string();
        let test = tree.get_index("Test".to_string()).expect("Error in test check_get_index");
        assert_eq!(test,index);
    }
}

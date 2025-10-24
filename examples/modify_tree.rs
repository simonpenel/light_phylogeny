use light_phylogeny::{
    add_child, move_child, phyloxml_processing, reset_pos, summary, ArenaTree, Config, Event,
    Options,
};
fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    let config: Config = Config::new();

    // Create a new node root
    let root = tree.new_node("root".to_string());
    // Create  new nodes
    let a1 = tree.new_node("a1".to_string());
    let a2 = tree.new_node("a2".to_string());
    let a = tree.new_node("a".to_string());
    let b1 = tree.new_node("b1".to_string());
    let b2 = tree.new_node("b2".to_string());
    let b = tree.new_node("b".to_string());
    let c = tree.new_node("c".to_string());
    let d = tree.new_node("d".to_string());
    println!("Initial tree :");
    summary(&mut tree);

    // Set names
    tree.arena[root].name = "MyRoot".to_string();
    tree.arena[a].name = "Gene A".to_string();
    tree.arena[a1].name = "Gene A1".to_string();
    tree.arena[a2].name = "Gene A2".to_string();
    tree.arena[b].name = "Gene B".to_string();
    tree.arena[b1].name = "Gene B1".to_string();
    tree.arena[b2].name = "Gene B2".to_string();
    tree.arena[c].name = "Gene C".to_string();
    tree.arena[d].name = "Gene D".to_string();
    println!("");
    println!("Tree after name attribution:");
    summary(&mut tree);
    // Set hierarchy
    //  a1 and a2 are children of a
    add_child(&mut tree, a, a1);
    add_child(&mut tree, a, a2);
    //  a1 and a2 are children of a
    add_child(&mut tree, b, b1);
    add_child(&mut tree, b, b2);
    // a and b are children of c
    add_child(&mut tree, c, a);
    add_child(&mut tree, c, b);
    // c and d are children of root
    add_child(&mut tree, root, c);
    add_child(&mut tree, root, d);

    println!("");
    println!("Tree after hierarchy attribution:");
    summary(&mut tree);
    // Display internal nodes
    options.gene_internal = true;

    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "modify_tree_ini.svg".to_string(),
    );
    // knuth_layout(&mut tree,root, &mut 1);
    // cladogramme(&mut tree);
    // check_contour_postorder(&mut tree, root);
    // shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
    // set_middle_postorder(&mut tree, root);
    // draw_tree(&mut tree,"modify_tree_ini.svg".to_string(),&options,&config);

    println!("Add a loss to C");
    let loss = tree.new_node("loss".to_string());
    tree.arena[loss].name = "Loss".to_string();
    tree.arena[loss].e = Event::Loss;
    add_child(&mut tree, c, loss);

    println!("Add a node up to  B");
    let add = tree.new_node("add".to_string());
    tree.arena[add].name = "Added up to  B".to_string();
    println!("Move A to new node ");
    move_child(&mut tree, a, add);
    println!("Move B to new node ");
    move_child(&mut tree, b, add);
    println!("Move  new node to C ");
    add_child(&mut tree, c, add);

    println!("Tree after hierarchy modification:");
    summary(&mut tree);
    reset_pos(&mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "modify_tree_mod.svg".to_string(),
    );

    // knuth_layout(&mut tree,root, &mut 1);
    // cladogramme(&mut tree);
    // check_contour_postorder(&mut tree, root);
    // shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
    // set_middle_postorder(&mut tree, root);
    // draw_tree(&mut tree,"modify_tree_mod.svg".to_string(),&options,&config);
    println!("Please open output files  'modify_tree_ini.svg' and 'modify_tree_mod.svg' with your browser");
    println!("OK.");
}

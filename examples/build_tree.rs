use light_phylogeny::{ArenaTree,Options,Config,Event,phyloxml_processing,summary};
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
    tree.arena[c].name = "Gene C".to_string();
    tree.arena[d].name = "Gene D".to_string();
    println!("");
    println!("Tree after name attribution:");
    summary(&mut tree);
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
    println!("");
    println!("Tree after hierarchy attribution:");
    summary(&mut tree);

    // set duplication
    tree.arena[a].e = Event::Duplication;
    println!("");
    println!("Event associated to gene {} ({}) : {:?}",a,tree.arena[a].name,tree.arena[a].e);
    // Display internal nodes
    options.gene_internal = true ;
    phyloxml_processing(&mut tree, &options, &config,"build_tree.svg".to_string());
    println!("Please open output file 'build_tree.svg' with your browser");
    println!("OK.");
}

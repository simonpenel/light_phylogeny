
[![light_phylogeny at crates.io](https://img.shields.io/crates/v/light_phylogeny.svg)](https://crates.io/crates/light_phylogeny)
[![light_phylogeny at docs.rs](https://docs.rs/light_phylogeny/badge.svg)](https://docs.rs/light_phylogeny)
[![light_phylogeny at docs.rs](https://github.com/simonpenel/light_phylogeny/actions/workflows/rust.yml/badge.svg)](https://github.com/simonpenel/light_phylogeny/actions/workflows/rust.yml)
[![light_phylogeny at docs.rs](https://github.com/simonpenel/light_phylogeny/actions/workflows/example.yml/badge.svg)](https://github.com/simonpenel/light_phylogeny/actions/workflows/example.yml)


# "light_phylogeny" is a rust library dedicated to phylogeny.

Homepage : https://github.com/simonpenel/light_phylogeny/wiki

You may :

- Read, build, modify and displays as svg reconciled phylogenetic trees.

- Drawing reconciled phylogenetic trees allowing 1, 2 or 3 reconciliation levels

- Build a svg representation of a phylogenetic reconciled (or not) tree with events (loss, duplication, speciation, transfer).

- Read a recphyloxml file:  create a svg representation of the  gene tree(s) and the associated species tree.

- Read 2 nested recphyloxml files:  create svg representations of the  gene tree(s), the associated symbiot tree(s) and the associated species tree. 

- Read a newick or phyloxml file: create a svg representation of the gene tree only.

You may use light_phylogeny functions and methods to build, manipulate, modify or draw phylogenetic trees.

You may use the "thirdkind" software  https://github.com/simonpenel/thirdkind  based on 'light_phylogeny' to represent  trees with 1, 2 or 3 reconciliation levels

Keywords:  phylogeny, reconciled trees, phylogenetic trees

# Formats:

phyloXML, recPhyloXML, rooted newick (NHX balises will not be considered).

# Output examples

single gene reconciliation with species tree:

https://raw.githubusercontent.com/simonpenel/thirdkind/74b9c84a5b2ed84ff5230fc3a52af856b9aba53d/output_examples/thirdkind_example1.svg


the same gene reconciliation without the species tree:

https://raw.githubusercontent.com/simonpenel/thirdkind/74b9c84a5b2ed84ff5230fc3a52af856b9aba53d/output_examples/thirdkind_example1_bis.svg


multiple genes reconciliation:

https://raw.githubusercontent.com/simonpenel/thirdkind/70a7a11aa89eda61926c5cabf221f47bb44e3409/output_examples/thirdkind_example4.svg

example with "free living" species:

https://raw.githubusercontent.com/simonpenel/thirdkind/9eb47ce644998164ff56cc68eb765c0c8a24d389/output_examples/thirdkind_fl_example.svg

multiple gene trees reconciliation with redundant transfers. Display only 1 gene tree and the transfers in red with an opacity according to the abundance of the transfer:

https://raw.githubusercontent.com/simonpenel/thirdkind/70a7a11aa89eda61926c5cabf221f47bb44e3409/output_examples/thirdkind_example2.svg


For  more examples, see the "thirdkind" software  https://github.com/simonpenel/thirdkind.


# Using the API:

https://crates.io/crates/light_phylogeny

You may find code  examples in the "examples" directory.

Simple Rust example: read a newick.txt file and creates the svg
```
use light_phylogeny::{ArenaTree,Options,Config,read_newick,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick-clado.svg".to_string());
    println!("Please open output file 'read_newick-clado.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick-real-clado.svg".to_string());
    println!("Please open output file 'read_newick-real-clado.svg' with your browser");
}
```

Some newick examples are available here : https://github.com/simonpenel/light_phylogeny/tree/master/newick_examples

Simple Rust example:build a gene tree, creates the svg, modiy the tree and creates a new svg:

```
use light_phylogeny::{ArenaTree,Options,Config,Event,add_child,move_child,phyloxml_processing,
    summary,reset_pos};
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
    add_child(&mut tree,a,a1);
    add_child(&mut tree,a,a2);
    //  a1 and a2 are children of a
    add_child(&mut tree,b,b1);
    add_child(&mut tree,b,b2);
    // a and b are children of c
    add_child(&mut tree,c,a);
    add_child(&mut tree,c,b);
    // c and d are children of root
    add_child(&mut tree,root,c);
    add_child(&mut tree,root,d);

    println!("");
    println!("Tree after hierarchy attribution:");
    summary(&mut tree);
    // Display internal nodes
    options.gene_internal = true ;

    phyloxml_processing(&mut tree, &options, &config,"modify_tree_ini.svg".to_string());

    println!("Add a loss to C");
    let loss = tree.new_node("loss".to_string());
    tree.arena[loss].name = "Loss".to_string();
    tree.arena[loss].e = Event::Loss;
    add_child(&mut tree,c,loss);

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
    phyloxml_processing(&mut tree, &options, &config,"modify_tree_mod.svg".to_string());

    println!("Please open output files  'modify_tree_ini.svg' and 'modify_tree_mod.svg' with your browser");
    println!("OK.");
}

```
# Code Examples

You may try the codes in the 'examples' directory:

    cargo run --example read_newick

    cargo run --example build_tree

    cargo run --example lca

    cargo run --example modify_tree

Read and display a reconciled tree from a recPhyloXML file:

https://github.com/simonpenel/light_phylogeny/blob/master/examples/read_recphyloxml.rs

# Source documentation

See Rust documentation : https://docs.rs/light_phylogeny/

# The "thirdkind" software

https://github.com/simonpenel/thirdkind


# recPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

recPhyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6198865/

# phyloXML documentation

See: http://www.phyloxml.org/

phyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2774328/

# Under development:
- Possible problem with the obsolete version of recPhyloXML format (speciationLoss is supported, speciationOutLoss and speciationOut are not supported yet)


# Tree drawing algorithms and structures

"Arena" Tree structure  is inspired by the code proposed [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)

Tree drawing algorithms are well explained [here](https://llimllib.github.io/pymag-trees/)  and [here](https://rachel53461.wordpress.com/2014/04/20/algorithm-for-drawing-trees/)

# Licence
CECILL: https://choosealicense.com/licenses/cecill-2.1/

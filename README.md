
[![light_phylogeny at crates.io](https://img.shields.io/crates/v/light_phylogeny.svg)](https://crates.io/crates/light_phylogeny)
[![light_phylogeny at docs.rs](https://docs.rs/light_phylogeny/badge.svg)](https://docs.rs/light_phylogeny)
[![light_phylogeny at docs.rs](https://github.com/simonpenel/light_phylogeny/actions/workflows/rust.yml/badge.svg)](https://github.com/simonpenel/light_phylogeny/actions/workflows/rust.yml)
[![light_phylogeny at docs.rs](https://github.com/simonpenel/light_phylogeny/actions/workflows/example.yml/badge.svg)](https://github.com/simonpenel/light_phylogeny/actions/workflows/example.yml)


Read, build,modify and displays as svg and manipulates reconciled phylogenetic trees.

Trees must be rooted.

Build a svg representation of a phylogenetic reconciled (or not) tree with events (loss, duplication, speciation, transfer).

Read a recphyloxml file:  create a svg representation of the  gene tree(s) and the associated species tree.

Read a newick or phyloxml file: create a svg representation of the gene tree only.

You may use light_phylogeny functions and methods to build, manipulate, modify or draw phylogenetic trees.

Keywords:  phylogeny, reconciled trees, phylogenetic trees

# Formats:

phyloXML, recPhyloXML, rooted newick ( NHX balises will not be considered ).

# Output examples

multiple genes reconciliation recphyloxml:

https://raw.githubusercontent.com/simonpenel/light_phylogeny/9244f3136961f909fd7b33818f0a220e3f32c880/tree2svg.example.svg


single gene reconciliation in recphyloxml:

https://raw.githubusercontent.com/simonpenel/light_phylogeny/9244f3136961f909fd7b33818f0a220e3f32c880/FAM000696_reconciliated_recphyloxml.svg

the same gene reconciliation in phyloxml:

https://raw.githubusercontent.com/simonpenel/light_phylogeny/9244f3136961f909fd7b33818f0a220e3f32c880/FAM000696_reconciliated_xml.svg

# Configuration file:

You may configure some of the features of the svg with the -c option.

The default values are the values of the "config_default.txt" file.

Modify the default values and save it into  "my_config.txt" then type:


# Using the API:

https://crates.io/crates/light_phylogeny

The API functions and methods are tagged as "API" in the Rust package documentation (https://docs.rs/light_phylogeny)

 Semantic Versioning Specification applies only to "API" tagged functions and methods.

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

Simple Rust example: build a gene tree with a duplication and creates the svg
```
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
```
# Code Examples

You may try the codes in the 'examples' directory:

    cargo run --example read_newick

    cargo run --example build_tree

    cargo run --example lca

    cargo run --example modify_tree


# Source documentation

See Rust documentation : https://docs.rs/light_phylogeny/

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

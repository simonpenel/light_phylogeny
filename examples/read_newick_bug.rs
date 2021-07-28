use light_phylogeny::{ArenaTree,Options,Config,read_newick,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_newick("examples/rec0.recphyloxml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_clado.svg".to_string());
    println!("Please open output file 'read_newick_clado.svg' with your browser");

}

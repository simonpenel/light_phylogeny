use light_phylogeny::{ArenaTree,Options,Config,read_phyloxml,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_phyloxml("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml.svg".to_string());
    println!("Please open output file 'read_phyloxml.svg' with your browser");
}

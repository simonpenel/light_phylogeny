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

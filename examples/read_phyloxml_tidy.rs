use light_phylogeny::{ArenaTree,Options,Config,read_phyloxml,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.branch = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real.svg' with your browser");
}

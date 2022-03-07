use light_phylogeny::{ArenaTree,Options,Config,read_newick,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_clado.svg".to_string());
    println!("Please open output file 'read_newick_clado.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.rotate = false;
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_real_landcsape.svg".to_string());
    println!("Please open output file 'read_newick_real_landcsape.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.rotate = false;
    options.scale = 5.0;
    options.branch = true;
    options.tidy = true;
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_real_by5_tidy_landcsape.svg".to_string());
    println!("Please open output file 'read_newick_real_by5_tidy_landcsape.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    // options.rotate = false;
    options.scale = 4.0;
    options.branch = true;
    options.tidy = true;
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_real_by4_tidy.svg".to_string());
    println!("Please open output file 'read_newick_real_by4_tidy.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.rotate = false;
    options.scale = 5.0;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/newick.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_real_by5_landcsape.svg".to_string());
    println!("Please open output file 'read_newick_real_by5_landcsape.svg' with your browser");


    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.rotate = false;
    let config: Config = Config::new();
    read_newick("examples/newick.220.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_220_real_landscape.svg".to_string());
    println!("Please open output file 'read_newick_220_real_landscape.svg' with your browser");



    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 5.0;
    options.support = true;
    let config: Config = Config::new();
    read_newick("examples/newick.220.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_220_real_by5_support.svg".to_string());
    println!("Please open output file 'read_newick_220_real_by5_support.svg' with your browser");


    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 5.0;
    let config: Config = Config::new();
    read_newick("examples/newick.220.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_220_real_by5.svg".to_string());
    println!("Please open output file 'read_newick_220_real_by5.svg' with your browser");



    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    let config: Config = Config::new();
    read_newick("examples/newick.220.txt".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_newick_220_landscape.svg".to_string());
    println!("Please open output file 'read_newick_220_landscape.svg' with your browser");


}

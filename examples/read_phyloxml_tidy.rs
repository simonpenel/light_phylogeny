use light_phylogeny::{ArenaTree,Options,Config,read_phyloxml,phyloxml_processing};

fn main() {
    // landscape
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.gene_internal = true;
    options.rotate = false;
    // options.branch = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf_name.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real.svg' with your browser");

    // landscape + tidy
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.gene_internal = true;
    options.rotate = false;
    options.tidy = true;
    // options.branch = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf_name.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real_tidy.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real_tidy.svg' with your browser");


    // portrait
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.gene_internal = true;
    // options.rotate = false;
    // options.branch = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf_name.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real_portrait.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real_portrait.svg' with your browser");

    // portrait + tidy
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.gene_internal = true;
    // options.rotate = false;
    options.tidy = true;
    // options.branch = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf_name.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real_tidy_portrait.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real_tidy_portrait.svg' with your browser");

}

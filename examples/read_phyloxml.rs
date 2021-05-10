use light_phylogeny::{ArenaTree,Options,Config,read_phyloxml,phyloxml_processing};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml.svg".to_string());
    println!("Please open output file 'read_phyloxml.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_landscape.svg".to_string());
    println!("Please open output file 'read_phyloxml_landscape.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_dist_real.svg".to_string());
    println!("Please open output file 'read_phyloxml_dist_real.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    let config: Config = Config::new();
    read_phyloxml("examples/bcl_2.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_events.svg".to_string());
    println!("Please open output file 'read_phyloxml_events.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 10.0;
    options.support = true;
    let config: Config = Config::new();
    read_phyloxml("examples/ENSGT00390000003602.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_ensembl.svg".to_string());
    println!("Please open output file 'read_phyloxml_ensembl.svg' with your browser");

}

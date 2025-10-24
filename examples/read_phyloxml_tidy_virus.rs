use light_phylogeny::{phyloxml_processing, read_newick, ArenaTree, Config, Options};

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.rotate = false;
    // options.support = true;
    // options.gene_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.rotate = false;
    options.tidy = true;
    // options.support = true;
    // options.gene_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_tidy.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus_tidy.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    // options.rotate = false;
    options.tidy = true;
    options.tidy_leaves_check = true;
    // options.support = true;
    // options.gene_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_portrait_tidy_check.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus_portrait_tidy_check.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    // options.rotate = false;
    options.tidy = true;
    options.tidy_leaves_check = true;
    // options.support = true;
    // options.gene_internal = true;
    // options.branch = true;
    let mut config: Config = Config::new();
    config.gene_police_size = "50.0".to_string();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_portrait_tidy_check_police.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus_portrait_tidy_check_police.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    // options.rotate = false;
    options.tidy = true;
    // options.tidy_leaves_check = true;
    // options.support = true;
    // options.gene_internal = true;
    // options.branch = true;
    let mut config: Config = Config::new();
    config.gene_police_size = "50.0".to_string();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_portrait_tidy_police.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus_portrait_tidy_police.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    // options.rotate = false;
    // options.support = true;
    // options.gene_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_portrait.svg".to_string(),
    );
    println!(
        "Please open output file 'read_phyloxml_dist_real_virus_portrait.svg' with your browser"
    );

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    // options.rotate = false;
    options.tidy = true;
    // options.support = true;
    // options.gene_internal = true;
    options.branch = true;
    let config: Config = Config::new();
    read_newick("examples/virus.nhx".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_virus_portrait_tidy.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_virus_portrait_tidy.svg' with your browser");
}

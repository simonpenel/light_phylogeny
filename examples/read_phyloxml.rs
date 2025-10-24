use light_phylogeny::{phyloxml_processing, read_phyloxml, ArenaTree, Config, Options};
use std::collections::HashMap;

fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    options.gene_internal = true;
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_landscape.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_landscape.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.branch = true;
    options.gene_internal = true;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    options.branch = true;
    options.rotate = false;
    let config: Config = Config::new();
    read_phyloxml("examples/apaf.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_dist_real_landscape.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_dist_real_landscape.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 4.0;
    options.support = true;
    let config: Config = Config::new();
    read_phyloxml("examples/bcl_2.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_events.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_events.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 10.0;
    options.support = true;
    let config: Config = Config::new();
    read_phyloxml("examples/ENSGT00390000003602.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_ensembl.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_ensembl.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 10.0;
    options.support = true;
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("red".to_string());
    let config: Config = Config::new();
    read_phyloxml("examples/ENSGT00390000003602.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_ensembl_color.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_ensembl_color.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.real_length_flag = true;
    options.scale = 10.0;
    options.support = true;
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("red".to_string());
    options.node_colors.push("ENSTRUG00000006177".to_string());
    let config: Config = Config::new();
    read_phyloxml("examples/ENSGT00390000003602.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_ensembl_color_2.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_ensembl_color_2.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.node_colors.push("02_speciation".to_string());
    options.node_colors.push("01_speciation".to_string());
    options.node_colors.push("b1".to_string());
    options.node_colors.push("b2".to_string());
    options.gene_internal = true;
    options.squaresize = 15.0;
    let config: Config = Config::new();
    read_phyloxml("xml_examples/example_dupli.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_example_dupli_color.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_example_dupli_color.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("red".to_string());
    options.gene_colors.push("orange".to_string());
    options.node_colors.push("02_speciation".to_string());
    options.node_colors.push("01_speciation".to_string());
    options.node_colors.push("b1".to_string());
    options.node_colors.push("b2".to_string());
    options.gene_internal = true;
    options.squaresize = 15.0;
    let config: Config = Config::new();
    read_phyloxml("xml_examples/example_dupli.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_example_dupli_color_2.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_example_dupli_color_2.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.gene_colors.push("green".to_string());
    options.gene_colors.push("red".to_string());
    options.gene_colors.push("orange".to_string());
    options.node_colors.push("02_speciation".to_string());
    options.node_colors.push("01_speciation".to_string());
    options.node_colors.push("b1".to_string());
    options.node_colors.push("b2".to_string());
    options.gene_internal = true;
    options.squaresize = 15.0;

    let mut time_line_1 = HashMap::new();
    time_line_1.insert("02_speciation".to_string(), "blue".to_string());
    time_line_1.insert("01_speciation".to_string(), "yellow".to_string());
    options.time_lines.push(time_line_1);
    let mut time_line_2 = HashMap::new();
    time_line_2.insert("0".to_string(), "purple".to_string());
    options.time_lines.push(time_line_2);

    let mut time_line_3 = HashMap::new();
    time_line_3.insert("b1".to_string(), "pink".to_string());
    time_line_3.insert("a1".to_string(), "%circle:green".to_string());
    time_line_3.insert("a2".to_string(), "%circlea:green".to_string());
    time_line_3.insert("c1".to_string(), "%ciracle:green".to_string());
    options.time_lines.push(time_line_3);

    let config: Config = Config::new();
    read_phyloxml("xml_examples/example_dupli.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_example_timelines.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_example_timelines.svg' with your browser");
}

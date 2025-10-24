use light_phylogeny::{phyloxml_processing, read_phyloxml, ArenaTree, Config, Options};

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

    options.collapsed_nodes.push("species_1".to_string());
    options.collapsed_nodes.push("species_6".to_string());
    options.collapsed_nodes.push("species_15".to_string());
    options.collapsed_nodes.push("species_11".to_string());

    options.gene_internal = true;
    let config: Config = Config::new();
    read_phyloxml("xml_examples/FAM000297_species.xml".to_string(), &mut tree);
    phyloxml_processing(
        &mut tree,
        &options,
        &config,
        "read_phyloxml_collapsed.svg".to_string(),
    );
    println!("Please open output file 'read_phyloxml_collapsed.svg' with your browser");
}

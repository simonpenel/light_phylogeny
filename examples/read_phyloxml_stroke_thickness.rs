use light_phylogeny::{ArenaTree,Options,Config,read_phyloxml,phyloxml_processing};

fn main() {

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_stroke_thickness_1.svg".to_string());
    println!("Please open output file 'read_phyloxml_stroke_thickness_1.svg' with your browser");


    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    options.gthickness = 1;
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_stroke_thickness_2.svg".to_string());
    println!("Please open output file 'read_phyloxml_stroke_thickness_2.svg' with your browser");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    options.rotate = false;
    options.gthickness = 6;
    let config: Config = Config::new();
    read_phyloxml("examples/FAM036542_gene.xml".to_string(), &mut tree);
    phyloxml_processing(&mut tree, &options, &config,"read_phyloxml_stroke_thickness_3.svg".to_string());
    println!("Please open output file 'read_phyloxml_stroke_thickness_3.svg' with your browser");

}

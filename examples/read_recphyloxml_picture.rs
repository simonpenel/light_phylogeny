// Display a reconciled tree form recPhyloXML format
use std::collections::HashMap;

use light_phylogeny::{ArenaTree,Options,Config,read_recphyloxml_multi,recphyloxml_processing};
// use std::env;

fn main() {




    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    let mut pictures = HashMap::new();
    pictures.insert("Homo".to_string(),"Zinaida-Serebryakova-Self-Portrait.jpg:100:50:90".to_string());
    pictures.insert("outgroup".to_string(),"Cisco.png:130:0:-400".to_string());
    pictures.insert("Mus".to_string(),"Mus_musculus_George_Shuklin.png:130:60:-150".to_string());
    pictures.insert("Pan".to_string(),"extract_from_015_Chimpanzee_at_Kibale_forest_National_Park_Photo_by_Giles_Laurent.png:85:40:90".to_string());
    options.pictures =  pictures;

    //options.time_lines.push(time_line_1);
    //options.time_lines.push(time_line_2);
    //options.time_lines.push(time_line_3);
    options.gene_colors.push("blue".to_string());
    options.species_internal = true;
    //options.fill_species = true;
    options.rotate=true;    // options.time_lines.insert("PPENT".to_string(),"red".to_string());
    // options.time_lines.insert("PQUAD".to_string(),"green".to_string());
    // options.time_lines.insert("PTRED".to_string(),"green".to_string());
    read_recphyloxml_multi("examples/simple.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_pictures.svg".to_string());
    println!("Please open output file 'read_recphyloxml_pictures.svg' with your browser");


    // ============================================================================================
    // Version landscape
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    let mut pictures = HashMap::new();
    pictures.insert("Homo".to_string(),"Zinaida-Serebryakova-Self-Portrait.jpg:100:50:90".to_string());
    pictures.insert("outgroup".to_string(),"Cisco.png:130:0:-400".to_string());
    pictures.insert("Mus".to_string(),"Mus_musculus_George_Shuklin.png:130:60:-150".to_string());
    pictures.insert("Pan".to_string(),"extract_from_015_Chimpanzee_at_Kibale_forest_National_Park_Photo_by_Giles_Laurent.png:85:40:90".to_string());
    options.pictures =  pictures;

    //options.time_lines.push(time_line_1);
    //options.time_lines.push(time_line_2);
    //options.time_lines.push(time_line_3);
    options.gene_colors.push("blue".to_string());
    options.species_internal = true;
    //options.fill_species = true;
    options.rotate=false;    
    // options.time_lines.insert("PQUAD".to_string(),"green".to_string());
    // options.time_lines.insert("PTRED".to_string(),"green".to_string());
    read_recphyloxml_multi("examples/simple.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_pictures_2.svg".to_string());
    println!("Please open output file 'read_recphyloxml_pictures_2.svg' with your browser");

    // ============================================================================================
    // Version portrait
    let transfers = vec![];
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    let mut sp_tree: ArenaTree<String> = ArenaTree::default();
    let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
    let mut global_roots: std::vec::Vec<usize> = Vec::new();

    let mut pictures = HashMap::new();
    pictures.insert("Homo".to_string(),"Zinaida-Serebryakova-Self-Portrait.jpg:100:50:90".to_string());
    pictures.insert("outgroup".to_string(),"Cisco.png:130:0:-400".to_string());
    pictures.insert("Mus".to_string(),"Mus_musculus_George_Shuklin.png:130:60:-150".to_string());
    pictures.insert("Pan".to_string(),"extract_from_015_Chimpanzee_at_Kibale_forest_National_Park_Photo_by_Giles_Laurent.png:85:40:90".to_string());
    options.pictures =  pictures;

    //options.time_lines.push(time_line_1);
    //options.time_lines.push(time_line_2);
    //options.time_lines.push(time_line_3);
    options.gene_colors.push("blue".to_string());
    options.species_only_flag = true;
    //options.fill_species = true;
    options.rotate=true;    // options.time_lines.insert("PPENT".to_string(),"red".to_string());
    // options.time_lines.insert("PQUAD".to_string(),"green".to_string());
    // options.time_lines.insert("PTRED".to_string(),"green".to_string());
    read_recphyloxml_multi("examples/simple.recphyloxml".to_string(),
        &mut sp_tree, &mut gene_trees, &mut global_roots);
    recphyloxml_processing(&mut sp_tree, &mut gene_trees, &mut options, &config, true,
         &transfers, "read_recphyloxml_pictures_3.svg".to_string());
    println!("Please open output file 'read_recphyloxml_pictures_3.svg' with your browser");

}

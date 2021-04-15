use light_phylogeny::{ArenaTree,Options,Config,newick2tree,phyloxml_processing,lca};
use std::fs;
fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    let config: Config = Config::new();
    println!("Reading newick file examples/newick.A4.txt...");
    let contents = fs::read_to_string("examples/newick.A4.txt")
                .expect("Something went wrong reading the newick file");
                println!("Create a first node which will be the root...");
                let root = tree.new_node("Root".to_string());
                println!("Build the tree from the file contents...");
                newick2tree(contents, &mut tree, root, &mut 0);
                let j = tree.get_index("J".to_string()).expect("Error : unable to find J");
                println!("Index of leaf J is {}",j);
                let l = tree.get_index("L".to_string()).expect("Error : unable to find L");
                println!("Index of leaf L is {}",l);
                let n = tree.get_index("N".to_string()).expect("Error : unable to find N");
                println!("Index of leaf N is {}",n);
                let lca_jl = lca(&mut tree,j,l);
                println!("Index of lca betwen J and L is {}",lca_jl);
                tree.arena[lca_jl].name = "LCA of J and L".to_string();
                let lca_jn = lca(&mut tree,j,n);
                println!("Index of lca betwen J and N is {}",lca_jn);
                tree.arena[lca_jn].name = "LCA of J and N".to_string();
                // Display internal nodes
                options.gene_internal = true ;
                phyloxml_processing(&mut tree, &options, &config,"lca.svg".to_string());

                println!("Please open output file 'lca.svg' with your browser");
                println!("OK.");
}

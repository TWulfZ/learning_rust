enum Block {
    Stone,
    Dirt,
    Grass,
    Custom { name: String},
}

fn describe_block(block: Block) {
    match block {
        Block::Stone => println!("Este es un bloque de piedra"),
        Block::Dirt => println!("Este es un bloque de tierra"),
        Block::Grass => println!("Este es un bloque de hierba"),
        Block::Custom { name } => {
            println!("Este es un bloque personalizado: {}", name);
        }
    }
}

fn main() {
    let block1 = Block::Stone;
    let block2 = Block::Dirt;
    let block3 = Block::Grass;
    let block4 = Block::Custom { name: "Custom block".to_string() };
    
    describe_block(block1);
    describe_block(block2);
    describe_block(block3);
    describe_block(block4);
}

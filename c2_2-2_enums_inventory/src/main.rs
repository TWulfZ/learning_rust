enum Item {
    Sword { damage: u32 },
    Pickaxe { durabiliy : u32 },
    Food (String),
    Other (String),
}

fn use_item(item: Item) {
    match item {
        Item::Sword { damage } => println!("Sword does {} damage", damage),
        Item::Pickaxe { durabiliy} => println!("Pickaxe has {} durability", durabiliy),
        Item::Food(name) => println!("Eating {} food", name),
        Item::Other(name) => println!("Using {} item", name),
    }
}

fn main() {
    let sword = Item::Sword { damage: 10 };
    let pickaxe = Item::Pickaxe { durabiliy: 20 };
    let food = Item::Food("Golden apple".to_string());
    let other = Item::Other("Fishrod".to_string());
    use_item(sword);
    use_item(pickaxe);
    use_item(food);
    use_item(other);
    use_item(Item::Other("Regeneration Potion II".to_string()));

}

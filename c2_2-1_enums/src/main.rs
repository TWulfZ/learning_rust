// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#[derive(Debug)]

enum Block {
  Stone,
  Dirt,
  Grass,
  Custom { name: String},
}

enum Colors {
  Red,
  Green,
  Blue,
}



enum GameEvent {
    BlockBreak { block: Block, x:i32, y:i32, z:i32},
    BlockPlace { block: Block, x:i32, y:i32, z:i32},
    PlayerJoin(String),
    PlayerLeave(String),
}

fn handle_event(event: GameEvent) {
  match event {
      GameEvent::BlockBreak { block, x, y, z } => {
            println!(
                "Se rompió el bloque {:?} en la posición ({}, {}, {})",
                block, x, y, z
            );
        },
        GameEvent::BlockPlace { block, x, y, z } => {
          println!(
            "Se colocó el bloque {:?} en la posición ({}, {}, {})",
            block, x, y, z
          )
        },
        GameEvent::PlayerJoin(name) => { println!("El jugador {} se unió al servidor", name); },
        GameEvent::PlayerLeave(name) => { println!("El jugador {} se desconectó del servidor", name); },
  }
}

fn main() {
  let event = GameEvent::BlockBreak {
    block: Block::Stone, x: 0, y: 0, z: 0, };
  let event2 = GameEvent::PlayerJoin("TWulfZ".to_string());
  handle_event(event);
  handle_event(event2);
  println!("{:?}", Colors::Red);
}

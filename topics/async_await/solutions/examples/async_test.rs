#![allow(unused)]

struct Tomato;
struct Lettuce;
struct Cheese;
struct Patty;
struct Bun;

struct Hamburger {
    pub tomato: Tomato,
    pub lettuce: Lettuce,
    pub cheese: Cheese,
    pub patty: Patty,
    pub bun: Bun,
}

async fn toast_bun() -> Bun {
    Bun
}

async fn cook_patty() -> Patty {
    Patty
}

async fn get_veggies() -> (Tomato, Lettuce) {
    (Tomato, Lettuce)
}

async fn get_cheese() -> Cheese {
    Cheese
}

async fn make_hamburger_seq() -> Hamburger {
    let bun = toast_bun().await;
    let patty = cook_patty().await;
    let (tomato, lettuce) = get_veggies().await;
    let cheese = get_cheese().await;

    println!("🍔 is ready");

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun,
    }
}
async fn make_hamburger() -> Hamburger {

   // do everything concurrently - all at the same time
   let (bun, patty, (tomato, lettuce), cheese) = tokio::join!(
        toast_bun(),
        cook_patty(),
        get_veggies(),
        get_cheese()
    );

    println!("🍔 is ready");

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun,
    }
}

#[tokio::main]
async fn main() {
    // make_hamburger_seq().await;
    // make_hamburger().await;
    let fut = make_hamburger().await;

}

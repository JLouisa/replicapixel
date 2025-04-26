#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use replicapixel::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenvy::dotenv().ok();
    let _ctx = playground::<App>().await?;

    // let active_model: articles::ActiveModel = articles::ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    // src/main.rs
    use axum_extra::extract::cookie::Key;
    use rand::{rng, Rng};

    // Generate a cryptographically random key of 64 bytes
    let mut rng = rng();
    let mut random_key = [0u8; 64];
    rng.fill(&mut random_key);
    match Key::try_from(&random_key[..]) {
        Ok(key) => {
            println!("Random key: {:?}", key.master());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}

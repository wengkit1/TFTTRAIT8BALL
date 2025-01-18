use crate::models::champions::{ChampionPool, Trait};
use once_cell::sync::OnceCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct Context {
    pub champion_pool: Arc<ChampionPool>,
    pub traits: Arc<Vec<Trait>>,
}

static CONTEXT: OnceCell<Context> = OnceCell::new();

pub fn init(champion_pool: ChampionPool, traits: Vec<Trait>) {
    let context = Context {
        champion_pool: Arc::new(champion_pool),
        traits: Arc::new(traits),
    };

    CONTEXT.set(context).expect("Context already initialized");
}

pub fn get() -> &'static Context {
    CONTEXT.get().expect("Context not initialized")
}

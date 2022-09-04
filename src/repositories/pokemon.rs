use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> PokemonNumber
}

pub struct InMemoryRepository {
    pokemons: Vec<Pokemon>,
}

impl Repository for InMemoryRepository {
fn insert (&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> PokemonNumber {
    number
}

// impl Repository for InMemoryRepository {
impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons: Vec<Pokemon> = vec![];
        Self { pokemons }
    }
}

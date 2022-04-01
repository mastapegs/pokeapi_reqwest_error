#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use serde::{Deserialize, Serialize};
use std::fmt;

const POKEAPI_ROOT: &str = "https://pokeapi.co/api/v2";

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind<'a> {
    RequestError(&'a str),
}

impl fmt::Display for ErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::RequestError(message) => write!(f, "{message}"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind<'static>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(_error: reqwest::Error) -> Self {
        Self {
            kind: ErrorKind::RequestError("Reqwest Error"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonDetail {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonResponse {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<PokemonDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinglePokemonResponse {
    id: u16,
    name: String,
    base_experience: u16,
    height: u16,
    is_default: bool,
    order: u16,
    weight: u16,
}

/// # Errors
///
/// Will return `Err` if request fails
pub async fn get_all_pokemon() -> Result<PokemonResponse> {
    reqwest::get(format!("{POKEAPI_ROOT}/pokemon"))
        .await?
        .json::<PokemonResponse>()
        .await
        .map_err(Error::from)
}

/// # Errors
///
/// Will return `Err` if request fails
pub async fn get_single_pokemon(id: u16) -> Result<SinglePokemonResponse> {
    reqwest::get(&format!("{POKEAPI_ROOT}/pokemon/{id}"))
        .await?
        .json::<SinglePokemonResponse>()
        .await
        .map_err(Error::from)
}

#[cfg(test)]
mod tests {

    use super::{
        get_all_pokemon, get_single_pokemon, PokemonResponse, Result, SinglePokemonResponse,
    };
    #[tokio::test]
    async fn test_reqwest() -> Result<()> {
        let pokemon_response = reqwest::get("https://pokeapi.co/api/v2/pokemon")
            .await?
            .json::<PokemonResponse>()
            .await?;

        assert_eq!(pokemon_response.results[0].name, "bulbasaur");
        assert_eq!(pokemon_response.results[1].name, "ivysaur");
        assert_eq!(pokemon_response.results[2].name, "venusaur");

        assert_eq!(pokemon_response.results[3].name, "charmander");
        assert_eq!(pokemon_response.results[4].name, "charmeleon");
        assert_eq!(pokemon_response.results[5].name, "charizard");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_pokemon() -> Result<()> {
        let pokemon_response = get_all_pokemon().await?;

        assert_eq!(pokemon_response.results[0].name, "bulbasaur");

        Ok(())
    }

    #[tokio::test]
    async fn test_single_fetch_prototype() -> Result<()> {
        let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/1")
            .await?
            .json::<SinglePokemonResponse>()
            .await?;

        assert_eq!(response.name, "bulbasaur");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_single_pokemon() -> Result<()> {
        let pokemon_one = get_single_pokemon(1).await?;
        let pokemon_two = get_single_pokemon(4).await?;
        let pokemon_three = get_single_pokemon(7).await?;

        assert_eq!(pokemon_one.name, "bulbasaur");
        assert_eq!(pokemon_two.name, "charmander");
        assert_eq!(pokemon_three.name, "squirtle");

        Ok(())
    }
}

use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::Repository;

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

fn execute(rep: &mut dyn Repository, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let mut repo = InMemoryRepository::new();
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };
        let res = execute(&mut repo, req);
        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let mut repo = InMemoryRepository::new();
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };
        let res = execute(&mut repo, req);
        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }
    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        repo.instert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("Pikachu"),
            types: vec![String::from("Fire")],
        };
        let res = execute(&mut repo, req);
        match res {
            Response::Conflict => {}
            _ => unreachable!(,),
        }
    }
}

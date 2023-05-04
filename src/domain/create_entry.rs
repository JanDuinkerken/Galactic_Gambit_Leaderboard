use super::entities::{EntryId, EntryUsername, EntryLevel, EntrySeconds};
use crate::repositories::leaderboard::{Insert, Repository};
use std::convert::TryFrom;

struct Request {
    id: u32,
    user: String,
    level: u32,
    seconds: u64,
}

enum Response {
    Ok(u32),
    BadRequest,
    Conflict,
    Error
}

fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (
        EntryId::try_from(req.id),
        EntryUsername::try_from(req.user),
        EntryLevel::try_from(req.level),
        EntrySeconds::try_from(req.seconds)
    ) {
        (Ok(id), Ok(user), Ok(level), Ok(seconds)) => match repo.insert(id, user, level, seconds) {
            Insert::Ok(id) => Response::Ok(u32::from(id)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}
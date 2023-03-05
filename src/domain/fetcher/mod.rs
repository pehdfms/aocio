use reqwest::{blocking::Client, header::COOKIE};

use crate::common::{day::AocDay, session::Session, year::AocYear};

use self::cache::{Cache, MemoryCache, NoCache, WriteConflictStrategy};

pub mod cache;

pub struct InputFetcher<C: Cache> {
    session: Session,
    cache: C,
}

impl InputFetcher<NoCache> {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            cache: NoCache::new(),
        }
    }
}

impl InputFetcher<MemoryCache> {
    pub fn with_memory_cache(session: Session, handle_conflict: WriteConflictStrategy) -> Self {
        Self {
            session,
            cache: MemoryCache::new(handle_conflict),
        }
    }
}

impl<C: Cache> InputFetcher<C> {
    pub fn with_cache(session: Session, cache: C) -> Self {
        Self { session, cache }
    }

    pub fn get_input(&mut self, year: AocYear, day: AocDay) -> Result<String, reqwest::Error> {
        self.cache.read(year, day).map(Ok).unwrap_or_else(|| {
            let input = self.fetch(year, day)?;
            self.cache.write(year, day, &input).unwrap();

            Ok(input)
        })
    }

    fn fetch(&self, year: AocYear, day: AocDay) -> Result<String, reqwest::Error> {
        Client::builder()
            .cookie_store(true)
            .build()?
            .get(
                format!("https://adventofcode.com/{year}/day/{day}/input")
            )
            .header(COOKIE, format!("session={}", self.session))
            .send()?
            .text()
            .and_then(|text| {
                if text.contains("Puzzle inputs differ by user.") {
                    panic!("Tried to fetch input data from the advent of code website, but could not authenticate. Did you set up your session key correctly?")
                } else {
                    Ok(text)
                }
            })
    }
}

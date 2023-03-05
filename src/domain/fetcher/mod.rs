use reqwest::{blocking::Client, header::COOKIE};

use crate::common::{day::AocDay, session::Session, year::AocYear};

use self::cache::{Cache, MemoryCache, NoCache};

pub mod cache;

pub struct InputFetcher<C: Cache> {
    session: Session,
    cache: C,
}

impl InputFetcher<NoCache> {
    #[must_use]
    pub const fn new(session: Session) -> Self {
        Self {
            session,
            cache: NoCache::new(),
        }
    }
}

impl InputFetcher<MemoryCache> {
    #[must_use]
    pub fn with_memory_cache(session: Session) -> Self {
        Self {
            session,
            cache: MemoryCache::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum HandleCacheHitStrategy {
    ReturnCache,
    OverwriteCache,
    ErrorOnCacheHit,
}

impl<C: Cache> InputFetcher<C> {
    #[must_use]
    pub const fn with_cache(session: Session, cache: C) -> Self {
        Self { session, cache }
    }

    pub fn get_input_handle_cache(
        &mut self,
        year: AocYear,
        day: AocDay,
        handle_cache_hit: HandleCacheHitStrategy,
    ) -> Result<String, reqwest::Error> {
        if let Some(cache) = self.cache.read(year, day) {
            match handle_cache_hit {
                HandleCacheHitStrategy::ReturnCache => return Ok(cache),
                HandleCacheHitStrategy::OverwriteCache => (),
                HandleCacheHitStrategy::ErrorOnCacheHit => todo!(),
            }
        }

        let input = self.fetch(year, day)?;
        self.cache.write(year, day, &input).unwrap();

        Ok(input)
    }

    pub fn get_input(&mut self, year: AocYear, day: AocDay) -> Result<String, reqwest::Error> {
        self.get_input_handle_cache(year, day, HandleCacheHitStrategy::ReturnCache)
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

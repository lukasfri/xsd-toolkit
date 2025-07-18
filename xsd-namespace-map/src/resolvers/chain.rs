use derive_more::{Display, Error};
use url::Url;

use crate::resolvers::{AsyncXmlResolver, XmlResolver};

pub struct PossibleResolver<P, F, Filter: Fn(&Url) -> bool> {
    possible: P,
    fallback: F,
    filter: Filter,
}

impl<T, U, Filter: Fn(&Url) -> bool> PossibleResolver<T, U, Filter> {
    pub fn new(possible: T, fallback: U, filter: Filter) -> Self {
        PossibleResolver {
            possible,
            fallback,
            filter,
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum Error<P, F> {
    #[display("Error in possible resolver: {_0:?}")]
    Possible(P),
    #[display("Error in fallback resolver: {_0:?}")]
    Fallback(F),
}

pub trait PossibleResolverExt: Sized {
    fn try_possible<P, Filter>(
        self,
        this_2: P,
        filter: Filter,
    ) -> PossibleResolver<P, Self, Filter>
    where
        P: 'static,
        Filter: Fn(&Url) -> bool;
}

impl<F: 'static> PossibleResolverExt for F {
    fn try_possible<P, Filter>(self, possible: P, filter: Filter) -> PossibleResolver<P, F, Filter>
    where
        P: 'static,
        Filter: Fn(&Url) -> bool,
    {
        PossibleResolver::new(possible, self, filter)
    }
}

impl<T: xmlity::DeserializeOwned, P, F, Filter> XmlResolver<T> for PossibleResolver<P, F, Filter>
where
    P: XmlResolver<T> + 'static,
    <P as XmlResolver<T>>::Error: 'static,
    F: XmlResolver<T> + 'static,
    <F as XmlResolver<T>>::Error: 'static,
    Filter: Fn(&Url) -> bool,
{
    type Error = Error<P::Error, F::Error>;

    fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        if (self.filter)(location) {
            return self
                .possible
                .resolve_document(location)
                .map_err(Error::Possible);
        }
        self.fallback
            .resolve_document(location)
            .map_err(Error::Fallback)
    }
}

impl<T: xmlity::DeserializeOwned, P, F, Filter> AsyncXmlResolver<T>
    for PossibleResolver<P, F, Filter>
where
    P: AsyncXmlResolver<T> + 'static,
    <P as AsyncXmlResolver<T>>::Error: 'static,
    F: AsyncXmlResolver<T> + 'static,
    <F as AsyncXmlResolver<T>>::Error: 'static,
    Filter: Fn(&Url) -> bool,
{
    type Error = Error<P::Error, F::Error>;

    async fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        if (self.filter)(location) {
            return self
                .possible
                .resolve_document(location)
                .await
                .map_err(Error::Possible);
        }
        self.fallback
            .resolve_document(location)
            .await
            .map_err(Error::Fallback)
    }
}

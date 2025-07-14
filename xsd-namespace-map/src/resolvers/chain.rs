use derive_more::{Display, Error};
use url::Url;

use crate::resolvers::{AsyncXmlSchemaResolver, XmlSchemaResolver};

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

impl<P, F, Filter> XmlSchemaResolver for PossibleResolver<P, F, Filter>
where
    P: XmlSchemaResolver + 'static,
    F: XmlSchemaResolver + 'static,
    Filter: Fn(&Url) -> bool,
{
    type Error = Error<P::Error, F::Error>;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        if (self.filter)(location) {
            return self
                .possible
                .resolve_schema(location)
                .map_err(Error::Possible);
        }
        self.fallback
            .resolve_schema(location)
            .map_err(Error::Fallback)
    }
}

impl<P, F, Filter> AsyncXmlSchemaResolver for PossibleResolver<P, F, Filter>
where
    P: AsyncXmlSchemaResolver + 'static,
    F: AsyncXmlSchemaResolver + 'static,
    Filter: Fn(&Url) -> bool,
{
    type Error = Error<P::Error, F::Error>;

    async fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        if (self.filter)(location) {
            return self
                .possible
                .resolve_schema(location)
                .await
                .map_err(Error::Possible);
        }
        self.fallback
            .resolve_schema(location)
            .await
            .map_err(Error::Fallback)
    }
}

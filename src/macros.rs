/// helper macro to avoid the `TryInto` boilerplate for builder functions
macro_rules! url_builder_fn {
    {
        $(#[$meta:meta])+
        $name:ident, $builder:ident
    } => {
        $(#[$meta])+
        pub fn $name<U: TryInto<::reqwest::Url, Err = Error>>(self, $name: U) -> $builder {
            match self.inner {
                Ok(mut inner) => {
                    match $name.try_into() {
                        Ok(url) => {
                            inner.$name = Some(url);
                            $builder { inner: Ok(inner) }
                        }
                        Err(e) => $builder { inner: Err(e) },
                    }
                }
                _ => self,
            }
        }
    }
}

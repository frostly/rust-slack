/// helper macro to avoid the `TryInto` boilerplate for builder functions
macro_rules! url_builder_fn {
    {
        $(#[$meta:meta])+
        $name:ident, $builder:ident
    } => {
        $(#[$meta])+
        pub fn $name(self, $name: &str) -> $builder {
            match self.inner {
                Ok(mut inner) => {
                    match Url::parse($name) {
                        Ok(url) => {
                            inner.$name = Some(url);
                            $builder { inner: Ok(inner) }
                        }
                        Err(e) => $builder { inner: Err(e.into()) },
                    }
                }
                _ => self,
            }
        }
    }
}

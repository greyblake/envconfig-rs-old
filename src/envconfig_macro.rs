#[macro_export]
macro_rules! envconfig {
    ($type:ident {$($field_name:ident : $field_type:ty = $var_name:expr), *}) => {
        #[derive(Debug)]
        struct $type {
            $(
                $field_name: $field_type,
            )*
        }

        impl $type {
            fn init() -> Result<Self, ::envconfig::Error> {
                $(
                    let $field_name: $field_type = ::envconfig::load_var($var_name)?;
                )*

                let config = Self {
                    $(
                        $field_name,
                    )*
                };

                Ok(config)
            }

            #[allow(dead_code)]
            fn init_or_die() -> Self {
                match Self::init() {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!("{}", err);
                        ::std::process::exit(1);
                    }
                }
            }
        }
    };
}

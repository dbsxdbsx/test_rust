#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ShareMode {
    Unique, // for (the types only for) regular/no-shared block
    Shared, // for (the types only for) shared block
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opts {
    name: String,
    shared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllConfigs {
    configs: Vec<Opts>,
    is_multi_blocks: bool,
}

impl AllConfigs {
    /// Creates a new `AllConfigs` instance from a slice of `Config` instances.
    ///
    /// # Arguments
    ///
    /// * `configs`: A slice of `Config` instances to be included in the `AllConfigs` instance.
    ///
    /// # Panics
    ///
    /// Panics if there are more than one `Config` instances with `ShareMode::Shared`.
    ///
    /// # Returns
    ///
    /// A new `AllConfigs` instance with the `Config` instances sorted so that the one with `ShareMode::Shared`
    /// comes first, followed by the ones with `ShareMode::Unique`. The `share_mode` field of the instance is
    /// set to `ShareMode::Unique` if there is no `Config` instance with `ShareMode::Shared`, or to `ShareMode::Shared`
    /// otherwise.
    pub fn new(configs: &[Opts]) -> Self {
        // pre-check
        let shared_count = configs.iter().filter(|c| c.shared).count();
        assert!(shared_count <= 1, "Only one shared config is allowed");
        // put config with shared_mode == true at very first, and leave others no changed
        let mut configs = configs.to_owned();
        configs.sort_by_key(|c| !c.shared);
        let mut configs = AllConfigs {
            configs: configs.into(),
            is_multi_blocks: false,
        };
        // check validation and if it is multi-blocks case
        configs.is_multi_blocks = if configs.get_shared_config().is_some() {
            assert!(configs.get_regular_configs().len() > 1);
            false
        } else {
            assert!(configs.get_regular_configs().len() == 1);
            true
        };
        // return
        configs
    }

    pub fn iter(&self) -> impl Iterator<Item = &Opts> {
        self.configs.iter()
    }

    pub fn get_regular_configs(&self) -> &[Opts] {
        let shared_config_index = self.configs.iter().position(|c| c.shared);
        match shared_config_index {
            Some(index) => &self.configs[index + 1..self.configs.len()],
            None => &self.configs[..],
        }
    }

    pub fn get_shared_config(&self) -> Option<&Opts> {
        self.configs.iter().find(|c| c.shared)
    }

    pub fn is_multi_blocks(&self) -> bool {
        self.is_multi_blocks
    }
}

impl<'a> IntoIterator for &'a AllConfigs {
    type Item = &'a Opts;
    type IntoIter = std::slice::Iter<'a, Opts>;

    fn into_iter(self) -> Self::IntoIter {
        self.configs.iter()
    }
}

impl IntoIterator for AllConfigs {
    type Item = Opts;
    type IntoIter = std::vec::IntoIter<Opts>;

    fn into_iter(self) -> Self::IntoIter {
        self.configs.into_iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_configs() {
        let configs = &[
            Opts {
                name: "Config 2".to_string(),
                shared: false,
            },
            Opts {
                name: "Config 1".to_string(),
                shared: false,
            },
            Opts {
                name: "Shared Config".to_string(),
                shared: true,
            },
        ];

        let all_configs = AllConfigs::new(configs);

        assert_eq!(
            all_configs.get_shared_config().unwrap().name,
            "Shared Config"
        );
        assert_eq!(all_configs.get_regular_configs().len(), 2);
        assert_eq!(all_configs.iter().count(), 3);

        let mut iter = all_configs.iter();
        assert_eq!(iter.next().unwrap().name, "Shared Config");
        assert_eq!(iter.next().unwrap().name, "Config 2");
        assert_eq!(iter.next().unwrap().name, "Config 1");
        assert_eq!(iter.next(), None);

        let mut iter = all_configs.clone().into_iter();
        assert_eq!(iter.next().unwrap().name, "Shared Config");
        assert_eq!(iter.next().unwrap().name, "Config 2");
        assert_eq!(iter.next().unwrap().name, "Config 1");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_all_configs_enumerate() {
        let configs = &[
            Opts {
                name: "Config 2".to_string(),
                shared: false,
            },
            Opts {
                name: "Config 1".to_string(),
                shared: false,
            },
            Opts {
                name: "Shared Config".to_string(),
                shared: true,
            },
        ];

        let all_configs = AllConfigs::new(configs);

        let mut iter = all_configs.iter().enumerate();
        assert_eq!(iter.next().unwrap(), (0, &configs[2]));
        assert_eq!(iter.next().unwrap(), (1, &configs[0]));
        assert_eq!(iter.next().unwrap(), (2, &configs[1]));
        assert_eq!(iter.next(), None);

        let mut iter = all_configs.clone().into_iter().enumerate();
        assert_eq!(iter.next().unwrap(), (0, configs[2].clone()));
        assert_eq!(iter.next().unwrap(), (1, configs[0].clone()));
        assert_eq!(iter.next().unwrap(), (2, configs[1].clone()));
        assert_eq!(iter.next(), None);
    }
}

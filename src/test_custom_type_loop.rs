#[derive(Debug, Clone)]
pub struct Config {
    name: String,
    share_mode: bool,
}

pub struct AllConfigs {
    configs: Vec<Config>,
}

impl AllConfigs {
    /// Creates a new `AllConfigs` instance from a slice of `Config` objects.
    ///
    /// The method checks that at most one `Config` object in the slice has its `share_mode` field set to `true`.
    /// If there is more than one such object, the method panics with an error message.
    ///
    /// Note: The method sorts the `Config` objects in the slice so that the one with `share_mode` set to `true` comes first,
    /// and the others follow in their original order.
    ///
    /// # Arguments
    ///
    /// * `configs` - A slice of `Config` objects to include in the `AllConfigs` instance.
    ///
    /// # Returns
    ///
    /// A new `AllConfigs` instance containing the `Config` objects in the slice.
    ///
    /// # Panics
    ///
    /// The method panics if there is more than one `Config` object in the slice with `share_mode` set to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::{AllConfigs, Config};
    ///
    /// let configs = vec![
    ///     Config { share_mode: true, .. },
    ///     Config { share_mode: false, .. },
    ///     Config { share_mode: false, .. },
    /// ];
    ///
    /// let all_configs = AllConfigs::new(&configs);
    /// ```
    pub fn new(configs: &[Config]) -> Self {
        let shared_count = configs.iter().filter(|c| c.share_mode).count();
        assert!(shared_count <= 1, "Only one shared config is allowed");

        // put config with shared_mode == true at very first, and leave others no changed
        let mut configs = configs.to_owned();
        configs.sort_by_key(|c| !c.share_mode);
        AllConfigs {
            configs: configs.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Config> {
        self.configs.iter()
    }
}

impl<'a> IntoIterator for &'a AllConfigs {
    type Item = &'a Config;
    type IntoIter = std::slice::Iter<'a, Config>;

    fn into_iter(self) -> Self::IntoIter {
        self.configs.iter()
    }
}

impl IntoIterator for AllConfigs {
    type Item = Config;
    type IntoIter = std::vec::IntoIter<Config>;

    fn into_iter(self) -> Self::IntoIter {
        self.configs.into_iter()
    }
}

pub fn test_loop_for_custom_struct() {
    let configs = &[
        Config {
            name: "Config 1".to_string(),
            share_mode: false,
        },
        Config {
            name: "Config 2".to_string(),
            share_mode: false,
        },
        Config {
            name: "Shared Config".to_string(),
            share_mode: true,
        },
    ];

    let struct_for_loop = AllConfigs::new(configs);

    for config in &struct_for_loop {
        println!("{}", config.name);
    }
    println!("======================");
    for config in struct_for_loop.iter() {
        println!("{}", config.name);
    }
    println!("======================");
    for (index, config) in struct_for_loop.iter().enumerate() {
        println!("Index {}: {}", index, config.name);
    }
    // println!("======================");
    // for config in struct_for_loop.into_iter() {
    //     println!("into: {}", config.name);
    // }
    println!("======================");
    for (index, config) in struct_for_loop.into_iter().enumerate() {
        println!("into: Index {}: {}", index, config.name);
    }
}

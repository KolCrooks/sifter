const ONE_GB: usize = 1_000_000_000;


pub struct Config {
    pub page_cache_size: usize,
    pub data_directory: String
}

pub async fn get_config() -> Config {
    Config { page_cache_size: ONE_GB, data_directory: "/tmp".into() }
}
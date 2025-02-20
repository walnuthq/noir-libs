pub const MANIFEST_FILE_NAME: &str = "Nargo.toml";
pub const COMPANY_NAME: &str = "walnut";
pub const COMPANY_TLD: &str = "dev";
// pub const REGISTRY_URL: &str = "https://api.noir-libs.org/api/v1";
pub const REGISTRY_URL: &str = "http://localhost:3001/api/v1";
// pub const REGISTRY_HOME_URL: &str = "https://noir-libs.org";
pub const REGISTRY_HOME_URL: &str = "http://localhost:3000";
// Packaged projects (packages) are generated in this folder e.g. target/package/my_cool_package_0.1.1
pub const PACKAGING_OUTPUT_FOLDER_PATH: &str = "target/package";
// The name of ENV variable that contains the API key - required for package publishing
pub const API_KEY_ENV_VAR_NAME: &str = "API_KEY";
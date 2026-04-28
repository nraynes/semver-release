use r_log::Logger;

use crate::{Config, utils::run_command};
use semver_common::{Alert, Version};

pub fn run(conf: &Config, logger: &Logger, version: &Version) -> Result<(), Alert> {
    for (plugin_name, short_config) in conf.plugins().iter() {
        logger.info(&format!("Running plugin {}", plugin_name));
        run_command(
            plugin_name,
            [
                &serde_json::to_string(short_config)?,
                &serde_json::to_string(version)?,
            ],
        )?;
    }
    Ok(())
}

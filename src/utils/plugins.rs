use r_log::Logger;

use crate::Config;
use semver_common::{Alert, Version, run_command};

pub fn run(conf: &Config, logger: &Logger, version: &Version) -> Result<(), Alert> {
    for (plugin_name, short_config) in conf.plugins().iter() {
        logger.info(&format!("Running plugin {}", plugin_name));
        run_command(
            &format!("{}/{}", conf.plugin_location(), plugin_name),
            [
                &format!("'{}'", &serde_json::to_string(short_config)?),
                &format!("'{}'", &serde_json::to_string(version)?),
                &format!("'{}'", &conf.log_level().to_string()),
            ],
            Some(logger),
        )?;
    }
    Ok(())
}

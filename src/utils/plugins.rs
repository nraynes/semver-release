use r_log::Logger;

use crate::{Alert, Config, utils::run_command};

pub fn run(conf: &Config, logger: &Logger) -> Result<(), Alert> {
    for (plugin_name, short_config) in conf.plugins().iter() {
        logger.info(&format!("Running plugin {}", plugin_name));
        run_command(plugin_name, serde_json::to_string(short_config))?;
    }
    Ok(())
}

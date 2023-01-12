use crate::Config;
use crate::plugins::image_store_plugin::ImageStorePlugin;
use crate::{config::errors::Errors};
use anyhow::{Result, anyhow};

use log::{info, error};

// The search string prefix for this plugin.
const PREFIX: &str  = "image_store_";

/** Called one time by each internal plugin to select their single action function. 
 * The internal_actions array component of the plugins configuration object lists
 * zero or more function names.  Each function name is associated with one of the 
 * defined plugins using a convention.  The convention is that each function begins
 * with a prefix derived from the plugin name by dropping the "plugin.rs" portion
 * of the plugin file's name.
 * 
 * This function returns the first function named in internal_actions that matches
 * the plugin's prefix string.  If no such entry is found, the no-op action is 
 * returned.  If a matching entry does not correspond to an action function defined
 * in this file, then an error is returned.
 * 
 * Each action function associated with this plugin requires an arm in the match 
 * statement, which requires maintenance when new action functions are developed. 
 */
pub fn select_action(config: &'static Config) -> Result<fn(&ImageStorePlugin)> {
    
    // Iterate through all configured actions looking for the one
    // that targets this plugin.  The convention is that a plugin's
    // actions start with a prefix of plugin's file name (i.e., the
    // text preceeding 'plugin.rs').
    for action in &config.plugins.internal_actions {
        // See if the current action is for this plugin.
        if !(*action).starts_with(PREFIX) {continue;}

        // We expect an implemented function to match the name.
        match action.as_str() {
            "image_store_noop_action" => {
                info!("{}", Errors::ActionConfigured("ImageStorePlugin".to_string(), action.to_string()));
                return Result::Ok(image_store_noop_action);
            },
            unknown => {
                let msg = Errors::ActionNotFound("ImageStorePlugin".to_string(), unknown.to_string());
                error!("{}", msg);
                return Result::Err(anyhow!(msg));
            }
        };
    }
    
    // Default is to take no action is specified for this plugin.
    Result::Ok(image_store_noop_action)
}

/** No-op action. */
#[allow(unused)]
pub fn image_store_noop_action(plugin: &ImageStorePlugin) {}

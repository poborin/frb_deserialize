use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainingPlan {
    pub(crate) weeks: u8,
}

impl TrainingPlan {
    #[frb(sync)]
    pub fn test_deserialize(content: String) -> Result<Self, String> {
        serde_json::from_str(&content).map_err(|e| e.to_string())
        
    }
}


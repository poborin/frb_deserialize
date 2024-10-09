use crate::BACKTRACE;
use flutter_rust_bridge::frb;
use log::info;
use serde::{Deserialize, Serialize};
use std::panic;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainingPlan {
    pub(crate) weeks: u8,
}

impl TrainingPlan {
    #[frb(sync)]
    pub fn test_deserialize(content: String) -> Result<Self, String> {
        info!("<!> deserializing training plan");

        let result = panic::catch_unwind(|| {
            serde_json::from_str(&content)
        });

        match result {
            Ok(Ok(plan)) => Ok(plan),
            Ok(Err(e)) => {
                info!("Deserialization error: {}", e.to_string());
                Err(e.to_string())
            }
            Err(e) => {
                let b = BACKTRACE.take().unwrap();
                info!("at panic:\n{}", b);
                let err = format!("A panic occurred during deserialization: {e:?}");
                info!("{}", err);
                Err(err)
            }
        }
    }
}


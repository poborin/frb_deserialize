use crate::BACKTRACE;
use flutter_rust_bridge::frb;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::panic;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainingPlan {
    pub(crate) cycles: Vec<CycleElement>,
    /// training plan title
    pub(crate) title: String,
}

// impl super::common_traits::Deserialize for TrainingPlan {
//     #[frb(sync)]
//     fn deserialize(content: String) -> Result<Self, String> {
//         info!("<!> deserializing training plan: {}", content);
//
//         let result = panic::catch_unwind(|| {
//             serde_json::from_str(&content)
//         });
//
//         match result {
//             Ok(Ok(plan)) => Ok(plan),
//             Ok(Err(e)) => {
//                 error!("Deserialization error: {}", e.to_string());
//                 Err(e.to_string())
//             }
//             Err(_) => {
//                 error!("A panic occurred during deserialization");
//                 Err("A panic occurred during deserialization".to_string())
//             }
//         }
//     }
// }

impl TrainingPlan {
    #[frb(sync)]
    pub fn test_deserialize(content: String) -> Result<Self, String> {
        info!("<!> deserializing training plan: {}", content);

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

    #[frb(sync)]
    pub fn test_panic() {
        info!("test panic:");
        let result = panic::catch_unwind(|| {
            panic!("test panic");
        });

        match result {
            Err(e) => {
                let b = BACKTRACE.take().unwrap();
                info!("at panic:\n{}", b);
            }
            _ => info!("Test not panic"),
        }
    }

    pub fn test_deserialize_async(content: String) -> Result<Self, String> {
        info!("<!> deserializing training plan: {}", content);

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CycleElement {
    /// Type of cycle
    pub(crate) cycle_type: CycleType,
    /// End date of the cycle or phase
    pub(crate) end_date: String,
    /// Name or description of the cycle or phase
    pub(crate) name: String,
    pub(crate) phases: Vec<PhaseElement>,
    pub(crate) sessions: Vec<SessionElement>,
    /// Start date of the cycle or phase
    pub(crate) start_date: String,
    /// Nested cycles for finer planning
    pub(crate) sub_cycles: Vec<CycleElement>,
    pub(crate) target: Target,
}

/// Type of cycle
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum CycleType {
    #[serde(rename = "in-season")]
    InSeason,
    Macrocycle,
    Mesocycle,
    Microcycle,
    #[serde(rename = "off-season")]
    OffSeason,
    #[serde(rename = "post-season")]
    PostSeason,
    #[serde(rename = "pre-season")]
    PreSeason,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhaseElement {
    /// Training phase
    pub(crate) phase: Phase,
    /// Number of weeks in the phase
    pub(crate) weeks: u8,
}

/// Training phase
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Phase {
    #[serde(rename = "in-season")]
    InSeason,
    #[serde(rename = "off-season")]
    OffSeason,
    #[serde(rename = "post-season")]
    PostSeason,
    #[serde(rename = "pre-season")]
    PreSeason,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionElement {
    /// Day of the week for the training session
    pub(crate) day: Day,
    /// Duration of the training session
    pub(crate) duration: String,
    /// Focus areas of the training session
    pub(crate) focus_areas: String,
    /// Intensity level of the training session
    pub(crate) intensity: String,
    /// Additional notes for the training session
    pub(crate) notes: String,
    /// Focus areas for recovery sessions (e.g., flexibility, relaxation)
    pub(crate) recovery_focus: String,
    /// Specific details or exercises for rehabilitation
    pub(crate) rehab_details: String,
    /// Start time of the training session
    pub(crate) start_time: String,
    /// Type of training session
    #[serde(rename = "type")]
    pub(crate) coordinat_type: String,
}

/// Day of the week for the training session
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Day {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    /// Name of individual athlete or team
    pub(crate) name: String,
    /// Target audience type
    #[serde(rename = "type")]
    pub(crate) target_type: Type,
}

/// Target audience type
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Individual,
    Team,
}

#[cfg(test)]
mod tests {

    use cargo_metadata;
    use cargo_metadata::MetadataCommand;
    use std::fs;

    use crate::api::training_plan::TrainingPlan;

    #[test]
    fn test_json_deserialization() {
        let metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .unwrap();

        // Construct the full path to the file
        let file_path = metadata.workspace_root
            .join("../assets/test_data/training_plan.json");

        println!("{file_path:?}");

        // Read the contents of the file
        let training_plan = fs::read_to_string(file_path).unwrap();

        // Attempt to deserialize JSON into TrainingPlan struct
        let result = TrainingPlan::test_deserialize(training_plan);

        // Assert that deserialization was successful
        assert!(result.is_ok(), "JSON deserialization failed: {result:?}");

        // Further assertions can be added here to validate the deserialized data
        let training_plan = result.unwrap();
        println!("{training_plan:?}");
        // For example:
        // assert_eq!(training_plan.title, PlanTitle("Example Plan".to_string()));
    }
}

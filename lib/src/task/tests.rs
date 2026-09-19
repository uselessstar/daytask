use super::*;

#[test]
fn test_task_creation() {
    let task = Task::new("Test Task").expect("valid task name");

    assert_eq!(task.name(), "Test Task");
    assert_ne!(task.id(), Uuid::nil());
    assert_eq!(task.status(), Status::Pending);
    assert!(task.is_pending());
    assert!(!task.is_in_progress());
    assert!(!task.is_completed());
}

#[test]
fn test_status_can_be_changed() {
    let mut task = Task::new("Status Test").expect("valid task name");

    task.set_status(Status::InProgress);
    assert_eq!(task.status(), Status::InProgress);
    assert!(task.is_in_progress());

    task.set_status(Status::Completed);
    assert_eq!(task.status(), Status::Completed);
    assert!(task.is_completed());
}

#[test]
fn test_setting_same_status_is_a_no_op() {
    let mut task = Task::new("Same Status Test").expect("valid task name");
    let id = task.id();

    task.set_status(Status::Pending);

    assert_eq!(task.status(), Status::Pending);
    assert_eq!(task.id(), id);
}

#[test]
fn test_task_display_without_description() {
    let task = Task::new("Display Test").expect("valid task name");

    assert_eq!(task.to_string(), "Display Test [Pending]");
}

#[test]
fn test_task_display_with_description() {
    let mut task = Task::new("Display Test").expect("valid task name");
    task.set_status(Status::Completed);
    task.set_description("Task details");

    assert_eq!(task.to_string(), "Display Test [Completed]: Task details");
}

#[test]
fn test_id_version_matches_feature() {
    let task = Task::new("UUID Version Test").expect("valid task name");

    #[cfg(feature = "v4")]
    assert_eq!(task.id().get_version(), Some(uuid::Version::Random));

    #[cfg(feature = "v7")]
    assert_eq!(task.id().get_version(), Some(uuid::Version::SortRand));
}

#[test]
fn test_unique_ids() {
    let task1 = Task::new("Task #1").expect("valid task name");
    let task2 = Task::new("Task #2").expect("valid task name");

    assert_ne!(task1.id(), task2.id());
}

#[test]
fn test_set_name() {
    let mut task = Task::new("Initial Name").expect("valid task name");
    task.set_name("Updated Name").expect("valid task name");

    assert_eq!(task.name(), "Updated Name");
}

#[test]
fn test_setting_same_name_is_a_no_op() {
    let mut task = Task::new("Same Name").expect("valid task name");
    let id = task.id();

    task.set_name("Same Name").expect("valid task name");

    assert_eq!(task.name(), "Same Name");
    assert_eq!(task.id(), id);
}

#[test]
fn test_empty_name_is_rejected() {
    assert_eq!(Task::new("   "), Err(TaskError::EmptyName));

    let mut task = Task::new("Valid Name").expect("valid task name");
    assert_eq!(task.set_name("\t"), Err(TaskError::EmptyName));
    assert_eq!(task.name(), "Valid Name");
}

#[test]
fn test_description_starts_empty_and_can_be_set() {
    let mut task = Task::new("Description Test").expect("valid task name");

    assert_eq!(task.description(), None);

    task.set_description("Task details");

    assert_eq!(task.description(), Some("Task details"));
}

#[test]
fn test_setting_same_description_is_a_no_op() {
    let mut task = Task::new("Description Noop Test").expect("valid task name");

    task.set_description("Same description");
    let id = task.id();

    task.set_description("Same description");

    assert_eq!(task.id(), id);
    assert_eq!(task.description(), Some("Same description"));
}

#[test]
fn test_description_can_be_replaced_and_removed() {
    let mut task = Task::new("Description Update Test").expect("valid task name");

    task.set_description("Initial details");
    task.set_description(String::from("Updated details"));

    assert_eq!(task.description(), Some("Updated details"));

    task.clear_description();
    assert_eq!(task.description(), None);

    task.clear_description();
    assert_eq!(task.description(), None);
}

#[test]
fn test_mutations_preserve_task_identity() {
    let mut task = Task::new("Identity Test").expect("valid task name");
    let id = task.id();

    task.set_name("Renamed").expect("valid task name");
    task.set_description("Details");
    task.clear_description();

    assert_eq!(task.id(), id);
}

#[test]
fn test_equality_and_cloning() {
    let task = Task::new("Clone Test").expect("valid task name");
    let cloned_task = task.clone();

    assert_eq!(task, cloned_task);
    assert_eq!(task.id(), cloned_task.id());
    assert_eq!(task.name(), cloned_task.name());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let mut task = Task::new("Serde Test").expect("valid task name");
    task.set_description("Serialized details");
    let serialized = serde_json::to_string(&task).expect("serialization failed");
    let deserialized: Task = serde_json::from_str(&serialized).expect("deserialization failed");

    assert_eq!(task, deserialized);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization_without_description() {
    let task = Task::new("Serde Test").expect("valid task name");
    let serialized = serde_json::to_value(&task).expect("serialization failed");

    assert_eq!(serialized["id"], task.id().to_string());
    assert_eq!(serialized["name"], "Serde Test");
    assert_eq!(serialized["description"], serde_json::Value::Null);
    assert_eq!(serialized["status"], "Pending");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_rejects_empty_names() {
    for name in ["", "   "] {
        let serialized = serde_json::json!({
            "id": "00000000-0000-0000-0000-000000000001",
            "name": name,
            "description": null,
            "status": "Pending"
        });

        let result = serde_json::from_value::<Task>(serialized);

        assert!(result.is_err(), "expected invalid name to be rejected: {name:?}");
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_rejects_nil_ids() {
    let serialized = serde_json::json!({
        "id": "00000000-0000-0000-0000-000000000000",
        "name": "Valid name",
        "description": null,
        "status": "Pending"
    });

    let result = serde_json::from_value::<Task>(serialized);

    assert!(result.is_err(), "expected nil task id to be rejected");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_rejects_invalid_statuses_and_fields() {
    let invalid_values = [
        serde_json::json!({
            "id": "00000000-0000-0000-0000-000000000001",
            "name": "Valid name",
            "description": null,
            "status": "Unknown"
        }),
        serde_json::json!({
            "id": "not-a-uuid",
            "name": "Valid name",
            "description": null,
            "status": "Pending"
        }),
        serde_json::json!({
            "id": "00000000-0000-0000-0000-000000000001",
            "description": null,
            "status": "Pending"
        }),
    ];

    for value in invalid_values {
        assert!(serde_json::from_value::<Task>(value).is_err());
    }
}

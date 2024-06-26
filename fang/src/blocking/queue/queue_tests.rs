use crate::blocking::queue::Queueable;
use crate::runnable::Runnable;
use crate::typetag;
use crate::FangError;
use crate::Scheduled;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct PepeTask {
    pub number: u16,
}

#[typetag::serde]
impl Runnable for PepeTask {
    fn run(&self, _queue: &dyn Queueable) -> Result<(), FangError> {
        println!("the number is {}", self.number);

        Ok(())
    }
    fn uniq(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AyratTask {
    pub number: u16,
}

#[typetag::serde]
impl Runnable for AyratTask {
    fn run(&self, _queue: &dyn Queueable) -> Result<(), FangError> {
        println!("the number is {}", self.number);

        Ok(())
    }
    fn uniq(&self) -> bool {
        true
    }

    fn task_type(&self) -> String {
        "weirdo".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ScheduledPepeTask {
    pub number: u16,
    pub datetime: String,
}

#[typetag::serde]
impl Runnable for ScheduledPepeTask {
    fn run(&self, _queue: &dyn Queueable) -> Result<(), FangError> {
        println!("the number is {}", self.number);

        Ok(())
    }
    fn uniq(&self) -> bool {
        true
    }

    fn task_type(&self) -> String {
        "scheduled".to_string()
    }

    fn cron(&self) -> Option<Scheduled> {
        let datetime = self.datetime.parse::<DateTime<Utc>>().ok()?;
        Some(Scheduled::ScheduleOnce(datetime))
    }
}

/// This macro creates a module with tests for a `Queueable` type.
///
/// Arguments:
/// + `$mod`: Name for the module
/// + `$q`: Full path to type that implements `Queueable`
/// + `$e`: An expression that returns a value of `$q` suitable for testing.
///   + Multiple values returned by `$e` must be able to be interacted with concurrently without interfering with each other.
macro_rules! test_queue {
    ($mod:ident, $q:ty, $e:expr) => {
        mod $mod {
            use chrono::Duration;
            use chrono::SubsecRound;
            use chrono::Utc;
            use $crate::blocking::queue::queue_tests::{AyratTask, PepeTask, ScheduledPepeTask};
            use $crate::queue::Queueable;
            use $crate::runnable::COMMON_TYPE;
            use $crate::FangTaskState;

            #[test]
            fn insert_task_test() {
                let task = PepeTask { number: 10 };

                let queue: $q = $e;

                let task = queue.insert_task(&task).unwrap();

                let metadata = task.metadata.as_object().unwrap();
                let number = metadata["number"].as_u64();
                let type_task = metadata["type"].as_str();

                assert_eq!(task.error_message, None);
                assert_eq!(FangTaskState::New, task.state);
                assert_eq!(Some(10), number);
                assert_eq!(Some("PepeTask"), type_task);
            }

            #[test]
            fn fetch_task_fetches_the_oldest_task() {
                let task1 = PepeTask { number: 10 };
                let task2 = PepeTask { number: 11 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&task1).unwrap();
                let _task2 = queue.insert_task(&task2).unwrap();

                let found_task = queue
                    .fetch_and_touch_task(COMMON_TYPE.to_string())
                    .unwrap()
                    .unwrap();

                assert_eq!(found_task.id, task1.id);
            }

            #[test]
            fn update_task_state_test() {
                let task = PepeTask { number: 10 };

                let queue: $q = $e;

                let task = queue.insert_task(&task).unwrap();

                let found_task = queue
                    .update_task_state(&task, FangTaskState::Finished)
                    .unwrap();

                let metadata = found_task.metadata.as_object().unwrap();
                let number = metadata["number"].as_u64();
                let type_task = metadata["type"].as_str();

                assert_eq!(found_task.id, task.id);
                assert_eq!(found_task.state, FangTaskState::Finished);
                assert_eq!(Some(10), number);
                assert_eq!(Some("PepeTask"), type_task);
            }

            #[test]
            fn fail_task_updates_state_field_and_sets_error_message() {
                let task = PepeTask { number: 10 };

                let queue: $q = $e;

                let task = queue.insert_task(&task).unwrap();

                let error = "Failed";

                let found_task = queue.fail_task(&task, error).unwrap();

                let metadata = found_task.metadata.as_object().unwrap();
                let number = metadata["number"].as_u64();
                let type_task = metadata["type"].as_str();

                assert_eq!(found_task.id, task.id);
                assert_eq!(found_task.state, FangTaskState::Failed);
                assert_eq!(Some(10), number);
                assert_eq!(Some("PepeTask"), type_task);
                assert_eq!(found_task.error_message.unwrap(), error);
            }

            #[test]
            fn fetch_and_touch_updates_state() {
                let task = PepeTask { number: 10 };

                let queue: $q = $e;

                let task = queue.insert_task(&task).unwrap();

                let found_task = queue
                    .fetch_and_touch_task(COMMON_TYPE.to_string())
                    .unwrap()
                    .unwrap();

                let metadata = found_task.metadata.as_object().unwrap();
                let number = metadata["number"].as_u64();
                let type_task = metadata["type"].as_str();

                assert_eq!(found_task.id, task.id);
                assert_eq!(found_task.state, FangTaskState::InProgress);
                assert_eq!(Some(10), number);
                assert_eq!(Some("PepeTask"), type_task);
            }

            #[test]
            fn fetch_and_touch_returns_none() {
                let queue: $q = $e;

                let found_task = queue.fetch_and_touch_task(COMMON_TYPE.to_string()).unwrap();

                assert_eq!(None, found_task);
            }

            #[test]
            fn insert_task_uniq_test() {
                let task = PepeTask { number: 10 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&task).unwrap();
                let task2 = queue.insert_task(&task).unwrap();
                assert_eq!(task2.id, task1.id);
            }

            #[test]
            fn schedule_task_test() {
                let queue: $q = $e;
                let datetime = (Utc::now() + Duration::seconds(7)).round_subsecs(0);

                let task = &ScheduledPepeTask {
                    number: 10,
                    datetime: datetime.to_string(),
                };
                let task = queue.schedule_task(task).unwrap();

                let metadata = task.metadata.as_object().unwrap();
                let number = metadata["number"].as_u64();
                let type_task = metadata["type"].as_str();

                assert_eq!(Some(10), number);
                assert_eq!(Some("ScheduledPepeTask"), type_task);
                assert_eq!(task.scheduled_at, datetime);
            }

            #[test]
            fn remove_all_scheduled_tasks_test() {
                let queue: $q = $e;
                let datetime = (Utc::now() + Duration::seconds(7)).round_subsecs(0);

                let task1 = &ScheduledPepeTask {
                    number: 10,
                    datetime: datetime.to_string(),
                };

                let task2 = &ScheduledPepeTask {
                    number: 11,
                    datetime: datetime.to_string(),
                };

                queue.schedule_task(task1).unwrap();
                queue.schedule_task(task2).unwrap();

                let number = queue.remove_all_scheduled_tasks().unwrap();
                assert_eq!(2, number);
            }

            #[test]
            fn remove_all_tasks_test() {
                let task1 = PepeTask { number: 10 };
                let task2 = PepeTask { number: 11 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&task1).unwrap();
                let task2 = queue.insert_task(&task2).unwrap();

                let result = queue.remove_all_tasks().unwrap();

                assert_eq!(2, result);
                assert_eq!(None, queue.find_task_by_id(&task1.id));
                assert_eq!(None, queue.find_task_by_id(&task2.id));
            }

            #[test]
            fn remove_task() {
                let task1 = PepeTask { number: 10 };
                let task2 = PepeTask { number: 11 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&task1).unwrap();
                let task2 = queue.insert_task(&task2).unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_some());
                assert!(queue.find_task_by_id(&task2.id).is_some());

                queue.remove_task(&task1.id).unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_none());
                assert!(queue.find_task_by_id(&task2.id).is_some());
            }

            #[test]
            fn remove_task_of_type() {
                let task1 = PepeTask { number: 10 };
                let task2 = AyratTask { number: 10 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&task1).unwrap();
                let task2 = queue.insert_task(&task2).unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_some());
                assert!(queue.find_task_by_id(&task2.id).is_some());

                queue.remove_tasks_of_type("weirdo").unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_some());
                assert!(queue.find_task_by_id(&task2.id).is_none());
            }

            #[test]
            fn remove_task_by_metadata() {
                let m_task1 = PepeTask { number: 10 };
                let m_task2 = PepeTask { number: 11 };
                let m_task3 = AyratTask { number: 10 };

                let queue: $q = $e;

                let task1 = queue.insert_task(&m_task1).unwrap();
                let task2 = queue.insert_task(&m_task2).unwrap();
                let task3 = queue.insert_task(&m_task3).unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_some());
                assert!(queue.find_task_by_id(&task2.id).is_some());
                assert!(queue.find_task_by_id(&task3.id).is_some());

                queue.remove_task_by_metadata(&m_task1).unwrap();

                assert!(queue.find_task_by_id(&task1.id).is_none());
                assert!(queue.find_task_by_id(&task2.id).is_some());
                assert!(queue.find_task_by_id(&task3.id).is_some());
            }
        }
    };
}

pub(crate) use test_queue;

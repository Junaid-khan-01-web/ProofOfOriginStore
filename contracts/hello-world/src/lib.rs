#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub detail: String,
    pub done: bool,
    pub timestamp: u64,
}

#[contracttype]
pub enum TaskKey {
    Count,
    Record(u64),
    OwnerTasks(Address),
}

#[contract]
pub struct TodoAPIService;

#[contractimpl]
impl TodoAPIService {
    pub fn create_task(env: Env, owner: Address, title: String, detail: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&TaskKey::Count).unwrap_or(0);
        count = count.saturating_add(1);
        env.storage().instance().set(&TaskKey::Count, &count);

        let t = Task {
            id: count,
            owner: owner.clone(),
            title,
            detail,
            done: false,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().instance().set(&TaskKey::Record(count), &t);

        let mut vec: Vec<u64> = env.storage().instance().get(&TaskKey::OwnerTasks(owner.clone())).unwrap_or(Vec::new(&env));
        vec.push_back(count);
        env.storage().instance().set(&TaskKey::OwnerTasks(owner), &vec);

        count
    }

    pub fn mark_done(env: Env, caller: Address, task_id: u64) {
        let mut t: Task = env.storage().instance().get(&TaskKey::Record(task_id)).expect("task not found");
        assert!(t.owner == caller, "only owner can modify");
        t.done = true;
        env.storage().instance().set(&TaskKey::Record(task_id), &t);
    }

    pub fn view_task(env: Env, task_id: u64) -> Task {
        env.storage().instance().get(&TaskKey::Record(task_id)).expect("task not found")
    }
}

use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 plan 到 DB
    // ...
    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("Tyr".into());
        // 使用免费 feature
        customer.feature1();
        customer.feature2();
        // 用着用着觉得产品不错愿意付费
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature1();
        // 付费用户解锁了新技能
        customer.advance_feature();
    }
}

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct HelloWorldContract {
}
#[near_bindgen]
impl HelloWorldContract {

    pub fn hello_world(name: String) -> String {
        return format!("Hello {}!", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_hello() {
        assert_eq!("Hello Chuck!", HelloWorldContract::hello_world("Chuck".to_string()))
    }
}
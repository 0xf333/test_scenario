#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod call_one {

    #[ink(storage)]
    pub struct CallOne {
        value: u8,
    }

    impl CallOne {
       
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: 1,
            }
        }

        #[ink(message)]
        pub fn get_one(&self) -> u8 {
            self.value
        }
    }
}

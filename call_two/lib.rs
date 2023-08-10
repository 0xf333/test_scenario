#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod call_two {

    #[ink(storage)]
    pub struct CallTwo {
        value: u8,
    }

    impl CallTwo {
       
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: 2,
            }
        }

        #[ink(message)]
        pub fn get_two(&self) -> u8 {
            self.value
        }
    }
}

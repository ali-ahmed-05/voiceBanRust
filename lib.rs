#![cfg_attr(not(feature = "std"), no_std)]



use ink_storage::collections::{Vec, HashMap, Stash, Bitvec };
use ink_lang as ink;



#[ink::contract]
mod login {
    use ink_prelude::string::String;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Login {

        // pub struct UserDetail {
        //     email:String,
        //     first_name:String,
        //     last_name:String,
        //     user_name:String,
        // }
        /// Stores a single `bool` value on the storage.
        value: bool,
      
      //  first: ink_storage::Vec<u64>,
        // userdetail: UserDetail, 
       

        //  UserDetail :[
        //     email,
        //     first_name,
        //     last_name,
        //     user_name,
        //  ],




        //mappings --

        is_user: ink_storage::collections::HashMap<String, bool>,
     
       email_password: ink_storage::collections::HashMap<String, String>,
         username_exist: ink_storage::collections::HashMap<String, bool>,
       _logged: ink_storage::collections::HashMap<(String,String),bool>,
     //   user_detail: ink_storage::collections::HashMap<AccountId,userdetail>,
         forget: ink_storage::collections::HashMap<String,bool>,

    }
    // mapping (bytes32=>bytes32) emaiToPassword;
    // mapping (bytes32=>bool) private _userNameExist;
    // mapping (bytes32=>mapping(bytes32=>bool)) _logged;
    // mapping (bytes32=>UserDetail) private _UserDetail;
    // mapping (bytes32=>bool) private _isUser;
    // mapping (bytes32=>forgetPassword) private _forgetDetails;
    // mapping (bytes32=>bool) private _forget;

    

    // pub struct forgetPassword {
    //     number:u64,
    //     start_time:u64,
    // }
    //let mut s = String::with_capacity(25);
    const MAX_WAIT:u64 = 100;
    impl Login {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        // const MAX_WAIT:u64 = 100;

        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
        //    let isUser:_isUser;
           Self{
                value: init_value,
       
                is_user: Default::default(),
                username_exist :Default::default(),
                _logged:Default::default(),
                email_password:Default::default(),
                forget:Default::default(),
             // email_password: Default::default(),
            //     username_exist: Default::default(),
            //     forget: Default::default(),
            // }
        }}

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
       
        
        #[ink(message)]
        pub fn signup(&mut self, _email : String, _pass : String) -> bool {
           // let caller = self.env().caller();
           #[derive(item:item)]
          let x = _email; 
           
            self.is_user.insert(_email, true);
          //  borrow(_email);
            
            //    self.email_password.insert(_email, _pass);
            
           // self.email_password.insert(,_pass);
            return true;
        } 
         

       #[ink(message)]
        pub fn is_user(&self, _email : String) -> bool{
            let caller = self.env().caller();
            let is = self.is_user.get(&_email).unwrap_or(&false);
            *is       
        }
    }
    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let login = Login::default();
            assert_eq!(login.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut login = Login::new(false);
            assert_eq!(login.get(), false);
            login.flip();
            assert_eq!(login.get(), true);
        }
    }
}
// function signup(string memory email ,
//     string memory password,
//     string memory _firstName,
//     string memory _lastName,
//     string memory _userName) public returns(bool){
//    bytes32 _email = stringToBytes32(_toLower(email));
//    bytes32 _password = toHash(_toLower(password));
//    require(_logged[_email][_password]==false , "user has already signup");
//    _logged[_email][_password]=true;
//    _isUser[_email] = true;
//    emaiToPassword[_email] = _password;
//     setUserDetail(email,_firstName,_lastName,_userName);
//    return true;
// }


// function isUser(string memory email)public view returns(bool){
//     bytes32 _email = stringToBytes32(_toLower(email));
//    return _isUser[_email];
// } 
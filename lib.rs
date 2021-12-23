#![cfg_attr(not(feature = "std"), no_std)]



use ink_storage::collections::{Vec, HashMap, Stash, Bitvec };
use ink_lang as ink;
use rand::prelude::*;


 

// #[derive(Clone, Debug)]
// pub struct ThreadRng {
//     // Rc is explictly !Send and !Sync
//     rng: Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>,
// }



#[ink::contract]
mod login {
    use ink_prelude::string::String;
   
   // use rand::rngs::ThreadRng;
    // use rand::thread_rng;
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
        
      
        // first: ink_storage::Vec<u64>,
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
         user_detail:ink_storage::collections::HashMap<String,(String,String,String)>,

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
                user_detail : Default::default(),
                
              
            // email_password: Default::default(),
            //     username_exist: Default::default(),
            //     forget: Default::default(),
            // }
        }}
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
     

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
        pub fn signup(&mut self, 
            _email : String, 
            _pass : String , 
            first_name : String ,
            last_name : String ,
            user_name : String) -> bool {
        // let caller = self.env().caller();
        //    #[derive(item:item)]
        //   let x = _email; 
        let mut email:String = String::from(_email);
        let mut pass:String = String::from(_pass);
        let copy2_email = email.clone();
        let copy2_pass = pass.clone();
        let checklog = self._logged.contains_key(&(copy2_email,copy2_pass));

        if checklog == true{
            return false;
        }

        let copy_email = email.clone();
        let copy1_email = email.clone();
        let copy1_pass = pass.clone();
        let copy3_email = email.clone();
       
         self.is_user.insert(email, true);   
         self.email_password.insert(copy_email, pass);
         self._logged.insert((copy1_email,copy1_pass), true);
         self.user_detail.insert(copy3_email, (first_name,last_name,user_name));
      //   let mut rng = thread_rng();

        
            return true;
        }
         

       #[ink(message)]
        pub fn is_user(&self, _email : String) -> bool{
            let is = self.is_user.contains_key(&_email);
            return is;       
        }

        #[ink(message)]
        pub fn login(&mut self, _email : String, _pass : String) -> bool{

        let mut email:String = String::from(_email);
        let mut pass:String = String::from(_pass);
        let copy2_email = email.clone();
        let copy2_pass = pass.clone();
        let checklog = self._logged.contains_key(&(copy2_email,copy2_pass));
        let checkuser = self.is_user.contains_key(&email);
     //   let mut rng = thread_rng();
        if checkuser {

            if !checklog {
            return false;
            }
        }
        return true;
        }
        
     //   #[ink(message)]
        // pub fn getUserDetails(&mut self, _email : String) -> (String,String,String,String){
        //     let mut email:String = String::from(_email);
        //     let copy_email = email.clone();
        //     let detail = self.is_user.get(&email).unwrap_or(&false);
        //     let first_name:String= String::from(detail::<{ (String as From<T>)<<&String }>);
        //   //  let (first_name,last_name,user_name) = detail;
            
        //     return (copy_email,first_name,last_name,user_name);

        // }
        
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
        fn not_signed() {
            let mut login = Login::new(false);
            let mut email:String = String::from("acb@gmail.com");
            assert_eq!(login.is_user(email), false);
        }
        #[ink::test]
        fn it_signs() {
            let mut login = Login::new(false);
            let mut email:String = String::from("acb@gmail.com");
            assert_eq!(login.is_user(email), false);
            login.signup( 
                "acb@gmail.com".to_string(),
                "a".to_string() , 
                "first_name".to_string(),
                "last_name".to_string(),
                "user_name".to_string());
                assert_eq!(login.is_user("acb@gmail.com".to_string()), true);
        }
        #[ink::test]
        fn it_Logins() {
            let mut login = Login::new(false);
            let mut email:String = String::from("acb@gmail.com");
            assert_eq!(login.is_user(email), false);
            login.signup( 
                "acb@gmail.com".to_string(),
                "a".to_string() , 
                "first_name".to_string(),
                "last_name".to_string(),
                "user_name".to_string());
                assert_eq!(login.is_user("acb@gmail.com".to_string()), true);
           let mut log =  login.login("acb@gmail.com".to_string(),"a".to_string());
           assert_eq!(log, true);
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

// function login(string memory email ,string memory password) public view returns(bool){
//     bytes32 _email = stringToBytes32(_toLower(email));
//     require(_isUser[_email]==true,"This email is not registered");
//     bytes32 _password = toHash(_toLower(password));
//     require(_logged[_email][_password]==true , "please provide correct password");
//     return true;
// }

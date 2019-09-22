use support::{decl_storage, decl_module, StorageMap, dispatch::Result};
use system::ensure_signed;

use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Sneaker<Hash, Balance> {
    id: Hash,
    price: Balance,
}


//pub trait Trait: system::Trait {}
pub trait Trait: balances::Trait {}


decl_storage! {
    trait Store for Module<T: Trait> as SneakerStorage {
        // Declare storage and getter functions here
        // Value: u64;
        Value: map T::AccountId => u64;
        OwnedSneaker get(sneaker_of_owner): map T::AccountId => Sneaker<T::Hash, T::Balance>;

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
         fn set_value(origin, value: u64) -> Result {

            let sender = ensure_signed(origin)?;

            //<Value<T>>::put(value);
            <Value<T>>::insert(sender, value);

            Ok(())
        }
        fn create_sneaker(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let new_sneaker = Sneaker {
                id: <T as system::Trait>::Hashing::hash_of(&0),
                price: <T::Balance as As<u64>>::sa(0),
            };

            <OwnedSneaker<T>>::insert(&sender, new_sneaker);

            Ok(())
        }
    }
}

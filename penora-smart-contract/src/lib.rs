#![cfg_attr(not(feature = "std"), no_std)]

pub use self::my_article_contract::MyArticleContract;

#[ink::contract]
mod my_article_contract {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MyArticleContract {
        articles: Mapping<AccountId, Vec<Article>>,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Article {
        content: String,
    }

    impl MyArticleContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                articles: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn post_article(&mut self, content: String) {
            let caller = self.env().caller();
            let article = Article { content };
            let mut user_articles = self.articles.get(&caller).unwrap_or_default();
            user_articles.push(article);
            self.articles.insert(caller, &user_articles);
        }

        #[ink(message)]
        pub fn get_articles(&self, account: AccountId) -> Vec<Article> {
            self.articles.get(&account).unwrap_or_default()
        }
    }
}

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue,Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OwnershipContract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

const DATA_IMAGE_PNG_CALIMERO_ICON: &str = "%3Csvg width=\'32\' height=\'32\' viewBox=\'0 0 32 32\' fill=\'none\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cpath fill-rule=\'evenodd\' clip-rule=\'evenodd\' d=\'M3.04306 4.60988C10.5403 0.251034 19.5615 1.39743 23.2946 2.55865C23.9906 2.77515 24.3813 3.52123 24.1672 4.22506C23.9531 4.92889 23.2153 5.32395 22.5193 5.10745C19.1612 4.06289 10.977 3.07311 4.35742 6.9217C4.21177 7.00638 4.12986 7.16143 4.13754 7.32299C4.389 12.6173 7.11023 22.7205 16.4855 28.0016C17.1217 28.3599 17.3501 29.1719 16.9958 29.8152C16.6414 30.4585 15.8384 30.6895 15.2023 30.3312C4.72293 24.4282 1.77864 13.2426 1.50355 7.45093C1.44804 6.28207 2.04683 5.18908 3.04306 4.60988Z\' fill=\'%2334D399\' /%3E%3Cpath fill-rule=\'evenodd\' clip-rule=\'evenodd\' d=\'M28.0788 4.50163C28.806 4.46299 29.4265 5.0278 29.4647 5.76316C29.9664 15.4186 25.0088 22.5 21.8134 25.4954C21.0122 26.2465 19.8155 26.5395 18.7207 26.0634C16.4491 25.0758 14.2504 22.998 12.4648 20.9258C10.6527 18.8228 9.13973 16.5752 8.25761 15.0662C7.88705 14.4323 8.09483 13.6146 8.72169 13.2399C9.34855 12.8652 10.1571 13.0753 10.5277 13.7092C11.3383 15.0959 12.7591 17.2082 14.4529 19.1739C16.1733 21.1704 18.0526 22.8702 19.7623 23.6136C19.8093 23.6341 19.9103 23.6432 20.0207 23.5398C22.8185 20.917 27.2808 14.5543 26.8313 5.90309C26.7931 5.16772 27.3516 4.54027 28.0788 4.50163Z\' fill=\'%2334D399\' /%3E%3Cpath fill-rule=\'evenodd\' clip-rule=\'evenodd\' d=\'M13.9093 9.619C11.5577 9.86424 9.59494 10.4359 8.57328 10.859C7.89942 11.1382 7.1294 10.812 6.85339 10.1306C6.57739 9.44913 6.89991 8.67046 7.57377 8.39135C8.84479 7.86491 11.0549 7.23587 13.6388 6.96641C16.2246 6.69675 19.2745 6.77764 22.1787 7.75272C23.4253 8.17128 24.3034 9.38164 24.2123 10.773C24.0235 13.6574 22.7145 16.6067 20.1067 19.9864C19.6587 20.567 18.8302 20.6704 18.2561 20.2174C17.682 19.7644 17.5797 18.9265 18.0277 18.346C20.4557 15.1993 21.44 12.7516 21.5811 10.5968C21.5886 10.4824 21.5174 10.3405 21.3477 10.2835C18.9078 9.46434 16.2592 9.37396 13.9093 9.619Z\' fill=\'%2334D399\' /%3E%3Cpath fill-rule=\'evenodd\' clip-rule=\'evenodd\' d=\'M2.79175 4.17763C10.4566 -0.278677 19.6383 0.897672 23.4431 2.08121C24.4057 2.38065 24.9384 3.40786 24.6455 4.37057C24.352 5.33555 23.3358 5.88508 22.3708 5.58489C20.7463 5.07957 17.9163 4.57803 14.6771 4.6794C11.4496 4.7804 7.84892 5.47884 4.63878 7.33652C4.8969 12.5493 7.58622 22.4147 16.7309 27.5659C17.6085 28.0602 17.9191 29.1753 17.4337 30.0564C16.9466 30.9408 15.8377 31.263 14.9569 30.7668C4.26833 24.746 1.28351 13.357 1.00412 7.47465C0.939766 6.11979 1.63388 4.8508 2.79175 4.17763ZM3.7255 4.79858L4.1349 5.51867C4.27138 5.75873 4.18742 6.06397 3.94736 6.20045C3.7073 6.33693 3.40206 6.25297 3.26558 6.01291L2.88255 5.33921C2.29841 5.8549 1.96459 6.61877 2.00299 7.42721C2.27377 13.1283 5.17752 24.1104 15.4477 29.8955C15.8392 30.1161 16.3362 29.9762 16.5578 29.574C16.7812 29.1685 16.6349 28.6596 16.2402 28.4372C6.65607 23.0385 3.8939 12.7324 3.6381 7.34671C3.62163 6.99982 3.79824 6.66844 4.10611 6.48944C7.50802 4.5116 11.2969 3.78469 14.6458 3.67989C17.9931 3.57514 20.9342 4.09077 22.6678 4.63002C23.0948 4.76283 23.5542 4.52224 23.6888 4.07955C23.8242 3.6346 23.5755 3.16965 23.1461 3.03608C19.5563 1.91943 10.9681 0.82436 3.7255 4.79858ZM28.0523 4.00234C29.0607 3.94876 29.9117 4.73111 29.964 5.73722C30.4754 15.5787 25.4259 22.7943 22.1554 25.8602C21.2244 26.7329 19.8182 27.0858 18.5213 26.522C16.1429 25.4879 13.8837 23.3385 12.086 21.2522C10.2517 19.1234 8.72144 16.8504 7.82595 15.3185C7.31827 14.45 7.60076 13.3274 8.46515 12.8107C9.33288 12.292 10.4498 12.5851 10.9593 13.4569C11.7566 14.8207 13.1601 16.9076 14.8317 18.8476C16.4775 20.7575 18.2257 22.3384 19.7846 23.0747C22.5127 20.4624 26.7662 14.2855 26.332 5.92903C26.2797 4.92336 27.0444 4.05589 28.0523 4.00234ZM28.9654 5.78911C28.9412 5.32449 28.5513 4.97723 28.1053 5.00093C27.6589 5.02465 27.3065 5.41208 27.3306 5.87714C27.7898 14.7142 23.2359 21.2111 20.3626 23.9046C20.1105 24.1409 19.7977 24.1742 19.5629 24.0722C17.7462 23.2823 15.8064 21.5107 14.0742 19.5003C12.3581 17.5088 10.92 15.3711 10.096 13.9615C9.86448 13.5654 9.36422 13.4383 8.97824 13.6691C8.5889 13.9018 8.45584 14.4145 8.68927 14.8138C9.55802 16.3 11.0537 18.5222 12.8436 20.5994C14.617 22.6576 16.7553 24.6637 18.9201 25.6049C19.8129 25.9931 20.8 25.7601 21.4715 25.1307C24.5916 22.2057 29.4574 15.2584 28.9654 5.78911ZM13.587 6.4691C16.2181 6.19473 19.3448 6.27382 22.3378 7.27872C23.7899 7.76626 24.8179 9.17784 24.7113 10.8057C24.5135 13.8255 23.1442 16.8682 20.5026 20.2918C19.8854 21.0917 18.74 21.2361 17.9464 20.6099C17.1562 19.9864 17.0172 18.837 17.6318 18.0405C19.9651 15.0166 20.9021 12.7036 21.0705 10.7185C18.7516 9.96517 16.2272 9.88 13.9612 10.1163C11.6541 10.3569 9.73867 10.9175 8.76461 11.321C7.83071 11.7078 6.76878 11.2535 6.38996 10.3183C6.01226 9.38576 6.45144 8.31502 7.38244 7.92941C8.70105 7.38325 10.9584 6.74321 13.587 6.4691ZM22.0195 8.22672C19.2042 7.28146 16.2312 7.19878 13.6907 7.46371C11.1513 7.72853 8.98852 8.34657 7.7651 8.8533C7.34838 9.0259 7.14251 9.51251 7.31682 9.94286C7.49001 10.3705 7.96813 10.5685 8.38194 10.3971C9.4512 9.95422 11.4613 9.37158 13.8575 9.1217C16.2526 8.87193 18.9781 8.96052 21.5069 9.80954C21.8926 9.93903 22.1023 10.2895 22.08 10.6295C21.9301 12.9196 20.8855 15.4607 18.4235 18.6514C18.1422 19.016 18.2078 19.5424 18.5658 19.8249C18.9204 20.1047 19.4321 20.0422 19.7108 19.681C22.2847 16.3452 23.5334 13.4894 23.7134 10.7404C23.789 9.58544 23.0608 8.57631 22.0195 8.22672Z\' fill=\'%2334D399\' /%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl OwnershipContract {
    pub fn hello() {
        println!("Hello, from contract!");
    }

    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for test purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_PNG_CALIMERO_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    ///Initialize contract
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    /// Mint a new token with ID='token_id' belonging to 'receiver_id'
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens
            .internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for OwnershipContract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

near_contract_standards::impl_non_fungible_token_core!(OwnershipContract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(OwnershipContract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(OwnershipContract, tokens);

#[cfg(all(test, not(target_arch = "wasm32")))]
mod ownership_tests;

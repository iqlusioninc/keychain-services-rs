//! Query the keychain, looking for particular items

use crate::{attr::*, dictionary::DictionaryBuilder, ffi::*};
use core_foundation::{
    base::{CFType, TCFType},
    number::CFNumber,
    string::CFString,
};

/// Limit the number of matched items to one or an unlimited number.
///
/// Wrapper for the `kSecMatchLimit` attribute key. See:
/// <https://developer.apple.com/documentation/security/ksecmatchlimit>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MatchLimit {
    /// Match exactly one item.
    ///
    /// Wrapper for the `kSecMatchLimitOne` attribute value. See:
    /// <https://developer.apple.com/documentation/security/ksecmatchlimitone>
    One,

    /// Match the specified number of items.
    ///
    /// Equivalent to passing a `CFNumberRef` as the value for
    /// `kSecMatchLimit`. See:
    /// <https://developer.apple.com/documentation/security/ksecmatchlimit>
    Number(usize),

    /// Match an unlimited number of items.
    ///
    /// Wrapper for the `kSecMatchLimitAll` attribute value. See:
    /// <https://developer.apple.com/documentation/security/ksecmatchlimitall>
    All,
}

impl MatchLimit {
    /// Get `CFType` containing the `kSecMatchLimit` dictionary value for
    /// this particular `SecMatchLimit`.
    pub fn as_CFType(self) -> CFType {
        match self {
            MatchLimit::One => {
                unsafe { CFString::wrap_under_get_rule(kSecMatchLimitOne) }.as_CFType()
            }
            MatchLimit::Number(n) => CFNumber::from(n as i64).as_CFType(),
            MatchLimit::All => {
                unsafe { CFString::wrap_under_get_rule(kSecMatchLimitAll) }.as_CFType()
            }
        }
    }
}

/// Query builder for locating particular keychain items.
///
/// For more information, see "Search Attribute Keys and Values":
/// <https://developer.apple.com/documentation/security/keychain_services/keychain_items/search_attribute_keys_and_values>
#[derive(Default, Debug)]
pub struct Query(DictionaryBuilder);

impl Query {
    /// Create a new keychain item query builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Query for keychain items with the provided `SecAttrApplicationLabel`
    /// (not to be confused with a `SecAttrLabel`), i.e. the hash/fingerprint
    /// of a public key in the keychain.
    ///
    /// Both the private and public key in a keypair have a
    /// `SecAttrApplicationLabel` set to the public key's fingerprint.
    ///
    /// Wrapper for the `kSecAttrApplicationLabel` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrlabel>
    pub fn application_label<L: Into<AttrApplicationLabel>>(mut self, label: L) -> Self {
        self.0.add_attr(&label.into());
        self
    }

    /// Query for keychain items with the provided `SecAttrApplicationTag`.
    ///
    /// Wrapper for the `kSecAttrApplicationTag` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrapplicationtag>
    pub fn application_tag<T>(mut self, tag: T) -> Self
    where
        T: Into<AttrApplicationTag>,
    {
        self.0.add_attr(&tag.into());
        self
    }

    /// Query for keys with the given `SecAttrKeyClass`.
    ///
    /// Wrapper for the `kSecAttrKeyClass` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrkeyclass>
    pub fn key_class(mut self, key_class: AttrKeyClass) -> Self {
        self.0.add_attr(&key_class);
        self
    }

    /// Query for keys with the given `SecAttrKeyType`.
    ///
    /// Wrapper for the `kSecAttrKeyType` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrkeytype>
    pub fn key_type(mut self, key_type: AttrKeyType) -> Self {
        self.0.add_attr(&key_type);
        self
    }

    /// Query for a particular (human-meaningful) label on keys
    ///
    /// Wrapper for the `kSecAttrLabel` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrlabel>
    pub fn label<L: Into<AttrLabel>>(mut self, label: L) -> Self {
        self.0.add_attr(&label.into());
        self
    }

    /// Query for keys which are or not permanent members of the default keychain.
    ///
    /// Wrapper for the `kSecAttrIsPermanent` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrispermanent>
    pub fn permanent(mut self, value: bool) -> Self {
        self.0.add_boolean(AttrKind::Permanent, value);
        self
    }

    /// Query for keys which are or are not synchronizable.
    ///
    /// Wrapper for the `kSecAttrSynchronizable` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrsynchronizable>
    pub fn synchronizable(mut self, value: bool) -> Self {
        self.0.add_boolean(AttrKind::Synchronizable, value);
        self
    }

    /// Query for keys which are or are not sensitive.
    ///
    /// Wrapper for the `kSecAttrIsSensitive` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrissensitive>
    pub fn sensitive(mut self, value: bool) -> Self {
        self.0.add_boolean(AttrKind::Sensitive, value);
        self
    }

    /// Query for keys stored in an external token i.e. the
    /// Secure Enclave Processor (SEP).
    ///
    /// Wrapper for the `kSecAttrTokenID` attribute key. See:
    /// <https://developer.apple.com/documentation/security/ksecattrtokenid>
    pub fn token_id(mut self, value: AttrTokenId) -> Self {
        self.0.add_attr(&value);
        self
    }

    /// Prompt the user with the given custom message when using keys returned
    /// from this query.
    ///
    /// Wrapper for the `kSecUseOperationPrompt`. See:
    /// <https://developer.apple.com/documentation/security/ksecuseoperationprompt>
    pub fn use_operation_prompt(mut self, value: &str) -> Self {
        self.0.add_string(unsafe { kSecUseOperationPrompt }, value);
        self
    }
}

impl From<Query> for DictionaryBuilder {
    fn from(params: Query) -> DictionaryBuilder {
        params.0
    }
}

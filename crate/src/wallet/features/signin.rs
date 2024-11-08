use std::hash::Hash;

use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    Reflection, SemverVersion, SignInOutput, SigninInput, WalletAccount, WalletError, WalletResult,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SignIn {
    version: SemverVersion,
    callback: Function,
}

impl SignIn {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        log::info!("SIGNING: {:?}", value);

        let signin_fn = Reflection::new(value)?
            .mutate_inner("signIn")?
            .as_owned_function()?;

        Ok(Self {
            version,
            callback: signin_fn,
        })
    }

    pub(crate) async fn call_signin(
        &self,
        signin_input: &SigninInput,
        public_key: [u8; 32],
    ) -> WalletResult<SignInOutput> {
        let outcome = self
            .callback
            .call1(&JsValue::null(), &signin_input.get_object()?.into())?;

        let outcome = js_sys::Promise::resolve(&outcome);

        let value = wasm_bindgen_futures::JsFuture::from(outcome).await?;
        let output_array = Reflection::new(value)?.get_array()?;

        let first_index = Reflection::new(output_array.get(0))?;
        let account = first_index.reflect_inner("account")?;
        let account = WalletAccount::parse(&Reflection::new(account)?)?;

        let message_value = first_index.reflect_inner("signedMessage")?;
        let message_bytes = message_value.dyn_into::<js_sys::Uint8Array>()?.to_vec();
        let message =
            core::str::from_utf8(&message_bytes).map_err(|error| WalletError::JsError {
                name: "Invalid UTF-8 Message".to_string(),
                message: error.to_string(),
                stack: "INTERNAL_ERROR".to_string(),
            })?;

        signin_input.check_eq(message)?;

        let signature_value = first_index.reflect_inner("signature")?;
        let signature_bytes: [u8; 64] = signature_value
            .dyn_into::<js_sys::Uint8Array>()?
            .to_vec()
            .try_into()
            .or(Err(WalletError::InvalidEd25519SignatureBytes))?;

        SigninInput::verify(public_key, &message_bytes, signature_bytes)?;

        Ok(SignInOutput {
            account,
            message: message.to_string(),
            signature: signature_bytes,
            public_key,
        })
    }
}

impl PartialOrd for SignIn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for SignIn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl Hash for SignIn {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

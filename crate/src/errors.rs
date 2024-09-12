use thiserror::Error;
use wasm_bindgen_futures::wasm_bindgen::JsValue;

/// Wraps a Rust [Result] type with the [WalletAdapterError]
/// as the `Err()` type in `core::result::Result`
pub type WalletAdapterResult<'a, T> = Result<T, WalletAdapterError<'a>>;

/// All the errors in this crate are converted to this type
#[derive(Debug, PartialEq, Clone, Error)]
pub enum WalletAdapterError<'a> {
    /// The window for the browser was not detected
    #[error("The window for the browser was not detected")]
    MissingAccessToBrowserWindow,
    /// The `window.document` was not detected
    #[error("The `window.document` was not detected")]
    MissingAccessToBrowserDocument,
    /// The cluster is not supported
    #[error("The cluster is not supported")]
    UnsupportedCluster(&'a str),
    /// The commitment passed is not supported. Check [crate::Commitment] for supported commitments.
    #[error(
        "The commitment passed is not supported. Check `Commitment` for supported commitments."
    )]
    UnsupportedCommitment(&'a str),
    /// The Error could not be parsed as an object
    #[error("The Error could not be parsed as an object")]
    DomErrorIsNotAnObject,
    /// Unable to get the key in the object
    #[error("Unable to get the key in the object")]
    UnableToGetKey(JsValue),
    /// A type you are referencing is undefined
    #[error("A type you are referencing is undefined")]
    Undefined,
    /// A type is Null
    #[error("A type is Null")]
    Null,
    /// A JsFunction is required
    #[error(" A JsFunction is required")]
    ExpectedAFunction(&'a str),
    /// This error is not supported. Open a bug report if you think the error needs to be supported
    #[error("This error is not supported. Open a bug report if you think the error needs to be supported")]
    UnrecognizedError,
    /// Parsing the error message from JavaScript error object was not possible
    #[error("Parsing the error message from JavaScript error object was not possible")]
    UnableToParseErrorMessage,
    /// Parsing the error name from Javascript error object was not possible
    #[error("Parsing the error name from Javascript error object was not possible")]
    UnableToParseErrorName,
    //******** IS VALID FOR PHANTOM DON"T RELY ON
    // THIS DUE TO MIGRATION TO WALLET STANDARD
    // PhantomObjectNotFound,
    // // Code 4001 = The user rejected the request through Phantom.
    // UserRejectedRequest,
    // // Code 4900 - Phantom could not connect to the network.
    // Disconnected,
    // // Code 4100 - The requested method and/or account has not been authorized by the user.
    // Unauthorized,
    // // Code -32000 - Missing or invalid parameters.
    // InvalidInput,
    // // Code -32002 - This error occurs when a dapp attempts to submit a new transaction
    // // while Phantom's approval dialog is already open for a previous transaction.
    // // Only one approve window can be open at a time.
    // // Users should  approve or reject their transaction before initiating a new transaction.
    // RequestedResourceNotAvailable,
    // // Code -32003 - Phantom does not recognize a valid transaction.
    // TransactionRejected,
    // // Code -32601 - Phantom does not recognize the method.
    // MethodNotFound,
    // // Code -32603 - Something went wrong within Phantom.
    // InternalError,
    // // Unable to parse the public key from a successful connection to the wallet
    // UnableToFetchPublicKey,
    // Error is not recognized or supported
    //*************** */
}

// impl WalletAdapterError {
//     pub fn parse_error_code(value: &JsValue) -> Self {
//         let code: i16 = if let Some(code) = value.as_f64() {
//             code as i16
//         } else {
//             return WalletAdapterError::UnrecognizedError;
//         };

//         match code {
//             4001 => WalletAdapterError::UserRejectedRequest,
//             4900 => WalletAdapterError::Disconnected,
//             4100 => WalletAdapterError::Unauthorized,
//             -32000 => WalletAdapterError::InvalidInput,
//             -32002 => WalletAdapterError::RequestedResourceNotAvailable,
//             -32003 => WalletAdapterError::TransactionRejected,
//             -32601 => WalletAdapterError::MethodNotFound,
//             -32603 => WalletAdapterError::InternalError,
//             _ => WalletAdapterError::UnrecognizedError,
//         }
//     }
// }

// impl From<JsValue> for WalletAdapterError {
//     fn from(value: JsValue) -> Self {
//         if !value.is_object() {
//             return WalletAdapterError::DomErrorIsNotAnObject;
//         }

//         let check_error = |error_value: JsValue| {
//             if error_value.is_undefined() {
//                 return WalletAdapterError::Undefined;
//             }

//             if error_value.is_null() {
//                 return WalletAdapterError::Null;
//             }

//             WalletAdapterError::UnableToGetKey(error_value)
//         };

//         let message_value = match js_sys::Reflect::get(&value, &"message".into()) {
//             Ok(value) => value,
//             Err(value) => return check_error(value),
//         };

//         let message = message_value.as_string();

//         if let Some(inner_message) = message.as_ref() {
//             if inner_message.contains("User rejected the request") {
//                 return WalletAdapterError::UserRejectedRequest;
//             }
//         } else {
//             return WalletAdapterError::UnableToParseErrorMessage;
//         }

//         let name = match js_sys::Reflect::get(&value, &"name".into()) {
//             Ok(value) => value,
//             Err(value) => return check_error(value),
//         };

//         let name = if let Some(name) = name.as_string() {
//             name
//         } else {
//             return WalletAdapterError::UnableToParseErrorName;
//         };

//         WalletAdapterError::TypeError {
//             name,
//             message: message.unwrap(),
//         }
//     }
// }

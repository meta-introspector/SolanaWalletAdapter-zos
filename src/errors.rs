use wasm_bindgen::JsValue;

pub type WalletAdapterResult<T> = Result<T, WalletAdapterError>;

#[derive(Debug, PartialEq, Clone)]
pub enum WalletAdapterError {
    SolanaObjectNotFound,
    PhantomObjectNotFound,
    DomErrorIsNotAnObject,
    TypeError { name: String, message: String },
    UnableToGetKey(JsValue),
    Undefined,
    Null,
    PhantonSolanaConnectNotFunction,
    // Code 4001 = The user rejected the request through Phantom.
    UserRejectedRequest,
    // Code 4900 - Phantom could not connect to the network.
    Disconnected,
    // Code 4100 - The requested method and/or account has not been authorized by the user.
    Unauthorized,
    // Code -32000 - Missing or invalid parameters.
    InvalidInput,
    // Code -32002 - This error occurs when a dapp attempts to submit a new transaction
    // while Phantom's approval dialog is already open for a previous transaction.
    // Only one approve window can be open at a time.
    // Users should  approve or reject their transaction before initiating a new transaction.
    RequestedResourceNotAvailable,
    // Code -32003 - Phantom does not recognize a valid transaction.
    TransactionRejected,
    // Code -32601 - Phantom does not recognize the method.
    MethodNotFound,
    // Code -32603 - Something went wrong within Phantom.
    InternalError,
    // Unable to parse the public key from a successful connection to the wallet
    UnableToFetchPublicKey,
    // Error is not recognized or supported
    UnrecognizedError,
}

impl WalletAdapterError {
    pub fn parse_error_code(value: &JsValue) -> Self {
        let code: i16 = if let Some(code) = value.as_f64() {
            code as i16
        } else {
            return WalletAdapterError::UnrecognizedError;
        };

        match code {
            4001 => WalletAdapterError::UserRejectedRequest,
            4900 => WalletAdapterError::Disconnected,
            4100 => WalletAdapterError::Unauthorized,
            -32000 => WalletAdapterError::InvalidInput,
            -32002 => WalletAdapterError::RequestedResourceNotAvailable,
            -32003 => WalletAdapterError::TransactionRejected,
            -32601 => WalletAdapterError::MethodNotFound,
            -32603 => WalletAdapterError::InternalError,
            _ => WalletAdapterError::UnrecognizedError,
        }
    }
}

impl From<JsValue> for WalletAdapterError {
    fn from(value: JsValue) -> Self {
        if !value.is_object() {
            return WalletAdapterError::DomErrorIsNotAnObject;
        }

        let check_error = |error_value: JsValue| {
            if error_value.is_undefined() {
                return WalletAdapterError::Undefined;
            }

            if error_value.is_null() {
                return WalletAdapterError::Null;
            }

            WalletAdapterError::UnableToGetKey(error_value)
        };

        let message = match js_sys::Reflect::get(&value, &"message".into()) {
            Ok(value) => value,
            Err(value) => return check_error(value),
        };

        let name = match js_sys::Reflect::get(&value, &"name".into()) {
            Ok(value) => value,
            Err(value) => return check_error(value),
        };

        let message = message
            .as_string()
            .unwrap_or("Unable to convert error `message` into Rust `String`".into());
        let name = name
            .as_string()
            .unwrap_or("Unable to convert error `name` into Rust `String`".into());

        WalletAdapterError::TypeError { name, message }
    }
}

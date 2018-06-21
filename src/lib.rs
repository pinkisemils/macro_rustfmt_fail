#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[macro_export]
macro_rules! ffi_error {
    ($result:ident, $error:expr) => {
        #[repr(C)]
        #[derive(Debug)]
        pub struct $result {
            success: bool,
        }
        
        impl $result {
            pub fn into_result(self) -> Result<()> {
                match self.success {
                    true => Ok(()),
                    false => Err($error),
                }
            }
        }
        
        impl Into<Result<()>> for $result {
            fn into(self) -> Result<()> {
                self.into_result()
            }
        }
    };
}

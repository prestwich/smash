extern "C" {
    pub fn CRunCeloPrecompile(
        op: ::std::os::raw::c_char,
        i: *const ::std::os::raw::c_char,
        i_len: u32,
        o: *mut ::std::os::raw::c_char,
        o_len: *mut u32,
        err: *mut ::std::os::raw::c_char,
        char_len: *mut u32,
    ) -> u32;
}

const MAX_OUTPUT_LEN: usize = 4096;
const ERROR_DESCRIPTION_LEN: usize = 256;

pub fn run_precompile(address: u8, input: &[u8]) -> Result<Vec<u8>, String> {
    let raw_operation_value: std::os::raw::c_char = unsafe { std::mem::transmute(address) };

    let mut result = vec![0u8; MAX_OUTPUT_LEN];
    let mut error_description_buffer = vec![0u8; ERROR_DESCRIPTION_LEN];
    let raw_input = input.as_ptr() as *const std::os::raw::c_char;
    let output = result.as_mut_ptr() as *mut std::os::raw::c_char;
    let error_buffer = error_description_buffer.as_mut_ptr() as *mut std::os::raw::c_char;
    let input_len = input.len() as u32;
    let mut output_len = 0u32;
    let mut error_description_len = 0u32;

    let is_error = unsafe {
        CRunCeloPrecompile(
            raw_operation_value,
            raw_input,
            input_len,
            output,
            &mut output_len as *mut u32,
            error_buffer,
            &mut error_description_len as *mut u32,
        )
    };

    if is_error != 0 {
        if error_description_len == 0 {
            return Err("C++ api returned empty error description".to_string());
        }
        error_description_buffer.truncate(error_description_len as usize);
        let error_description_string = std::ffi::CString::new(error_description_buffer);
        match error_description_string {
            Ok(c_string) => {
                let string = c_string.into_string();
                match string {
                    Ok(string) => {
                        return Err(string);
                    }
                    Err(err) => {
                        return Err(format!(
                            "Error on conversion of string description, {:?}",
                            err
                        ));
                    }
                }
            }
            Err(n_error) => {
                return Err(format!(
                    "CString made from {} bytes containts empty bytes in a middle, {:?}",
                    error_description_len, n_error
                ));
            }
        }
    }

    result.truncate(output_len as usize);

    Ok(result)
}

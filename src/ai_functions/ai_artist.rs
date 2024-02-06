use warka_macros::func_to_str;

#[func_to_str]
pub fn print_frag_shader_code(_shader_component_description_and_template: &str) {
    /// INPUT: takes in a SHADER_DESCRIPTION and CODE_TEMPLATE for a fragment shader.
    /// IMPORTANT: The CODE_TEMPLATE is only an EXAMPLE, if the SHADER_DESCRIPTION requires it, make as many changes to fit it into the SHADER_DESCRIPTION
    /// IMPORTANT: The CODE_TEMPLATE doesn't have to followed exactly. Write functions that make sense for the users request if required.
    /// IMPORTANT: DO NOT ADD #version STATEMENT AT THE BEGINING.
    /// IMPORTANT: DO NOT ADD ``` WITH THE TYPE OF LANGUAGE.
    /// FUNCTION: Takes in an existing set of code marked as CODE_TEMPLATE and updates or rewrites it to wok for the purpose in the SHADER_DESCRIPTION
    /// OUTPUT: Print ONLY the SHADER CODE, NO COMMENTARY. This function will ONLY print the code.
    println!(OUTPUT)
}

#[func_to_str]
pub fn print_improved_frag_shader_code(_shader_component_description_and_template: &str) {
    /// INPUT: takes in a SHADER_DESCRIPTION and CODE_TEMPLATE for a fragment shader.
    /// FUNCTION: Performs the following tasks
    ///     1. Removes any bugs in the code
    ///     2. Makes sure everything requested in the spec of the fragment shader requirements were followed. If not, add the feature. No code should be implemented later. Everything should be printed now.
    ///     3. Only writes the code. JUST THE CODE THAT CAN BE COMPILED.
    /// OUTPUT: Print ONLY the SHADER CODE, NO COMMENTARY. This function will ONLY print the code.
    println!(OUTPUT)
}

#[func_to_str]
pub fn print_fixed_code(_broken_code_with_bugs: &str) {
    /// INPUT: Takes in Fragment shader code that doesn't compile as BROKEN_CODE and the ERROR_BUGS found
    /// FUNCTION: Removes bugs defined by ERROR_BUGS from the BROKEN_CODE
    /// OUTPUT: Print ONLY the new and IMPROVED SHADER CODE, NO COMMENTARY. This function will ONLY print the code.
    println!(OUTPUT)
}

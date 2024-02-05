use warka_macros::func_to_str;

#[func_to_str]
pub fn input_to_goal(_req: &str) {
    /// Input: User request
    /// Function: Converts the user request into a short summarized goal
    /// Output: Prints goal. All outputs starts with "build a gles fragment shader that ..."
    /// Example 1:
    ///     user_request = "I need a shader that takes in time and no textures to create fire that moves with time."
    ///     OUTPUT = "build a gles fragment shader that takes only time as a uniform and NO TEXTURE uniforms to create fire that moves with time."
    /// Example 2:
    ///     user_request = "Create something that will change color with a mosaic pattern with time."
    ///     OUTPUT = "build a gles fragment shader that takes time as a uniform to create a mosaic pattern which changes colors with time."
    println!(OUTPUT)
}

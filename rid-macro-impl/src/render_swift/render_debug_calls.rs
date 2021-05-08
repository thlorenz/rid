use crate::render_swift::render_swift_call;

pub fn render_swift_debug_calls(
    debug_method_name: &str,
    debug_pretty_method_name: &str,
    comment: &str,
) -> String {
    format!(
        r###"
{comment} Swift calls generated for exported debug methods to prevent removal.
{comment} 
{comment} ```swift
{comment} func dummyCalls_{debug_method_name}_{debug_pretty_method_name} {{
{comment}     {debug_call};
{comment}     {debug_pretty_call};
{comment} }}
{comment} ```
    "###,
        comment = comment,
        debug_method_name = debug_method_name,
        debug_pretty_method_name = debug_pretty_method_name,
        debug_call = render_swift_call(debug_method_name, &[], true),
        debug_pretty_call =
            render_swift_call(debug_pretty_method_name, &[], true),
    )
}

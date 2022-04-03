pub fn outer_attributes<'a>(
    attrs: &'a [syn::Attribute],
) -> impl Iterator<Item = &syn::Attribute> + 'a {
    attrs
        .iter()
        .filter(|attr| matches!(attr.style, syn::AttrStyle::Outer))
}

pub fn inner_attributes<'a>(
    attrs: &'a [syn::Attribute],
) -> impl Iterator<Item = &syn::Attribute> + 'a {
    attrs
        .iter()
        .filter(|attr| matches!(attr.style, syn::AttrStyle::Inner(_)))
}
pub fn capitalize(s: &mut str) -> String {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    (*s).to_string()
}

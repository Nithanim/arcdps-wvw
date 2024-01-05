pub fn drop_static_mut_option<T>(reference: &mut Option<T>) {
    let old_option = core::mem::replace(reference, None);
    if let Some(old) = old_option {
        drop(old);
    }
}

pub fn swap_static_mut_option<T>(reference: &mut Option<T>, new: Option<T>) {
    let old_option = core::mem::replace(reference, new);
    if let Some(old) = old_option {
        drop(old);
    }
}


pub mod linux {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("linkme_{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("__start_linkme_{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("__stop_linkme_{}", ident)
    }
}

pub mod freebsd {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("linkme_{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("__start_linkme_{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("__stop_linkme_{}", ident)
    }
}

pub mod macos {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("__DATA,__{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("\x01section$start$__DATA$__{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("\x01section$end$__DATA$__{}", ident)
    }
}

pub mod windows {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!(".linkme_{}$b", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!(".linkme_{}$a", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!(".linkme_{}$c", ident)
    }
}

pub mod illumos {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("set_linkme_{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("__start_set_linkme_{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("__stop_set_linkme_{}", ident)
    }
}

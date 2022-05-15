use indexmap::IndexMap;

pub trait Translator {
    fn fill(kvm: IndexMap<String, String>, template: &str) -> String;
}

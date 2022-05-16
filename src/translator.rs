use indexmap::IndexMap;

pub trait Translator {
    fn render(&self, kvm: IndexMap<String, String>, template: &str) -> String;
}

pub struct HandlebarsTrans {}

impl HandlebarsTrans {
    pub fn new() -> Self {
        Self {}
    }
}

impl Translator for HandlebarsTrans {
    fn render(&self, kvm: IndexMap<String, String>, template: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_one_word_with_handlebars_trans() {
        let mut kvm = IndexMap::new();
        kvm.insert("name".to_string(), "Taro".to_string());

        let translator = HandlebarsTrans::new();
        let result = translator.render(kvm, "hello {{name}}");

        assert_eq!(result, "hello Taro");
    }

    #[test]
    fn render_two_words_with_handlebars_trans() {
        let mut kvm = IndexMap::new();
        kvm.insert("name1".to_string(), "Taro".to_string());
        kvm.insert("name2".to_string(), "Yamada".to_string());

        let translator = HandlebarsTrans::new();
        let result = translator.render(kvm, "hello {{name1}}, My name is {{name2}}");

        assert_eq!(result, "hello Taro My name is Yamada");
    }

    #[test]
    fn render_numbers_with_handlebars_trans() {
        let mut kvm = IndexMap::new();
        kvm.insert("num".to_string(), "1".to_string());
        kvm.insert("depth".to_string(), "5".to_string());

        let translator = HandlebarsTrans::new();
        let result = translator.render(kvm, "$NUM={{num}} $DEPTH={{depth}}");

        assert_eq!(result, "$NUM=1 $DEPTH=5");
    }
}

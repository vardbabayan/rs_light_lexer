// # Задача
//
// Необходимо разработать утилиту подсчитывающую общее количество строк
// в тексте, количество строк кода, пустых строк, строк комментариев (`// Foo`)
// и количество символов кода в переданном в нее тексте, и писать результаты
// в консоль. Количество строк кода и количество символов кода не должны
// учитывать строки и символы не относящиеся к коду (к примеру, пустые строки
// или комментарии).
//
// Для тестового текста можете использовать константу `CODE_FRAGMENT`.
//
// Пример ожидаемого выполнения программы c `CODE_FRAGMENT` на входе:
//
// Lines in total: 7
// Lines containing code: 3
// Code symbols: 43
// Empty lines: 1
// Comment lines: 3

static CODE_FRAGMENT: &'static str = r#"// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}"#;

struct SourceUtility {
    code_source: String,
}

impl SourceUtility {
    pub fn new(code_source: String) -> Self {
        Self { code_source }
    }

    pub fn count_total_lines(&self) -> usize {
        return self.code_source.lines().count();
    }

    pub fn count_code_lines(&self) -> usize {
        return self.count_total_lines() - 
            self.count_empty_lines() - 
            self.count_comment_lines();
    }

    pub fn count_code_symbols(&self) -> usize {
        let mut counter = 0;
        
        for line in self.code_source.split('\n') {
            if line.len() != 0 && !line.trim().starts_with("//") {
                counter += line.chars().count();
            }
        }

        return counter;
    }

    pub fn count_empty_lines(&self) -> usize {
        return self.code_source.lines()
                                .filter(|&line| line.trim().is_empty())
                                .count();
    }

    pub fn count_comment_lines(&self) -> usize {
        return self.code_source.lines()
                                .filter(|&line| line.trim().starts_with("//"))
                                .count();
    }
}

fn main() {
    // let obj = SourceUtility::new(String::from(CODE_FRAGMENT));
    // println!("Lines in total: {}", obj.count_total_lines());
    // println!("Lines containing code: {}", obj.count_code_lines());
    // println!("Code symbols: {}", obj.count_code_symbols());
    // println!("Empty lines: {}", obj.count_empty_lines());
    // println!("Comment lines: {}", obj.count_comment_lines());
}

#[cfg(test)]
mod tests {
    use super::*;
// Lines in total: 7
// Lines containing code: 3
// Code symbols: 43
// Empty lines: 1
// Comment lines: 3
    #[test]
    fn test_base_fragment() {
        let obj = SourceUtility::new(String::from(CODE_FRAGMENT));
        
        assert_eq!(obj.count_total_lines(), 7);
        assert_eq!(obj.count_code_lines(), 3);
        assert_eq!(obj.count_code_symbols(), 41);
        assert_eq!(obj.count_empty_lines(), 1);
        assert_eq!(obj.count_comment_lines(), 3);
    }

    #[test]
    fn test_edge_fragment() {
        static test_fragment: &'static str = r#"//EDGE CASE
        //ONLY COMMENTS



        //WITH EMPTY LINES"#;

        let obj = SourceUtility::new(String::from(test_fragment));
        
        assert_eq!(obj.count_total_lines(), 6);
        assert_eq!(obj.count_code_lines(), 0);
        assert_eq!(obj.count_code_symbols(), 0);
        assert_eq!(obj.count_empty_lines(), 3);
        assert_eq!(obj.count_comment_lines(), 3);
    }

    #[test] 
    fn test_inner_fragment() {
        static test_fragment: &'static str = r#"
        struct SourceUtility {
            code_source: String,
        }
        
        impl SourceUtility {
            pub fn new(code_source: String) -> Self {
                Self { code_source }
            }
        
            pub fn count_total_lines(&self) -> usize {
                return self.code_source.lines().count();
            }
        
            pub fn count_code_lines(&self) -> usize {
                return (self.count_total_lines() - 
                        self.count_empty_lines() - 
                        self.count_comment_lines() 
                        );
            }
        
            pub fn count_code_symbols(&self) -> usize {
                let mut counter = 0;
                
                for line in self.code_source.split('\n') {
                    if line.len() != 0 && !line.trim().starts_with("//") {
                        counter += line.chars().count();
                    }
                }
        
                return counter;
            }
        
            pub fn count_empty_lines(&self) -> usize {
                return self.code_source.lines()
                                        .filter(|&line| line.trim().is_empty())
                                        .count();
            }
        
            pub fn count_comment_lines(&self) -> usize {
                return self.code_source.lines()
                                        .filter(|&line| line.trim().starts_with("//"))
                                        .count();
            }
        }"#;

        let obj = SourceUtility::new(String::from(test_fragment));
        println!("Lines in total: {}", obj.count_total_lines());
        println!("Lines containing code: {}", obj.count_code_lines());
        println!("Code symbols: {}", obj.count_code_symbols());
        println!("Empty lines: {}", obj.count_empty_lines());
        println!("Comment lines: {}", obj.count_comment_lines());

        assert_eq!(obj.count_total_lines(), 45);
        assert_eq!(obj.count_code_lines(), 36);
        assert_eq!(obj.count_code_symbols(), 1504);
        assert_eq!(obj.count_empty_lines(), 9);
        assert_eq!(obj.count_comment_lines(), 0);
    }
}
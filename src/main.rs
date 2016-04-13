#[derive(Debug)]
struct Buffer {
    data: Vec<Vec<char>>,
    style: Vec<Vec<Style>>
}

#[derive(Debug, Clone, PartialEq)]
enum Style { NoStyle, Keyword, Symbol }

impl Buffer {
    fn ensure(&mut self, line: usize, col: usize) {
        while line >= self.data.len() {
            self.data.push(vec![]);
            self.style.push(vec![]);
        }
        while col >= self.data[line].len() {
            self.data[line].push(' ');
            self.style[line].push(Style::NoStyle);
        }
    }
    pub fn put_char(&mut self, line: usize, col: usize, chr: char, style: Style) {
        self.ensure(line, col);
        self.data[line][col] = chr;
        self.style[line][col] = style;
    }
    pub fn put_string(&mut self, line: usize, col: usize, string: &str, style: Style) {
        self.ensure(line, col + string.len() - 1);

        for (idx, c) in string.chars().enumerate() {
            self.data[line][col + idx] = c;
            self.style[line][col + idx] = style.clone();
        }
    }
    pub fn insert_char(&mut self, line: usize, col: usize, chr: char, style: Style) {
        self.ensure(line, col);
        self.data[line].insert(col, chr);
        self.style[line].insert(col, style);
    }
    pub fn delete_char(&mut self, line: usize, col: usize) {
        if self.data.len() > line && self.data[line].len() > col {
            self.data[line].remove(col);
            self.style[line].remove(col);
        }
    }
    pub fn delete_line(&mut self, line: usize) {
        if self.data.len() > line {
            self.data.remove(line);
            self.style.remove(line);
        }
    }
    pub fn pair_at(&mut self, line: usize, col: usize) -> (char, Style) {
        self.ensure(line, col);

        (self.data[line][col], self.style[line][col].clone())
    }
    pub fn new() -> Buffer {
        Buffer { data: vec![], style: vec![] }
    }
}

fn main() {
    let mut buffer = Buffer::new();

    buffer.put_char(1, 1, '*', Style::Keyword);
    buffer.put_string(2, 2, "void", Style::Keyword);
    buffer.insert_char(1, 1, '@', Style::Symbol);

    println!("{:?}", buffer);
}

#[test]
fn test_put_and_insert() {
    let mut buffer = Buffer::new();

    buffer.put_char(1, 1, '*', Style::Keyword);
    buffer.put_string(2, 2, "void", Style::Keyword);
    buffer.insert_char(1, 1, '@', Style::Symbol);

    assert_eq!(buffer.pair_at(1, 1), ('@', Style::Symbol));
    assert_eq!(buffer.pair_at(1, 2), ('*', Style::Keyword));
    assert_eq!(buffer.pair_at(2, 3), ('o', Style::Keyword));
}

#[test]
fn test_delete() {
    let mut buffer = Buffer::new();

    buffer.put_char(2, 1, '*', Style::Symbol);
    buffer.put_char(2, 2, '-', Style::NoStyle);
    buffer.put_char(2, 3, '@', Style::Keyword);

    buffer.delete_char(2, 2);

    assert_eq!(buffer.pair_at(2, 1), ('*', Style::Symbol));
    assert_eq!(buffer.pair_at(2, 2), ('@', Style::Keyword));

    buffer.delete_line(1);

    assert_eq!(buffer.pair_at(1, 1), ('*', Style::Symbol));
    assert_eq!(buffer.pair_at(1, 2), ('@', Style::Keyword));
}

